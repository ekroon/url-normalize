use crate::options::{EmptyQueryValue, Options, RemoveQueryParameters};
use crate::url_parser::ParsedUrl;
use std::collections::HashSet;

/// Encoded reserved characters that need special handling during sort.
/// These are: : / ? # [ ] @ ! $ & ' ( ) * + , ; =
const ENCODED_RESERVED: &[&str] = &[
    "%3A", "%2F", "%3F", "%23", "%5B", "%5D", "%40", "%21", "%24", "%26", "%27", "%28", "%29",
    "%2A", "%2B", "%2C", "%3B", "%3D",
];

/// A parsed query parameter preserving its original format.
#[derive(Debug, Clone)]
struct QueryParam {
    key: String,
    value: Option<String>, // None means no `=` sign, Some("") means `key=`
}

/// Parse a raw query string into key-value pairs, preserving the distinction
/// between `key` (no equals) and `key=` (empty value).
fn parse_query_string(query: &str) -> Vec<QueryParam> {
    if query.is_empty() {
        return vec![];
    }
    query
        .split('&')
        .filter(|s| !s.is_empty())
        .map(|part| {
            if let Some(eq_idx) = part.find('=') {
                QueryParam {
                    key: part[..eq_idx].to_string(),
                    value: Some(part[eq_idx + 1..].to_string()),
                }
            } else {
                QueryParam {
                    key: part.to_string(),
                    value: None,
                }
            }
        })
        .collect()
}

/// Decode a query key for matching purposes (decode %XX and replace + with space).
fn decode_query_key(key: &str) -> String {
    let with_spaces = key.replace('+', "%20");
    percent_decode_lossy(&with_spaces)
}

/// Percent-decode with lossy UTF-8 conversion (invalid sequences become U+FFFD).
fn percent_decode_lossy(input: &str) -> String {
    let mut result = Vec::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hi = hex_val(bytes[i + 1]);
            let lo = hex_val(bytes[i + 2]);
            if bytes[i + 1].is_ascii_hexdigit() && bytes[i + 2].is_ascii_hexdigit() {
                result.push((hi << 4) | lo);
                i += 3;
                continue;
            }
        }
        result.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&result).into_owned()
}

/// Build a query string from parameters.
fn build_query_string(params: &[QueryParam]) -> Option<String> {
    if params.is_empty() {
        return None;
    }
    let parts: Vec<String> = params
        .iter()
        .map(|p| match &p.value {
            Some(v) => format!("{}={v}", p.key),
            None => p.key.clone(),
        })
        .collect();
    Some(parts.join("&"))
}

/// Determine which keys had no `=` in the original query string.
fn keys_without_equals(original_query: &str) -> HashSet<String> {
    let mut keys = HashSet::new();
    if original_query.is_empty() {
        return keys;
    }
    for part in original_query.split('&') {
        if !part.is_empty() && !part.contains('=') {
            keys.insert(decode_query_key(part));
        }
    }
    keys
}

/// Check if a string contains any encoded reserved character (case-insensitive).
fn has_encoded_reserved(s: &str) -> bool {
    let upper = s.to_uppercase();
    ENCODED_RESERVED.iter().any(|enc| upper.contains(enc))
}

/// Replace encoded reserved characters with temporary tokens for sorting.
/// Returns the modified string and the token prefix used.
/// Uses \x01 delimiters to prevent collision with user data after decoding.
fn replace_encoded_reserved(query: &str) -> (String, String) {
    let token_base = "\x01NR\x01";

    // Find a token index not already used in the query
    let mut token_index = 0u32;
    loop {
        let token = format!("{token_base}{token_index}\x01");
        if !query.contains(&token) {
            break;
        }
        token_index += 1;
    }

    let prefix = format!("{token_base}{token_index}\x01");
    let mut result = query.to_string();

    // Replace each encoded reserved char (case-insensitive) with token+hex
    for enc in ENCODED_RESERVED {
        let hex = &enc[1..]; // e.g., "3A"
        let upper = hex.to_uppercase();
        let lower = hex.to_lowercase();
        let token = format!("{prefix}{upper}");
        // Replace both upper and lowercase variants
        result = result.replace(&format!("%{upper}"), &token);
        if upper != lower {
            result = result.replace(&format!("%{lower}"), &token);
        }
    }

    (result, prefix)
}

/// Restore encoded reserved characters from temporary tokens.
fn restore_encoded_reserved(query: &str, prefix: &str) -> String {
    let mut result = query.to_string();
    for enc in ENCODED_RESERVED {
        let hex = &enc[1..].to_uppercase();
        let token = format!("{prefix}{hex}");
        result = result.replace(&token, &format!("%{hex}"));
    }
    result
}

/// Get a sortable key by decoding the temporary tokens back to their original chars.
fn get_sortable_key(key: &str, prefix: &str) -> String {
    let mut result = key.to_string();
    for enc in ENCODED_RESERVED {
        let hex = &enc[1..].to_uppercase();
        let token = format!("{prefix}{hex}");
        if let Some(ch) = u8::from_str_radix(hex, 16).ok().map(|b| b as char) {
            result = result.replace(&token, &ch.to_string());
        }
    }
    result
}

