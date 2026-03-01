use crate::NormalizeUrlError;

/// Normalize a data URL.
///
/// Handles: MIME type lowercasing, charset normalization, base64 flag,
/// default MIME type removal.
pub fn normalize_data_url(url_string: &str, strip_hash: bool) -> Result<String, NormalizeUrlError> {
    // data:[<type>],<data>[#<hash>]
    let without_prefix = &url_string[5..]; // skip "data:"

    // Find the comma separating type from data
    let comma_idx = without_prefix
        .find(',')
        .ok_or_else(|| NormalizeUrlError::InvalidUrl(url_string.to_string()))?;

    let type_part = &without_prefix[..comma_idx];
    let rest = &without_prefix[comma_idx + 1..];

    // Split data and hash
    let (data, hash) = if let Some(hash_idx) = rest.find('#') {
        let hash_content = rest[hash_idx + 1..].trim();
        if hash_content.is_empty() {
            // Empty hash (bare #) — strip it
            (&rest[..hash_idx], None)
        } else {
            (&rest[..hash_idx], Some(hash_content))
        }
    } else {
        (rest, None)
    };

    // Parse media type
    let mut media_type_parts: Vec<&str> = type_part.split(';').collect();

    let is_base64 = media_type_parts
        .last()
        .map(|s| s.trim() == "base64")
        .unwrap_or(false);
    if is_base64 {
        media_type_parts.pop();
    }

    // First part is the MIME type — lowercase it
    let mime_type = if !media_type_parts.is_empty() {
        media_type_parts.remove(0).to_lowercase()
    } else {
        String::new()
    };

    // Process attributes
    let mut attributes: Vec<String> = Vec::new();
    for attr in &media_type_parts {
        let attr = attr.trim();
        if let Some(eq_idx) = attr.find('=') {
            let key = &attr[..eq_idx];
            let mut value = attr[eq_idx + 1..].to_string();

            // Lowercase charset, remove default charset
            if key.eq_ignore_ascii_case("charset") {
                value = value.to_lowercase();
                if value == "us-ascii" {
                    continue;
                }
            }

            if value.is_empty() {
                // Key-only attribute (empty value after =)
                attributes.push(key.to_string());
            } else {
                attributes.push(format!("{key}={value}"));
            }
        } else if !attr.is_empty() {
            attributes.push(attr.to_string());
        }
    }

    // Build normalized media type
    let mut normalized_media_type: Vec<String> = attributes;

    if is_base64 {
        normalized_media_type.push("base64".to_string());
    }

    if !normalized_media_type.is_empty()
        || (!mime_type.is_empty() && mime_type != "text/plain")
    {
        normalized_media_type.insert(0, mime_type.clone());
    }

    let hash_part = if strip_hash || hash.is_none() {
        String::new()
    } else {
        format!("#{}", hash.unwrap())
    };

    let data_part = if is_base64 { data.trim() } else { data };

    Ok(format!(
        "data:{},{}{}",
        normalized_media_type.join(";"),
        data_part,
        hash_part
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_data_url() {
        let result = normalize_data_url("data:text/html,<h1>Hello</h1>", false).unwrap();
        assert_eq!(result, "data:text/html,<h1>Hello</h1>");
    }

    #[test]
    fn test_default_mime_type_removal() {
        let result = normalize_data_url("data:text/plain,hello", false).unwrap();
        assert_eq!(result, "data:,hello");
    }

    #[test]
    fn test_mime_type_lowercase() {
        let result = normalize_data_url("data:TEXT/HTML,hello", false).unwrap();
        assert_eq!(result, "data:text/html,hello");
    }

    #[test]
    fn test_charset_lowercase() {
        let result =
            normalize_data_url("data:text/plain;charset=UTF-8,hello", false).unwrap();
        assert_eq!(result, "data:text/plain;charset=utf-8,hello");
    }

    #[test]
    fn test_default_charset_removal() {
        let result =
            normalize_data_url("data:text/plain;charset=us-ascii,hello", false).unwrap();
        // default MIME type text/plain is removed, default charset us-ascii is removed
        assert_eq!(result, "data:,hello");
    }

    #[test]
    fn test_base64() {
        let result = normalize_data_url("data:text/html;base64,SGVsbG8=", false).unwrap();
        assert_eq!(result, "data:text/html;base64,SGVsbG8=");
    }

    #[test]
    fn test_strip_hash() {
        let result =
            normalize_data_url("data:text/html,hello#fragment", true).unwrap();
        assert_eq!(result, "data:text/html,hello");
    }

    #[test]
    fn test_preserve_hash() {
        let result =
            normalize_data_url("data:text/html,hello#fragment", false).unwrap();
        assert_eq!(result, "data:text/html,hello#fragment");
    }
}
