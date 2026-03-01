mod data_url;
mod encode;
mod options;
mod path;
mod query;
mod url_parser;

pub use options::*;

use std::fmt;

/// Errors that can occur during URL normalization.
#[derive(Debug)]
pub enum NormalizeUrlError {
    /// The URL string could not be parsed.
    InvalidUrl(String),
    /// `force_http` and `force_https` cannot be used together.
    ConflictingOptions,
}

impl fmt::Display for NormalizeUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUrl(url) => write!(f, "Invalid URL: {url}"),
            Self::ConflictingOptions => {
                write!(f, "The `force_http` and `force_https` options cannot be used together")
            }
        }
    }
}

impl std::error::Error for NormalizeUrlError {}

/// Normalize a URL string according to the provided options.
///
/// # Examples
///
/// ```
/// use url_normalize::{normalize_url, Options};
///
/// let result = normalize_url("https://www.example.com/foo/", &Options::default()).unwrap();
/// assert_eq!(result, "https://example.com/foo");
/// ```
pub fn normalize_url(url_string: &str, options: &Options) -> Result<String, NormalizeUrlError> {
    let mut url_string = url_string.trim().to_string();

    if options.force_http && options.force_https {
        return Err(NormalizeUrlError::ConflictingOptions);
    }

    // Data URL
    if url_string
        .get(..5)
        .map(|s| s.eq_ignore_ascii_case("data:"))
        .unwrap_or(false)
    {
        return data_url::normalize_data_url(&url_string, options.strip_hash);
    }

    // Custom protocol detection
    let custom_protocol = detect_custom_protocol(&url_string);
    let normalized_custom_protocols: Vec<String> = options
        .custom_protocols
        .iter()
        .filter_map(|p| {
            let p = p.trim().to_lowercase();
            let p = p.strip_suffix(':').unwrap_or(&p).to_string();
            if p.is_empty() {
                None
            } else {
                Some(format!("{p}:"))
            }
        })
        .collect();

    if let Some(ref cp) = custom_protocol {
        if !normalized_custom_protocols.iter().any(|p| p == cp) {
            return Ok(url_string);
        }
    }

    let has_relative_protocol = url_string.starts_with("//");
    let is_relative_url = !has_relative_protocol
        && (url_string.starts_with("./") || url_string.starts_with("../"));

    // Reject invalid relative paths like "/" or "/relative/path/"
    if !is_relative_url && !has_relative_protocol && custom_protocol.is_none() {
        if url_string == "/" || (url_string.starts_with('/') && !url_string.starts_with("//")) {
            return Err(NormalizeUrlError::InvalidUrl(url_string));
        }
    }

    // Prepend protocol
    if !is_relative_url && custom_protocol.is_none() {
        let default_proto = match options.default_protocol {
            Protocol::Http => "http:",
            Protocol::Https => "https:",
        };

        if has_relative_protocol {
            url_string = format!("{default_proto}{url_string}");
        } else if !url_string.contains("://") {
            url_string = format!("{default_proto}//{url_string}");
        }
    }

    let mut parsed = url_parser::ParsedUrl::parse(&url_string)
        .map_err(|_| NormalizeUrlError::InvalidUrl(url_string.clone()))?;

    // Reject URLs with empty host (like "http://")
    if parsed.host.is_empty()
        && (parsed.scheme == "http" || parsed.scheme == "https")
    {
        return Err(NormalizeUrlError::InvalidUrl(url_string));
    }

    // Force HTTP / HTTPS
    if options.force_http && parsed.scheme.eq_ignore_ascii_case("https") {
        parsed.scheme = "http".to_string();
    }
    if options.force_https && parsed.scheme.eq_ignore_ascii_case("http") {
        parsed.scheme = "https".to_string();
    }

    // Strip authentication
    if options.strip_authentication {
        parsed.username = String::new();
        parsed.password = String::new();
    }

    // Strip hash
    if options.strip_hash {
        parsed.fragment = None;
    } else if options.strip_text_fragment {
        if let Some(ref mut frag) = parsed.fragment {
            if let Some(idx) = frag.find(":~:text") {
                if idx == 0 {
                    *frag = String::new();
                } else {
                    frag.truncate(idx);
                }
            }
        }
    }

    // Remove empty fragment (bare #)
    if let Some(ref frag) = parsed.fragment {
        if frag.is_empty() {
            parsed.fragment = None;
        }
    }

    // Path normalization: remove duplicate slashes
    if !parsed.path.is_empty() {
        parsed.path = path::remove_duplicate_slashes(&parsed.path);
    }

    // Decode URI octets in pathname
    if !parsed.path.is_empty() {
        parsed.path = path::decode_pathname(&parsed.path);
    }

    // Remove directory index
    path::remove_directory_index(&mut parsed.path, &options.remove_directory_index);

    // Remove path
    if options.remove_path {
        parsed.path = "/".to_string();
    }

    // Transform path
    if let Some(ref transform) = options.transform_path {
        let components: Vec<String> = parsed
            .path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        let new_components = transform(components);
        if new_components.is_empty() {
            parsed.path = "/".to_string();
        } else {
            parsed.path = format!("/{}", new_components.join("/"));
        }
    }

    // Hostname normalization
    if !parsed.host.is_empty() {
        // Remove trailing dot
        if parsed.host.ends_with('.') {
            parsed.host.pop();
        }

        // Strip www.
        if options.strip_www {
            let host = parsed.host.to_lowercase();
            if host.starts_with("www.")
                && !host[4..].starts_with("www.")
                && is_valid_www_strip(&host)
            {
                parsed.host = parsed.host[4..].to_string();
            }
        }
    }

    // Remove default ports
    match parsed.port {
        Some(80) if parsed.scheme.eq_ignore_ascii_case("http") => parsed.port = None,
        Some(443) if parsed.scheme.eq_ignore_ascii_case("https") => parsed.port = None,
        _ => {}
    }

    // Query parameter operations
    let original_query = parsed.query.clone();
    query::process_query(&mut parsed, options, original_query.as_deref());

    // Remove trailing slash
    if options.remove_trailing_slash && parsed.path.ends_with('/') && parsed.path.len() > 1 {
        parsed.path.pop();
    }

    // Remove explicit port
    if options.remove_explicit_port {
        parsed.port = None;
    }

    // Build URL string
    let old_url_string = url_string.clone();
    url_string = parsed.to_string();

    // Single slash handling
    if !options.remove_single_slash
        && parsed.path == "/"
        && !old_url_string.ends_with('/')
        && parsed.fragment.is_none()
    {
        if url_string.ends_with('/') {
            url_string.pop();
        }
    }

    // Remove ending `/`
    if (options.remove_trailing_slash || parsed.path == "/")
        && parsed.fragment.is_none()
        && options.remove_single_slash
    {
        if url_string.ends_with('/') {
            url_string.pop();
        }
    }

    // Restore relative protocol
    if has_relative_protocol && !options.normalize_protocol {
        if let Some(rest) = url_string.strip_prefix("http://") {
            url_string = format!("//{rest}");
        }
    }

    // Strip protocol
    if options.strip_protocol {
        if let Some(rest) = url_string.strip_prefix("https://") {
            url_string = rest.to_string();
        } else if let Some(rest) = url_string.strip_prefix("http://") {
            url_string = rest.to_string();
        } else if let Some(rest) = url_string.strip_prefix("//") {
            url_string = rest.to_string();
        }
    }

    Ok(url_string)
}

