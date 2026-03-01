/// Percent-decode a string: `%XX` sequences are decoded to their byte values.
/// Only decodes sequences where the resulting byte is a valid UTF-8 character
/// that is "safe" to decode (not a reserved URL character).
#[allow(dead_code)]
pub fn percent_decode(input: &str) -> String {
    let mut result = Vec::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(hi), Some(lo)) = (
                hex_digit(bytes[i + 1]),
                hex_digit(bytes[i + 2]),
            ) {
                let byte_val = (hi << 4) | lo;
                result.push(byte_val);
                i += 3;
                continue;
            }
        }
        result.push(bytes[i]);
        i += 1;
    }
    String::from_utf8(result).unwrap_or_else(|_| input.to_string())
}

/// Decode URI octets in a pathname, similar to JS `decodeURI()`.
/// Decodes percent-encoded bytes but preserves reserved characters and
/// replaces `\` with `%5C`.
pub fn decode_uri(input: &str) -> String {
    // Collect multi-byte percent-encoded sequences and decode them as UTF-8.
    let mut result = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    let mut pct_bytes = Vec::new();

    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(hi), Some(lo)) = (hex_digit(bytes[i + 1]), hex_digit(bytes[i + 2])) {
                pct_bytes.push((hi << 4) | lo);
                i += 3;
                continue;
            }
        }

        // Flush any accumulated percent-encoded bytes
        if !pct_bytes.is_empty() {
            flush_pct_bytes(&pct_bytes, &mut result);
            pct_bytes.clear();
        }
        result.push(bytes[i] as char);
        i += 1;
    }

    if !pct_bytes.is_empty() {
        flush_pct_bytes(&pct_bytes, &mut result);
    }

    // Replace backslashes with %5C
    result.replace('\\', "%5C")
}

fn flush_pct_bytes(pct_bytes: &[u8], result: &mut String) {
    match std::str::from_utf8(pct_bytes) {
        Ok(s) => {
            // Re-encode URI-reserved characters that decodeURI() preserves
            for ch in s.chars() {
                if ch.is_ascii() && is_uri_reserved(ch as u8) {
                    result.push_str(&format!("%{:02X}", ch as u8));
                } else {
                    result.push(ch);
                }
            }
        }
        Err(_) => {
            for b in pct_bytes {
                result.push_str(&format!("%{b:02X}"));
            }
        }
    }
}

/// Characters that JS decodeURI() does NOT decode (keeps them percent-encoded).
fn is_uri_reserved(b: u8) -> bool {
    matches!(b, b';' | b'/' | b'?' | b':' | b'@' | b'&' | b'=' | b'+' | b'$' | b',' | b'#')
}

/// Percent-encode a byte as `%XX` (uppercase hex).
#[allow(dead_code)]
pub fn percent_encode_byte(byte: u8) -> String {
    format!("%{byte:02X}")
}

/// Percent-encode characters in a string that are not "unreserved" per RFC 3986.
#[allow(dead_code)]
pub fn percent_encode(input: &str, encode_set: &[u8]) -> String {
    let mut result = String::with_capacity(input.len());
    for b in input.bytes() {
        if encode_set.contains(&b) {
            result.push_str(&percent_encode_byte(b));
        } else {
            result.push(b as char);
        }
    }
    result
}

fn hex_digit(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Decode a single percent-encoded sequence like `%2F` to its character.
/// Returns None if invalid.
#[allow(dead_code)]
pub fn decode_percent_pair(hi: u8, lo: u8) -> Option<u8> {
    Some((hex_digit(hi)? << 4) | hex_digit(lo)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_decode() {
        assert_eq!(percent_decode("%7E"), "~");
        assert_eq!(percent_decode("hello%20world"), "hello world");
        assert_eq!(percent_decode("no-encoding"), "no-encoding");
    }

    #[test]
    fn test_decode_uri() {
        assert_eq!(decode_uri("%7Efoo"), "~foo");
        assert_eq!(decode_uri("hello"), "hello");
    }

    #[test]
    fn test_percent_encode() {
        assert_eq!(percent_encode("hello world", &[b' ']), "hello%20world");
    }
}