/// Process query parameters: remove/keep, sort, normalize empty values.
pub fn process_query(parsed: &mut ParsedUrl, options: &Options, original_query: Option<&str>) {
    let query_str = match &parsed.query {
        Some(q) => q.clone(),
        None => return,
    };

    if query_str.is_empty() {
        parsed.query = None;
        return;
    }

    // Check for encoded reserved characters that need token-replacement during sort
    let use_tokens = options.sort_query_parameters && has_encoded_reserved(&query_str);
    let (working_query, token_prefix) = if use_tokens {
        let (q, p) = replace_encoded_reserved(&query_str);
        (q, Some(p))
    } else {
        (query_str.clone(), None)
    };

    let mut params = parse_query_string(&working_query);

    // Encode query components (spaces, unicode, special chars) like URLSearchParams
    for p in &mut params {
        p.key = encode_query_component(&p.key, false);
        if let Some(ref v) = p.value {
            p.value = Some(encode_query_component(v, true));
        }
    }

    let has_keep = options.keep_query_parameters.is_some();

    // Remove unwanted query parameters
    if !has_keep {
        match &options.remove_query_parameters {
            RemoveQueryParameters::All => {
                parsed.query = None;
                return;
            }
            RemoveQueryParameters::List(filters) if !filters.is_empty() => {
                params.retain(|p| {
                    let key = if let Some(ref pfx) = token_prefix {
                        restore_encoded_reserved(&p.key, pfx)
                    } else {
                        p.key.clone()
                    };
                    let decoded_key = decode_query_key(&key);
                    !filters.iter().any(|f| f.matches(&decoded_key))
                });
            }
            _ => {}
        }
    }

    // Keep wanted query parameters
    if let Some(ref keep_filters) = options.keep_query_parameters {
        if keep_filters.is_empty() {
            parsed.query = None;
            return;
        }
        params.retain(|p| {
            let key = if let Some(ref pfx) = token_prefix {
                restore_encoded_reserved(&p.key, pfx)
            } else {
                p.key.clone()
            };
            let decoded_key = decode_query_key(&key);
            keep_filters.iter().any(|f| f.matches(&decoded_key))
        });
    }

    // Sort query parameters
    if options.sort_query_parameters {
        if let Some(ref pfx) = token_prefix {
            // Sort by decoded (original character) order
            params.sort_by(|a, b| {
                let ak = get_sortable_key(&a.key, pfx);
                let bk = get_sortable_key(&b.key, pfx);
                ak.cmp(&bk)
            });
        } else {
            params.sort_by(|a, b| a.key.cmp(&b.key));
        }
    }

    // Normalize empty query values
    let original = original_query.unwrap_or("");
    normalize_empty_values(
        &mut params,
        options.empty_query_value,
        original,
        &token_prefix,
    );

    // Normalize + to %20 in keys
    for p in &mut params {
        p.key = p.key.replace('+', "%20");
    }

    // Restore encoded reserved tokens
    if let Some(ref pfx) = token_prefix {
        for p in &mut params {
            p.key = restore_encoded_reserved(&p.key, pfx);
            if let Some(ref mut v) = p.value {
                *v = restore_encoded_reserved(v, pfx);
            }
        }
    }

    parsed.query = build_query_string(&params);
}

