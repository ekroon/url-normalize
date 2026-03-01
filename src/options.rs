/// Options for URL normalization.
///
/// All options have sensible defaults matching the behavior of the original
/// `normalize-url` npm package.
pub struct Options {
    /// Default protocol to prepend if missing.
    ///
    /// Default: `Protocol::Http`
    pub default_protocol: Protocol,

    /// Additional protocols to normalize (beyond http, https, file, data).
    /// Protocols should be specified without `:`.
    ///
    /// Default: `vec![]`
    pub custom_protocols: Vec<String>,

    /// Prepend `default_protocol` to protocol-relative URLs.
    ///
    /// Default: `true`
    pub normalize_protocol: bool,

    /// Normalize HTTPS to HTTP.
    ///
    /// Default: `false`
    pub force_http: bool,

    /// Normalize HTTP to HTTPS. Cannot be used with `force_http`.
    ///
    /// Default: `false`
    pub force_https: bool,

    /// Strip the authentication part of the URL.
    ///
    /// Default: `true`
    pub strip_authentication: bool,

    /// Strip the hash/fragment part of the URL.
    ///
    /// Default: `false`
    pub strip_hash: bool,

    /// Remove the protocol from the URL.
    ///
    /// Default: `false`
    pub strip_protocol: bool,

    /// Strip the text fragment part of the URL (`#:~:text=...`).
    ///
    /// Default: `true`
    pub strip_text_fragment: bool,

    /// Remove `www.` from the URL.
    ///
    /// Default: `true`
    pub strip_www: bool,

    /// Controls removal of query parameters.
    ///
    /// Default: `RemoveQueryParameters::List` with a single filter matching `utm_*`
    pub remove_query_parameters: RemoveQueryParameters,

    /// If set, only keep query parameters matching these filters.
    /// Overrides `remove_query_parameters`.
    ///
    /// Default: `None`
    pub keep_query_parameters: Option<Vec<QueryFilter>>,

    /// Remove trailing slash from the path.
    ///
    /// Default: `true`
    pub remove_trailing_slash: bool,

    /// Remove a sole `/` pathname in the output.
    ///
    /// Default: `true`
    pub remove_single_slash: bool,

    /// Remove directory index files matching the given filters.
    ///
    /// Default: `RemoveDirectoryIndex::None`
    pub remove_directory_index: RemoveDirectoryIndex,

    /// Remove explicit port numbers.
    ///
    /// Default: `false`
    pub remove_explicit_port: bool,

    /// Sort query parameters alphabetically by key.
    ///
    /// Default: `true`
    pub sort_query_parameters: bool,

    /// Controls how empty query parameter values are formatted.
    ///
    /// Default: `EmptyQueryValue::Preserve`
    pub empty_query_value: EmptyQueryValue,

    /// Remove the entire URL path, leaving only the domain.
    ///
    /// Default: `false`
    pub remove_path: bool,

    /// Custom function to transform path components.
    ///
    /// Default: `None`
    pub transform_path: Option<Box<dyn Fn(Vec<String>) -> Vec<String>>>,
}

/// Default protocol for URLs without a scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Http,
    Https,
}

/// A filter for matching query parameter keys or path components.
///
/// Can be either an exact string match or a closure-based predicate.
pub enum QueryFilter {
    /// Match the parameter key exactly.
    Exact(String),
    /// Match using a predicate function. For regex matching,
    /// users can bring their own regex crate.
    Predicate(Box<dyn Fn(&str) -> bool>),
}

impl QueryFilter {
    /// Test whether a parameter name matches this filter.
    pub fn matches(&self, name: &str) -> bool {
        match self {
            QueryFilter::Exact(s) => s == name,
            QueryFilter::Predicate(f) => f(name),
        }
    }
}

/// Controls whether query parameters will be removed.
pub enum RemoveQueryParameters {
    /// No query parameters will be removed (disabled).
    None,
    /// All query parameters will be removed.
    All,
    /// Only query parameters matching any of the provided filters will be removed.
    List(Vec<QueryFilter>),
}

/// Controls whether directory index files will be removed from the path.
pub enum RemoveDirectoryIndex {
    /// No directory indices will be removed.
    None,
    /// Use the default pattern: `index.*`
    Default,
    /// Only directory indices matching any of the provided filters will be removed.
    List(Vec<QueryFilter>),
}

/// Controls how query parameters with empty values are formatted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmptyQueryValue {
    /// Keep the original format (`?key` stays `?key`, `?key=` stays `?key=`).
    Preserve,
    /// Always include `=` for empty values (`?key` becomes `?key=`).
    Always,
    /// Never include `=` for empty values (`?key=` becomes `?key`).
    Never,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            default_protocol: Protocol::Http,
            custom_protocols: vec![],
            normalize_protocol: true,
            force_http: false,
            force_https: false,
            strip_authentication: true,
            strip_hash: false,
            strip_protocol: false,
            strip_text_fragment: true,
            strip_www: true,
            remove_query_parameters: RemoveQueryParameters::List(vec![QueryFilter::Predicate(
                Box::new(|key: &str| {
                    key.len() >= 4
                        && key.is_char_boundary(4)
                        && key[..4].eq_ignore_ascii_case("utm_")
                }),
            )]),
            keep_query_parameters: None,
            remove_trailing_slash: true,
            remove_single_slash: true,
            remove_directory_index: RemoveDirectoryIndex::None,
            remove_explicit_port: false,
            sort_query_parameters: true,
            empty_query_value: EmptyQueryValue::Preserve,
            remove_path: false,
            transform_path: None,
        }
    }
}
