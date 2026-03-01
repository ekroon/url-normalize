use std::fmt;

/// A parsed URL split into its components.
#[derive(Debug, Clone)]
pub struct ParsedUrl {
    pub scheme: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

#[derive(Debug)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "URL parse error")
    }
}

impl std::error::Error for ParseError {}

impl ParsedUrl {
    /// Parse a URL string into components.
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let mut rest = input;

        // Extract scheme
        let scheme;
        if let Some(idx) = rest.find("://") {
            scheme = rest[..idx].to_lowercase();
            rest = &rest[idx + 3..];
        } else {
            return Err(ParseError);
        }

        // Extract fragment
        let fragment;
        if let Some(hash_idx) = rest.find('#') {
            fragment = Some(rest[hash_idx + 1..].to_string());
            rest = &rest[..hash_idx];
        } else {
            fragment = None;
        }

        // Extract query
        let query;
        if let Some(q_idx) = rest.find('?') {
            let q = rest[q_idx + 1..].to_string();
            query = Some(q);
            rest = &rest[..q_idx];
        } else {
            query = None;
        }

        // Split authority from path (backslash counts as path separator)
        let (authority, path) = {
            let first_sep = rest.find(['/', '\\']);
            if let Some(sep_idx) = first_sep {
                (&rest[..sep_idx], rest[sep_idx..].replace('\\', "/"))
            } else {
                (rest, String::new())
            }
        };

        // Parse authority: [user[:password]@]host[:port]
        let (userinfo, hostport) = if let Some(at_idx) = authority.rfind('@') {
            (&authority[..at_idx], &authority[at_idx + 1..])
        } else {
            ("", authority)
        };

        let (username, password) = if userinfo.is_empty() {
            (String::new(), String::new())
        } else if let Some(colon_idx) = userinfo.find(':') {
            (
                userinfo[..colon_idx].to_string(),
                userinfo[colon_idx + 1..].to_string(),
            )
        } else {
            (userinfo.to_string(), String::new())
        };

        // Parse host:port — handle IPv6 [::1]:port
        let (host_raw, port) = if hostport.starts_with('[') {
            // IPv6
            if let Some(bracket_end) = hostport.find(']') {
                let host_part = &hostport[..bracket_end + 1];
                let after = &hostport[bracket_end + 1..];
                let port = if let Some(colon_rest) = after.strip_prefix(':') {
                    colon_rest.parse::<u16>().ok()
                } else {
                    None
                };
                (host_part.to_string(), port)
            } else {
                (hostport.to_string(), None)
            }
        } else if let Some(colon_idx) = hostport.rfind(':') {
            // Could be host:port
            let potential_port = &hostport[colon_idx + 1..];
            if let Ok(p) = potential_port.parse::<u16>() {
                (hostport[..colon_idx].to_string(), Some(p))
            } else {
                (hostport.to_string(), None)
            }
        } else {
            (hostport.to_string(), None)
        };

        // IDNA encode the host
        let host = encode_host(&host_raw);

        // Resolve path (handle `.` and `..` segments)
        let resolved_path = if path.is_empty() {
            // For http/https/file, default to "/"
            let s = scheme.as_str();
            if s == "http" || s == "https" || s == "file" {
                "/".to_string()
            } else {
                String::new()
            }
        } else {
            resolve_path(&path)
        };

        Ok(ParsedUrl {
            scheme,
            username,
            password,
            host,
            port,
            path: resolved_path,
            query,
            fragment,
        })
    }
}

impl fmt::Display for ParsedUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}://", self.scheme)?;

        if !self.username.is_empty() {
            write!(f, "{}", self.username)?;
            if !self.password.is_empty() {
                write!(f, ":{}", self.password)?;
            }
            write!(f, "@")?;
        }

        write!(f, "{}", self.host)?;

        if let Some(port) = self.port {
            write!(f, ":{port}")?;
        }

        write!(f, "{}", self.path)?;

        if let Some(ref q) = self.query {
            write!(f, "?{q}")?;
        }

        if let Some(ref frag) = self.fragment {
            write!(f, "#{frag}")?;
        }

        Ok(())
    }
}

/// IDNA-encode a hostname using the `idna` crate.
fn encode_host(host: &str) -> String {
    if host.is_empty() || host.starts_with('[') {
        return host.to_string();
    }

    // Check if host contains non-ASCII characters
    if host.is_ascii() {
        return host.to_lowercase();
    }

    // Use idna crate for international domain names
    match idna::domain_to_ascii(host) {
        Ok(ascii) => ascii,
        Err(_) => host.to_lowercase(),
    }
}

/// Resolve `.` and `..` path segments.
fn resolve_path(path: &str) -> String {
    let mut segments: Vec<&str> = Vec::new();
    for segment in path.split('/') {
        match segment {
            "." => {}
            ".." => {
                segments.pop();
            }
            s => segments.push(s),
        }
    }
    let result = segments.join("/");
    if result.is_empty() || !result.starts_with('/') {
        format!("/{result}")
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parse() {
        let url = ParsedUrl::parse("http://example.com/path?q=1#frag").unwrap();
        assert_eq!(url.scheme, "http");
        assert_eq!(url.host, "example.com");
        assert_eq!(url.path, "/path");
        assert_eq!(url.query.as_deref(), Some("q=1"));
        assert_eq!(url.fragment.as_deref(), Some("frag"));
    }

    #[test]
    fn test_auth_parse() {
        let url = ParsedUrl::parse("http://user:pass@example.com/").unwrap();
        assert_eq!(url.username, "user");
        assert_eq!(url.password, "pass");
        assert_eq!(url.host, "example.com");
    }

    #[test]
    fn test_port_parse() {
        let url = ParsedUrl::parse("http://example.com:8080/").unwrap();
        assert_eq!(url.port, Some(8080));
    }

    #[test]
    fn test_dot_segment_resolution() {
        let url = ParsedUrl::parse("http://example.com/foo/bar/../baz").unwrap();
        assert_eq!(url.path, "/foo/baz");
    }

    #[test]
    fn test_dot_resolution() {
        let url = ParsedUrl::parse("http://example.com/foo/./bar").unwrap();
        assert_eq!(url.path, "/foo/bar");
    }

    #[test]
    fn test_to_string() {
        let url = ParsedUrl::parse("http://example.com/path?q=1#frag").unwrap();
        assert_eq!(url.to_string(), "http://example.com/path?q=1#frag");
    }

    #[test]
    fn test_idna() {
        let url = ParsedUrl::parse("http://êxample.com/").unwrap();
        assert_eq!(url.host, "xn--xample-hva.com");
    }
}