/// Encode a query component the way JS URLSearchParams would.
/// This percent-encodes non-ASCII, control chars, and special chars like `<`, `>`, `"`.
/// Spaces in values become `+`, in keys become `%20`.
/// Already-encoded ASCII sequences (%XX where XX decodes to ASCII) are preserved as-is.
/// Percent-encoded non-ASCII sequences are decoded, validated as UTF-8, and re-encoded.
/// Invalid UTF-8 sequences are replaced with U+FFFD.
fn encode_query_component(s: &str, is_value: bool) -> String {
    let mut result = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        // Handle percent-encoded sequences
        if b == b'%'
            && i + 2 < bytes.len()
            && bytes[i + 1].is_ascii_hexdigit()
            && bytes[i + 2].is_ascii_hexdigit()
        {
            let hi = hex_val(bytes[i + 1]);
            let lo = hex_val(bytes[i + 2]);
            let decoded_byte = (hi << 4) | lo;

            if decoded_byte < 0x80 {
                // ASCII: decode characters that URLSearchParams wouldn't encode
                // (unreserved form-urlencoded chars: alphanumeric, *, -, ., _)
                // Space (%20) in values becomes +
                if decoded_byte == 0x20 && is_value {
                    result.push('+');
                } else if is_urlsearchparams_safe(decoded_byte) {
                    result.push(decoded_byte as char);
                } else {
                    result.push('%');
                    result.push(to_hex_upper(hi));
                    result.push(to_hex_upper(lo));
                }
                i += 3;
            } else {
                // Non-ASCII: collect consecutive percent-encoded bytes for UTF-8 validation
                let mut pct_bytes = vec![decoded_byte];
                let mut j = i + 3;
                while j < bytes.len()
                    && bytes[j] == b'%'
                    && j + 2 < bytes.len()
                    && bytes[j + 1].is_ascii_hexdigit()
                    && bytes[j + 2].is_ascii_hexdigit()
                {
                    let h = hex_val(bytes[j + 1]);
                    let l = hex_val(bytes[j + 2]);
                    let db = (h << 4) | l;
                    if db < 0x80 {
                        break;
                    } // Stop at ASCII
                    pct_bytes.push(db);
                    j += 3;
                }
                // Validate as UTF-8 and re-encode (replaces invalid sequences with U+FFFD)
                let decoded = String::from_utf8_lossy(&pct_bytes);
                for ch in decoded.chars() {
                    let mut buf = [0u8; 4];
                    let encoded = ch.encode_utf8(&mut buf);
                    for &byte in encoded.as_bytes() {
                        result.push_str(&format!("%{:02X}", byte));
                    }
                }
                i = j;
            }
            continue;
        }
        // Space handling
        if b == b' ' {
            if is_value {
                result.push('+');
            } else {
                result.push_str("%20");
            }
            i += 1;
            continue;
        }
        // Characters that URLSearchParams encodes
        if b == b'"' {
            result.push_str("%22");
            i += 1;
            continue;
        }
        if b == b'<' {
            result.push_str("%3C");
            i += 1;
            continue;
        }
        if b == b'>' {
            result.push_str("%3E");
            i += 1;
            continue;
        }
        // Non-ASCII: UTF-8 percent-encode
        if b > 127 {
            let rest = &s[i..];
            if let Some(ch) = rest.chars().next() {
                let mut buf = [0u8; 4];
                let encoded = ch.encode_utf8(&mut buf);
                for &byte in encoded.as_bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                }
                i += ch.len_utf8();
                continue;
            }
        }
        result.push(b as char);
        i += 1;
    }
    result
}

fn hex_val(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => 0,
    }
}

fn to_hex_upper(nibble: u8) -> char {
    if nibble < 10 {
        (b'0' + nibble) as char
    } else {
        (b'A' + nibble - 10) as char
    }
}

/// Characters that URLSearchParams does NOT encode (the form-urlencoded safe set).
/// These are: alphanumeric, `*`, `-`, `.`, `_`
fn is_urlsearchparams_safe(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'*' || byte == b'-' || byte == b'.' || byte == b'_'
}

/// Normalize empty query parameter values based on the `EmptyQueryValue` setting.
fn normalize_empty_values(
    params: &mut [QueryParam],
    mode: EmptyQueryValue,
    original_query: &str,
    token_prefix: &Option<String>,
) {
    let no_equals_keys = match mode {
        EmptyQueryValue::Preserve => Some(keys_without_equals(original_query)),
        _ => None,
    };

    for p in params.iter_mut() {
        // Skip params with non-empty values
        if let Some(ref v) = p.value {
            if !v.is_empty() {
                continue;
            }
        }

        // Empty key with = sign: always keep as "="
        if p.key.is_empty() && p.value.is_some() {
            continue;
        }

        match mode {
            EmptyQueryValue::Always => {
                p.value = Some(String::new());
            }
            EmptyQueryValue::Never => {
                p.value = None;
            }
            EmptyQueryValue::Preserve => {
                if let Some(ref no_eq) = no_equals_keys {
                    // Restore tokens before matching against original keys
                    let key_for_match = if let Some(ref pfx) = token_prefix {
                        restore_encoded_reserved(&p.key, pfx)
                    } else {
                        p.key.clone()
                    };
                    let decoded = decode_query_key(&key_for_match);
                    if no_eq.contains(&decoded) {
                        p.value = None;
                    } else {
                        p.value = Some(String::new());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query_string() {
        let params = parse_query_string("a=1&b=2&c");
        assert_eq!(params.len(), 3);
        assert_eq!(params[0].key, "a");
        assert_eq!(params[0].value, Some("1".to_string()));
        assert_eq!(params[2].key, "c");
        assert_eq!(params[2].value, None);
    }

    #[test]
    fn test_build_query_string() {
        let params = vec![
            QueryParam {
                key: "a".to_string(),
                value: Some("1".to_string()),
            },
            QueryParam {
                key: "b".to_string(),
                value: None,
            },
        ];
        assert_eq!(build_query_string(&params), Some("a=1&b".to_string()));
    }

    #[test]
    fn test_empty_params() {
        assert_eq!(build_query_string(&[]), None);
    }

    #[test]
    fn test_has_encoded_reserved() {
        assert!(has_encoded_reserved("token=a%2Fb"));
        assert!(has_encoded_reserved("foo%3abar=1"));
        assert!(!has_encoded_reserved("foo=bar"));
    }

    #[test]
    fn test_replace_restore_roundtrip() {
        let input = "token=a%2Fb&foo%3Abar=1";
        let (replaced, prefix) = replace_encoded_reserved(input);
        assert!(!replaced.contains("%2F"));
        assert!(!replaced.contains("%3A"));
        let restored = restore_encoded_reserved(&replaced, &prefix);
        assert_eq!(restored, "token=a%2Fb&foo%3Abar=1");
    }
}