fn detect_custom_protocol(url_string: &str) -> Option<String> {
    if let Some(colon_idx) = url_string.find(':') {
        let scheme = &url_string[..colon_idx];
        if !scheme.is_empty()
            && scheme.as_bytes()[0].is_ascii_alphabetic()
            && scheme
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'+' || b == b'-' || b == b'.')
        {
            let lower_scheme = scheme.to_lowercase();
            let has_authority = url_string
                .get(colon_idx + 1..colon_idx + 3)
                .map(|s| s == "//")
                .unwrap_or(false);

            if lower_scheme != "http"
                && lower_scheme != "https"
                && lower_scheme != "file"
                && lower_scheme != "data"
            {
                if !lower_scheme.contains('.') || has_authority {
                    return Some(format!("{lower_scheme}:"));
                }
            }
        }
    }
    None
}

fn is_valid_www_strip(host: &str) -> bool {
    let without_www = &host[4..];
    if let Some(dot_idx) = without_www.find('.') {
        let label = &without_www[..dot_idx];
        let rest = &without_www[dot_idx + 1..];
        if label.is_empty()
            || label.len() > 63
            || !label
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        {
            return false;
        }
        if rest.is_empty()
            || rest.len() < 2
            || rest.len() > 63
            || !rest
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'.')
        {
            return false;
        }
        true
    } else {
        false
    }
}
