use crate::encode;
use crate::options::RemoveDirectoryIndex;
use crate::QueryFilter;

/// Remove duplicate slashes in a path, preserving protocol-like substrings
/// (e.g., `https://` embedded in the path).
pub fn remove_duplicate_slashes(path: &str) -> String {
    let mut result = String::with_capacity(path.len());
    let bytes = path.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        // Check if we're at the start of an embedded protocol like "http://" or "https://"
        if bytes[i] == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            // Look back to see if this is preceded by a scheme (e.g., "http:" or "https:")
            if is_preceded_by_scheme(&result) {
                // This is a protocol separator — keep both slashes
                result.push('/');
                result.push('/');
                i += 2;
                continue;
            }
            // Skip duplicate slashes
            result.push('/');
            while i < bytes.len() && bytes[i] == b'/' {
                i += 1;
            }
            continue;
        }
        result.push(bytes[i] as char);
        i += 1;
    }

    result
}

/// Check if the accumulated result ends with something like `http:` or `https:`
fn is_preceded_by_scheme(result: &str) -> bool {
    // Look for a pattern like `<letter><alphanum+-.>{1,50}:` at the end
    // Must be at least 2 chars (letter + at least one more), at most 51 chars
    if !result.ends_with(':') {
        return false;
    }
    let without_colon = &result[..result.len() - 1];
    // Find the start of the scheme (last `/` or start of string)
    let scheme_start = without_colon.rfind('/').map(|i| i + 1).unwrap_or(0);
    let scheme = &without_colon[scheme_start..];

    // Scheme must be at least 2 chars and at most 51 chars (matching JS {1,50} after first letter)
    if scheme.len() < 2 || scheme.len() > 51 {
        return false;
    }

    let bytes = scheme.as_bytes();
    bytes[0].is_ascii_alphabetic()
        && bytes[1..]
            .iter()
            .all(|b| b.is_ascii_alphanumeric() || *b == b'+' || *b == b'-' || *b == b'.')
}

/// Decode percent-encoded octets in a pathname, similar to JS `decodeURI()`.
pub fn decode_pathname(path: &str) -> String {
    encode::decode_uri(path)
}

/// Remove directory index file from the end of a path.
pub fn remove_directory_index(path: &mut String, option: &RemoveDirectoryIndex) {
    let filters: Option<Vec<&QueryFilter>> = match option {
        RemoveDirectoryIndex::None => None,
        RemoveDirectoryIndex::Default => {
            // Use inline check: matches "index." followed by lowercase letters
            let components: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
            if let Some(last) = components.last() {
                if last.starts_with("index.") && last.len() > 6 && last[6..].bytes().all(|b| b.is_ascii_lowercase())
                {
                    let new_components = components[..components.len() - 1].to_vec();
                    if new_components.is_empty() {
                        *path = "/".to_string();
                    } else {
                        *path = format!("/{}/", new_components.join("/"));
                    }
                }
            }
            return;
        }
        RemoveDirectoryIndex::List(filters) => Some(filters.iter().collect()),
    };

    if let Some(filters) = filters {
        if filters.is_empty() {
            return;
        }
        let components: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if let Some(last) = components.last() {
            if filters.iter().any(|f| f.matches(last)) {
                let new_components = &components[..components.len() - 1];
                if new_components.is_empty() {
                    *path = "/".to_string();
                } else {
                    *path = format!("/{}/", new_components.join("/"));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicate_slashes() {
        assert_eq!(remove_duplicate_slashes("/foo//bar"), "/foo/bar");
        assert_eq!(remove_duplicate_slashes("/foo///bar"), "/foo/bar");
        assert_eq!(remove_duplicate_slashes("/foo/bar"), "/foo/bar");
    }

    #[test]
    fn test_preserve_embedded_protocol() {
        assert_eq!(
            remove_duplicate_slashes("/https://bar.com/foo//bar"),
            "/https://bar.com/foo/bar"
        );
    }

    #[test]
    fn test_decode_pathname() {
        assert_eq!(decode_pathname("/%7Efoo/"), "/~foo/");
    }
}
