//! Comprehensive test suite ported from sindresorhus/normalize-url test.js (v9.0.0)

use url_normalize::*;

fn n(url: &str) -> String {
    normalize_url(url, &Options::default()).unwrap()
}

fn nopt(url: &str, opts: Options) -> String {
    normalize_url(url, &opts).unwrap()
}

// ============================================================
// Main tests
// ============================================================

#[test]
fn test_main() {
    assert_eq!(n("sindresorhus.com"), "http://sindresorhus.com");
    assert_eq!(n("sindresorhus.com "), "http://sindresorhus.com");
    assert_eq!(n("sindresorhus.com."), "http://sindresorhus.com");
    assert_eq!(n("SindreSorhus.com"), "http://sindresorhus.com");
    assert_eq!(n("HTTP://sindresorhus.com"), "http://sindresorhus.com");
    assert_eq!(n("//sindresorhus.com"), "http://sindresorhus.com");
    assert_eq!(n("http://sindresorhus.com"), "http://sindresorhus.com");
    assert_eq!(n("http://sindresorhus.com:80"), "http://sindresorhus.com");
    assert_eq!(n("https://sindresorhus.com:443"), "https://sindresorhus.com");
    assert_eq!(n("http://www.sindresorhus.com"), "http://sindresorhus.com");
    assert_eq!(n("www.com"), "http://www.com");
    assert_eq!(n("http://www.www.sindresorhus.com"), "http://www.www.sindresorhus.com");
    assert_eq!(n("www.sindresorhus.com"), "http://sindresorhus.com");
    assert_eq!(n("http://sindresorhus.com/foo/"), "http://sindresorhus.com/foo");
    assert_eq!(n("https://foo.com/https://bar.com"), "https://foo.com/https://bar.com");
    assert_eq!(n("https://foo.com/https://bar.com/foo//bar"), "https://foo.com/https://bar.com/foo/bar");
    assert_eq!(n("https://foo.com/http://bar.com"), "https://foo.com/http://bar.com");
    assert_eq!(n("https://foo.com/http://bar.com/foo//bar"), "https://foo.com/http://bar.com/foo/bar");
    assert_eq!(n("http://sindresorhus.com/%7Efoo/"), "http://sindresorhus.com/~foo");
    assert_eq!(n("http://sindresorhus.com/?"), "http://sindresorhus.com");
    assert_eq!(n("êxample.com"), "http://xn--xample-hva.com");
    assert_eq!(n("http://sindresorhus.com/?b=bar&a=foo"), "http://sindresorhus.com/?a=foo&b=bar");
    assert_eq!(n("http://sindresorhus.com:5000"), "http://sindresorhus.com:5000");
    assert_eq!(n("http://sindresorhus.com/foo#bar"), "http://sindresorhus.com/foo#bar");
    assert_eq!(n("http://sindresorhus.com/foo/bar/../baz"), "http://sindresorhus.com/foo/baz");
    assert_eq!(n("http://sindresorhus.com/foo/bar/./baz"), "http://sindresorhus.com/foo/bar/baz");
    assert_eq!(n("sindresorhus.com:123"), "http://sindresorhus.com:123");
    // URLs as query values should be preserved
    assert_eq!(
        n("https://i.vimeocdn.com/filter/overlay?src0=https://i.vimeocdn.com/video/598160082_1280x720.jpg&src1=https://f.vimeocdn.com/images_v6/share/play_icon_overlay.png"),
        "https://i.vimeocdn.com/filter/overlay?src0=https://i.vimeocdn.com/video/598160082_1280x720.jpg&src1=https://f.vimeocdn.com/images_v6/share/play_icon_overlay.png"
    );
}

#[test]
fn test_normalize_protocol_option() {
    let opts = || Options { normalize_protocol: false, ..Options::default() };
    assert_eq!(nopt("//sindresorhus.com/", opts()), "//sindresorhus.com");
    assert_eq!(nopt("//sindresorhus.com:80/", opts()), "//sindresorhus.com");
}

// ============================================================
// defaultProtocol option
// ============================================================

#[test]
fn test_default_protocol() {
    assert_eq!(nopt("sindresorhus.com", Options { default_protocol: Protocol::Https, ..Options::default() }), "https://sindresorhus.com");
    assert_eq!(nopt("sindresorhus.com", Options { default_protocol: Protocol::Http, ..Options::default() }), "http://sindresorhus.com");
}

// ============================================================
// stripAuthentication option
// ============================================================

#[test]
fn test_strip_authentication() {
    assert_eq!(n("http://user:password@www.sindresorhus.com"), "http://sindresorhus.com");
    assert_eq!(n("https://user:password@www.sindresorhus.com"), "https://sindresorhus.com");
    assert_eq!(n("https://user:password@www.sindresorhus.com/@user"), "https://sindresorhus.com/@user");
    assert_eq!(n("http://user:password@www.êxample.com"), "http://xn--xample-hva.com");

    let opts = || Options { strip_authentication: false, ..Options::default() };
    assert_eq!(nopt("http://user:password@www.sindresorhus.com", opts()), "http://user:password@sindresorhus.com");
    assert_eq!(nopt("https://user:password@www.sindresorhus.com", opts()), "https://user:password@sindresorhus.com");
    assert_eq!(nopt("https://user:password@www.sindresorhus.com/@user", opts()), "https://user:password@sindresorhus.com/@user");
    assert_eq!(nopt("http://user:password@www.êxample.com", opts()), "http://user:password@xn--xample-hva.com");
}

// ============================================================
// stripProtocol option
// ============================================================

#[test]
fn test_strip_protocol() {
    let opts = || Options { strip_protocol: true, ..Options::default() };
    assert_eq!(nopt("http://www.sindresorhus.com", opts()), "sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com", opts()), "sindresorhus.com");
    assert_eq!(nopt("https://www.sindresorhus.com", opts()), "sindresorhus.com");
    assert_eq!(nopt("//www.sindresorhus.com", opts()), "sindresorhus.com");
}

// ============================================================
// stripTextFragment option
// ============================================================

#[test]
fn test_strip_text_fragment() {
    assert_eq!(n("http://sindresorhus.com"), "http://sindresorhus.com");
    assert_eq!(n("http://sindresorhus.com/about#"), "http://sindresorhus.com/about");
    assert_eq!(n("http://sindresorhus.com/about#:~:text=hello"), "http://sindresorhus.com/about");
    assert_eq!(n("http://sindresorhus.com/about#main"), "http://sindresorhus.com/about#main");
    assert_eq!(n("http://sindresorhus.com/about#main:~:text=hello"), "http://sindresorhus.com/about#main");
    assert_eq!(n("http://sindresorhus.com/about#main:~:text=hello%20world"), "http://sindresorhus.com/about#main");

    let opts = || Options { strip_text_fragment: false, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com", opts()), "http://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com/about#:~:text=hello", opts()), "http://sindresorhus.com/about#:~:text=hello");
    assert_eq!(nopt("http://sindresorhus.com/about#main", opts()), "http://sindresorhus.com/about#main");
    assert_eq!(nopt("http://sindresorhus.com/about#main:~:text=hello", opts()), "http://sindresorhus.com/about#main:~:text=hello");

    let opts2 = || Options { strip_hash: true, strip_text_fragment: false, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com", opts2()), "http://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com/about#:~:text=hello", opts2()), "http://sindresorhus.com/about");
    assert_eq!(nopt("http://sindresorhus.com/about#main", opts2()), "http://sindresorhus.com/about");
    assert_eq!(nopt("http://sindresorhus.com/about#main:~:text=hello", opts2()), "http://sindresorhus.com/about");
}

// ============================================================
// stripWWW option
// ============================================================

#[test]
fn test_strip_www() {
    let opts_off = || Options { strip_www: false, ..Options::default() };
    assert_eq!(nopt("http://www.sindresorhus.com", opts_off()), "http://www.sindresorhus.com");
    assert_eq!(nopt("www.sindresorhus.com", opts_off()), "http://www.sindresorhus.com");

    assert_eq!(n("http://www.vue.amsterdam"), "http://vue.amsterdam");
    assert_eq!(n("http://www.sorhus.xx--bck1b9a5dre4c"), "http://sorhus.xx--bck1b9a5dre4c");

    // TLD too long (> 63 chars)
    let long_tld = format!("http://www.sorhus.{}", "a".repeat(64));
    assert_eq!(n(&long_tld), long_tld);

    // Multi-level subdomains
    assert_eq!(n("www.unix.stackexchange.com"), "http://unix.stackexchange.com");
    assert_eq!(n("https://www.unix.stackexchange.com"), "https://unix.stackexchange.com");
    assert_eq!(n("www.api.example.com"), "http://api.example.com");

    // www.com should NOT be stripped
    assert_eq!(n("www.com"), "http://www.com");
    assert_eq!(n("https://www.com"), "https://www.com");

    // www.www.com should NOT be stripped
    assert_eq!(n("www.www.com"), "http://www.www.com");
    assert_eq!(n("www.www.example.com"), "http://www.www.example.com");
}

// ============================================================
// stripHash option
// ============================================================

#[test]
fn test_strip_hash() {
    let opts = || Options { strip_hash: true, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com/foo#bar", opts()), "http://sindresorhus.com/foo");
    assert_eq!(nopt("http://sindresorhus.com/foo#bar:~:text=hello%20world", opts()), "http://sindresorhus.com/foo");
}

// ============================================================
// removeQueryParameters option
// ============================================================

#[test]
fn test_remove_query_parameters_default() {
    assert_eq!(n("www.sindresorhus.com?foo=bar&utm_medium=test"), "http://sindresorhus.com/?foo=bar");
}

#[test]
fn test_remove_query_parameters_custom() {
    let opts = Options {
        remove_query_parameters: RemoveQueryParameters::List(vec![
            QueryFilter::Predicate(Box::new(|key: &str| key.len() >= 4 && key[..4].eq_ignore_ascii_case("utm_"))),
            QueryFilter::Exact("ref".to_string()),
        ]),
        strip_www: false,
        ..Options::default()
    };
    assert_eq!(nopt("www.sindresorhus.com?foo=bar&utm_medium=test&ref=test_ref", opts), "http://www.sindresorhus.com/?foo=bar");
}

#[test]
fn test_remove_query_parameters_all() {
    let opts = || Options {
        remove_query_parameters: RemoveQueryParameters::All,
        strip_www: false,
        ..Options::default()
    };
    assert_eq!(nopt("http://www.sindresorhus.com", opts()), "http://www.sindresorhus.com");
    assert_eq!(nopt("www.sindresorhus.com?foo=bar", opts()), "http://www.sindresorhus.com");
    assert_eq!(nopt("www.sindresorhus.com?foo=bar&utm_medium=test&ref=test_ref", opts()), "http://www.sindresorhus.com");
}

#[test]
fn test_remove_query_parameters_none() {
    let opts = Options {
        remove_query_parameters: RemoveQueryParameters::None,
        strip_www: false,
        ..Options::default()
    };
    assert_eq!(nopt("www.sindresorhus.com?foo=bar&utm_medium=test", opts), "http://www.sindresorhus.com/?foo=bar&utm_medium=test");
}

// ============================================================
// keepQueryParameters option
// ============================================================

#[test]
fn test_keep_query_parameters() {
    let opts = || Options {
        keep_query_parameters: Some(vec![QueryFilter::Exact("ref".to_string())]),
        ..Options::default()
    };
    assert_eq!(nopt("https://sindresorhus.com?foo=bar&ref=unicorn", opts()), "https://sindresorhus.com/?ref=unicorn");
}

#[test]
fn test_keep_query_parameters_empty() {
    let opts = Options { keep_query_parameters: Some(vec![]), ..Options::default() };
    assert_eq!(nopt("https://sindresorhus.com?foo=bar&ref=unicorn", opts), "https://sindresorhus.com");
}

#[test]
fn test_keep_overrides_remove() {
    let opts = Options {
        remove_query_parameters: RemoveQueryParameters::List(vec![QueryFilter::Exact("foo".to_string())]),
        keep_query_parameters: Some(vec![QueryFilter::Exact("foo".to_string()), QueryFilter::Exact("bar".to_string())]),
        ..Options::default()
    };
    assert_eq!(nopt("https://example.com?foo=1&bar=2", opts), "https://example.com/?bar=2&foo=1");
}

#[test]
fn test_keep_with_remove_all() {
    let opts = Options {
        remove_query_parameters: RemoveQueryParameters::All,
        keep_query_parameters: Some(vec![QueryFilter::Exact("foo".to_string())]),
        ..Options::default()
    };
    assert_eq!(nopt("https://example.com?foo=1&bar=2", opts), "https://example.com/?foo=1");
}

// ============================================================
// forceHttp / forceHttps options
// ============================================================

#[test]
fn test_force_http() {
    let opts = || Options { force_http: true, ..Options::default() };
    assert_eq!(n("https://sindresorhus.com"), "https://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com", opts()), "http://sindresorhus.com");
    assert_eq!(nopt("https://www.sindresorhus.com", opts()), "http://sindresorhus.com");
    assert_eq!(nopt("//sindresorhus.com", opts()), "http://sindresorhus.com");
}

#[test]
fn test_force_https() {
    let opts = || Options { force_https: true, ..Options::default() };
    assert_eq!(n("https://sindresorhus.com"), "https://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com", opts()), "https://sindresorhus.com");
    assert_eq!(nopt("https://www.sindresorhus.com", opts()), "https://sindresorhus.com");
    assert_eq!(nopt("//sindresorhus.com", opts()), "https://sindresorhus.com");
}

#[test]
fn test_force_http_https_conflict() {
    let opts = Options { force_http: true, force_https: true, ..Options::default() };
    assert!(normalize_url("http://sindresorhus.com", &opts).is_err());
}

// ============================================================
// removeTrailingSlash option
// ============================================================

#[test]
fn test_remove_trailing_slash() {
    assert_eq!(n("http://sindresorhus.com"), "http://sindresorhus.com");
    assert_eq!(n("http://sindresorhus.com/"), "http://sindresorhus.com");
    assert_eq!(n("http://sindresorhus.com/redirect"), "http://sindresorhus.com/redirect");
    assert_eq!(n("http://sindresorhus.com/redirect/"), "http://sindresorhus.com/redirect");

    let opts = || Options { remove_trailing_slash: false, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com", opts()), "http://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com/", opts()), "http://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com/redirect/", opts()), "http://sindresorhus.com/redirect/");

    // Hash with trailing slash preserved
    assert_eq!(n("http://sindresorhus.com/#/"), "http://sindresorhus.com/#/");
    let opts2 = Options { remove_trailing_slash: false, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com/#/", opts2), "http://sindresorhus.com/#/");

    assert_eq!(n("http://sindresorhus.com/?unicorns=true"), "http://sindresorhus.com/?unicorns=true");
}

// ============================================================
// removeExplicitPort option
// ============================================================

#[test]
fn test_remove_explicit_port() {
    let opts = || Options { remove_explicit_port: true, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com:123", opts()), "http://sindresorhus.com");
    assert_eq!(nopt("https://sindresorhus.com:123", opts()), "https://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com:443", opts()), "http://sindresorhus.com");
    assert_eq!(nopt("https://sindresorhus.com:80", opts()), "https://sindresorhus.com");
}

// ============================================================
// removeSingleSlash option
// ============================================================

#[test]
fn test_remove_single_slash() {
    assert_eq!(n("https://sindresorhus.com/"), "https://sindresorhus.com");
    let opts = || Options { remove_single_slash: false, ..Options::default() };
    assert_eq!(nopt("https://sindresorhus.com", opts()), "https://sindresorhus.com");
    assert_eq!(nopt("https://sindresorhus.com/", opts()), "https://sindresorhus.com/");
    assert_eq!(nopt("https://sindresorhus.com/redirect", opts()), "https://sindresorhus.com/redirect");
    assert_eq!(nopt("https://sindresorhus.com/redirect/", opts()), "https://sindresorhus.com/redirect");
    assert_eq!(nopt("https://sindresorhus.com/#/", opts()), "https://sindresorhus.com/#/");
    assert_eq!(nopt("https://sindresorhus.com/?unicorns=true", opts()), "https://sindresorhus.com/?unicorns=true");
}

#[test]
fn test_remove_single_slash_combined_with_trailing_slash() {
    let opts = || Options { remove_trailing_slash: false, remove_single_slash: false, ..Options::default() };
    assert_eq!(nopt("https://sindresorhus.com", opts()), "https://sindresorhus.com");
    assert_eq!(nopt("https://sindresorhus.com/", opts()), "https://sindresorhus.com/");
    assert_eq!(nopt("https://sindresorhus.com/redirect", opts()), "https://sindresorhus.com/redirect");
    assert_eq!(nopt("https://sindresorhus.com/redirect/", opts()), "https://sindresorhus.com/redirect/");
    assert_eq!(nopt("https://sindresorhus.com/#/", opts()), "https://sindresorhus.com/#/");
}

// ============================================================
// removeDirectoryIndex option
// ============================================================

#[test]
fn test_remove_directory_index_not_by_default() {
    assert_eq!(n("http://sindresorhus.com/index.html"), "http://sindresorhus.com/index.html");
    assert_eq!(n("http://sindresorhus.com/path/index.html"), "http://sindresorhus.com/path/index.html");
}

#[test]
fn test_remove_directory_index_default() {
    let opts = || Options { remove_directory_index: RemoveDirectoryIndex::Default, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com/index.html", opts()), "http://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com/index.htm", opts()), "http://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com/index.php", opts()), "http://sindresorhus.com");
}

#[test]
fn test_remove_directory_index_custom() {
    let opts = Options {
        remove_directory_index: RemoveDirectoryIndex::List(vec![
            QueryFilter::Exact("index.html".to_string()),
            QueryFilter::Exact("index.php".to_string()),
        ]),
        ..Options::default()
    };
    assert_eq!(nopt("http://sindresorhus.com/index.html", opts), "http://sindresorhus.com");

    let opts2 = Options {
        remove_directory_index: RemoveDirectoryIndex::List(vec![
            QueryFilter::Exact("index.html".to_string()),
            QueryFilter::Exact("index.php".to_string()),
        ]),
        ..Options::default()
    };
    assert_eq!(nopt("http://sindresorhus.com/index.htm", opts2), "http://sindresorhus.com/index.htm");

    let opts3 = Options {
        remove_directory_index: RemoveDirectoryIndex::List(vec![
            QueryFilter::Exact("index.html".to_string()),
            QueryFilter::Exact("index.php".to_string()),
        ]),
        ..Options::default()
    };
    assert_eq!(nopt("http://sindresorhus.com/path/index.html", opts3), "http://sindresorhus.com/path");

    let opts4 = Options {
        remove_directory_index: RemoveDirectoryIndex::List(vec![
            QueryFilter::Exact("index.html".to_string()),
            QueryFilter::Exact("index.php".to_string()),
        ]),
        ..Options::default()
    };
    assert_eq!(nopt("http://sindresorhus.com/foo/bar/index.html", opts4), "http://sindresorhus.com/foo/bar");
}

#[test]
fn test_remove_directory_index_with_predicate() {
    let opts = || Options {
        remove_directory_index: RemoveDirectoryIndex::List(vec![
            QueryFilter::Predicate(Box::new(|s: &str| s.starts_with("default.") && s[8..].chars().all(|c| c.is_ascii_lowercase()))),
        ]),
        ..Options::default()
    };
    assert_eq!(nopt("www.sindresorhus.com/foo/default.php", opts()), "http://sindresorhus.com/foo");
}

// ============================================================
// sortQueryParameters option
// ============================================================

#[test]
fn test_sort_query_parameters() {
    assert_eq!(n("http://sindresorhus.com/?a=Z&b=Y&c=X&d=W"), "http://sindresorhus.com/?a=Z&b=Y&c=X&d=W");
    assert_eq!(n("http://sindresorhus.com/?b=Y&c=X&a=Z&d=W"), "http://sindresorhus.com/?a=Z&b=Y&c=X&d=W");
    assert_eq!(n("http://sindresorhus.com/?a=Z&d=W&b=Y&c=X"), "http://sindresorhus.com/?a=Z&b=Y&c=X&d=W");

    let opts = || Options { sort_query_parameters: false, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com/?a=Z&b=Y&c=X&d=W", opts()), "http://sindresorhus.com/?a=Z&b=Y&c=X&d=W");
    assert_eq!(nopt("http://sindresorhus.com/?b=Y&c=X&a=Z&d=W", opts()), "http://sindresorhus.com/?b=Y&c=X&a=Z&d=W");
}

// ============================================================
// invalid urls
// ============================================================

#[test]
fn test_invalid_urls() {
    assert!(normalize_url("http://", &Options::default()).is_err());
    assert!(normalize_url("/", &Options::default()).is_err());
    assert!(normalize_url("/relative/path/", &Options::default()).is_err());
}

// ============================================================
// remove duplicate pathname slashes
// ============================================================

#[test]
fn test_remove_duplicate_slashes() {
    assert_eq!(n("http://sindresorhus.com////foo/bar"), "http://sindresorhus.com/foo/bar");
    assert_eq!(n("http://sindresorhus.com////foo////bar"), "http://sindresorhus.com/foo/bar");
    assert_eq!(nopt("//sindresorhus.com//foo", Options { normalize_protocol: false, ..Options::default() }), "//sindresorhus.com/foo");
    assert_eq!(n("http://sindresorhus.com:5000///foo"), "http://sindresorhus.com:5000/foo");
    assert_eq!(n("http://sindresorhus.com///foo"), "http://sindresorhus.com/foo");
    assert_eq!(n("http://sindresorhus.com:5000//foo"), "http://sindresorhus.com:5000/foo");
    assert_eq!(n("http://sindresorhus.com//foo"), "http://sindresorhus.com/foo");
    // Embedded protocols preserved
    assert_eq!(n("http://sindresorhus.com/s3://sindresorhus.com"), "http://sindresorhus.com/s3://sindresorhus.com");
    assert_eq!(n("http://sindresorhus.com/s3://sindresorhus.com//foo"), "http://sindresorhus.com/s3://sindresorhus.com/foo");
    assert_eq!(n("http://sindresorhus.com//foo/s3://sindresorhus.com"), "http://sindresorhus.com/foo/s3://sindresorhus.com");
    assert_eq!(n("http://sindresorhus.com/git://sindresorhus.com"), "http://sindresorhus.com/git://sindresorhus.com");
    assert_eq!(n("http://sindresorhus.com/git://sindresorhus.com//foo"), "http://sindresorhus.com/git://sindresorhus.com/foo");
    assert_eq!(n("http://sindresorhus.com//foo/git://sindresorhus.com//foo"), "http://sindresorhus.com/foo/git://sindresorhus.com/foo");
    // Single-char "scheme" is NOT a protocol
    assert_eq!(n("http://sindresorhus.com/a://sindresorhus.com//foo"), "http://sindresorhus.com/a:/sindresorhus.com/foo");
    // Valid scheme characters
    assert_eq!(n("http://sindresorhus.com/a2-.+://sindresorhus.com"), "http://sindresorhus.com/a2-.+://sindresorhus.com");
    // Underscore is not valid in scheme
    assert_eq!(n("http://sindresorhus.com/a2-.+_://sindresorhus.com"), "http://sindresorhus.com/a2-.+_:/sindresorhus.com");
    // Doesn't start with letter
    assert_eq!(n("http://sindresorhus.com/2abc://sindresorhus.com"), "http://sindresorhus.com/2abc:/sindresorhus.com");
}

// ============================================================
// data URL
// ============================================================

#[test]
fn test_data_url() {
    // Invalid
    assert!(normalize_url("data:", &Options::default()).is_err());

    // Strip default MIME type
    assert_eq!(n("data:text/plain,foo"), "data:,foo");

    // Strip default charset
    assert_eq!(n("data:;charset=us-ascii,foo"), "data:,foo");

    // Empty MIME type
    assert_eq!(n("data:,"), "data:,");

    // Empty MIME type with charset
    assert_eq!(n("data:;charset=utf-8,foo"), "data:;charset=utf-8,foo");

    // Lowercase the MIME type
    assert_eq!(n("data:TEXT/HTML,foo"), "data:text/html,foo");

    // Lowercase the charset
    assert_eq!(n("data:;charset=UTF-8,foo"), "data:;charset=utf-8,foo");

    // Keep spaces when it's not base64
    assert_eq!(n("data:, foo #bar"), "data:, foo #bar");

    // Options don't affect data URLs (except stripHash)
    let opts = Options { strip_hash: true, ..Options::default() };
    assert_eq!(nopt("data:,foo#bar", opts), "data:,foo");
    assert_eq!(n("data:,sindresorhus.com/"), "data:,sindresorhus.com/");
    assert_eq!(n("data:,www.sindresorhus.com"), "data:,www.sindresorhus.com");
}

// ============================================================
// prevents homograph attack
// ============================================================

#[test]
fn test_homograph_prevention() {
    // Uses Unicode Cyrillic 'а' to look like 'ebay.com'
    assert_eq!(n("https://eb\u{0430}y.com"), "https://xn--eby-7cd.com");
}

// ============================================================
// ignore custom schemes
// ============================================================

#[test]
fn test_ignore_custom_schemes() {
    assert_eq!(n("tel:004346382763"), "tel:004346382763");
    assert_eq!(n("mailto:office@foo.com"), "mailto:office@foo.com");
    assert_eq!(n("sindre://www.sindresorhus.com"), "sindre://www.sindresorhus.com");
    assert_eq!(n("foo.bar://www.example.com"), "foo.bar://www.example.com");
    assert_eq!(n("foo:bar"), "foo:bar");

    // Opt-in via customProtocols
    let opts = Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() };
    assert_eq!(nopt("sindre://www.sindresorhus.com", opts), "sindre://sindresorhus.com");
}

// ============================================================
// customProtocols option
// ============================================================

#[test]
fn test_custom_protocols() {
    let opts = || Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() };

    assert_eq!(nopt("sindre://www.sorhus.com", opts()), "sindre://sorhus.com");
    assert_eq!(nopt("sindre://www.sorhus.com/", opts()), "sindre://sorhus.com");
    assert_eq!(nopt("sindre://www.sorhus.com/foo/bar", opts()), "sindre://sorhus.com/foo/bar");
    // Auth stripping
    assert_eq!(nopt("sindre://user:password@www.sorhus.com", opts()), "sindre://sorhus.com");
    // Trailing slash removal
    assert_eq!(nopt("sindre://sorhus.com/foo/", opts()), "sindre://sorhus.com/foo");
    // Query sorting
    assert_eq!(nopt("sindre://sorhus.com?b=two&a=one", opts()), "sindre://sorhus.com?a=one&b=two");
    // Hash handling
    assert_eq!(nopt("sindre://sorhus.com/foo#bar", opts()), "sindre://sorhus.com/foo#bar");
    assert_eq!(nopt("sindre://sorhus.com/foo#bar", Options { custom_protocols: vec!["sindre".to_string()], strip_hash: true, ..Options::default() }), "sindre://sorhus.com/foo");
    // UTM stripping
    assert_eq!(nopt("sindre://sorhus.com?foo=bar&utm_source=test", opts()), "sindre://sorhus.com?foo=bar");
    // Duplicate slashes
    assert_eq!(nopt("sindre://sorhus.com//foo//bar", opts()), "sindre://sorhus.com/foo/bar");
    // URI decoding
    assert_eq!(nopt("sindre://sorhus.com/%7Efoo/", opts()), "sindre://sorhus.com/~foo");
    // Empty customProtocols behaves like not providing it
    let opts_empty = Options { custom_protocols: vec![], ..Options::default() };
    assert_eq!(nopt("sindre://www.sorhus.com", opts_empty), "sindre://www.sorhus.com");
    // Unmatched protocols pass through
    assert_eq!(nopt("other://www.sorhus.com", opts()), "other://www.sorhus.com");

    // Multiple custom protocols
    let multi = || Options { custom_protocols: vec!["sindre".to_string(), "app".to_string()], ..Options::default() };
    assert_eq!(nopt("sindre://www.sorhus.com", multi()), "sindre://sorhus.com");
    assert_eq!(nopt("app://www.sorhus.com", multi()), "app://sorhus.com");
    assert_eq!(nopt("other://www.sorhus.com", multi()), "other://www.sorhus.com");

    // Dotted protocol names
    assert_eq!(nopt("foo.bar://www.example.com", Options { custom_protocols: vec!["foo.bar".to_string()], ..Options::default() }), "foo.bar://example.com");

    // ForceHttp/forceHttps don't affect custom protocols
    assert_eq!(nopt("sindre://sorhus.com", Options { custom_protocols: vec!["sindre".to_string()], force_http: true, ..Options::default() }), "sindre://sorhus.com");
    assert_eq!(nopt("sindre://sorhus.com", Options { custom_protocols: vec!["sindre".to_string()], force_https: true, ..Options::default() }), "sindre://sorhus.com");

    // StripProtocol doesn't affect custom protocols
    assert_eq!(nopt("sindre://sorhus.com", Options { custom_protocols: vec!["sindre".to_string()], strip_protocol: true, ..Options::default() }), "sindre://sorhus.com");

    // Port handling
    assert_eq!(nopt("sindre://sorhus.com:8080", opts()), "sindre://sorhus.com:8080");
    assert_eq!(nopt("sindre://sorhus.com:8080/foo", Options { custom_protocols: vec!["sindre".to_string()], remove_explicit_port: true, ..Options::default() }), "sindre://sorhus.com/foo");

    // Case-insensitive protocol matching
    assert_eq!(nopt("sindre://www.sorhus.com", Options { custom_protocols: vec!["SINDRE".to_string()], ..Options::default() }), "sindre://sorhus.com");
    assert_eq!(nopt("sindre://www.sorhus.com", Options { custom_protocols: vec!["Sindre".to_string()], ..Options::default() }), "sindre://sorhus.com");
    assert_eq!(nopt("sindre://www.sorhus.com", Options { custom_protocols: vec!["sindre:".to_string()], ..Options::default() }), "sindre://sorhus.com");
    assert_eq!(nopt("sindre://www.sorhus.com", Options { custom_protocols: vec![" sindre ".to_string()], ..Options::default() }), "sindre://sorhus.com");

    // Path traversal
    assert_eq!(nopt("sindre://sorhus.com/foo/../bar", opts()), "sindre://sorhus.com/bar");
    assert_eq!(nopt("sindre://sorhus.com/foo/./bar", opts()), "sindre://sorhus.com/foo/bar");

    // Auth stripping with stripAuthentication: false
    assert_eq!(nopt("sindre://user:password@www.sorhus.com", Options { custom_protocols: vec!["sindre".to_string()], strip_authentication: false, ..Options::default() }), "sindre://user:password@sorhus.com");

    // stripWWW: false
    assert_eq!(nopt("sindre://www.sorhus.com", Options { custom_protocols: vec!["sindre".to_string()], strip_www: false, ..Options::default() }), "sindre://www.sorhus.com");

    // removeTrailingSlash: false
    assert_eq!(nopt("sindre://sorhus.com/foo/", Options { custom_protocols: vec!["sindre".to_string()], remove_trailing_slash: false, ..Options::default() }), "sindre://sorhus.com/foo/");

    // removeQueryParameters: true
    assert_eq!(nopt("sindre://sorhus.com?foo=bar", Options { custom_protocols: vec!["sindre".to_string()], remove_query_parameters: RemoveQueryParameters::All, ..Options::default() }), "sindre://sorhus.com");

    // Built-in protocols still work
    assert_eq!(nopt("http://www.sorhus.com", opts()), "http://sorhus.com");
    assert_eq!(nopt("https://www.sorhus.com", opts()), "https://sorhus.com");
}

// ============================================================
// removePath option
// ============================================================

#[test]
fn test_remove_path() {
    let opts = || Options { remove_path: true, ..Options::default() };
    assert_eq!(nopt("https://example.com/path/to/page", opts()), "https://example.com");
    assert_eq!(nopt("https://example.com/path/to/page?query=1", opts()), "https://example.com/?query=1");
    assert_eq!(nopt("https://example.com/path/to/page#hash", opts()), "https://example.com/#hash");
    assert_eq!(nopt("https://example.com/", opts()), "https://example.com");
    assert_eq!(nopt("https://example.com", opts()), "https://example.com");

    assert_eq!(nopt("https://www.example.com/path", Options { remove_path: true, strip_www: true, ..Options::default() }), "https://example.com");
}

// ============================================================
// transformPath option
// ============================================================

#[test]
fn test_transform_path() {
    // Keep only first component
    let opts = || Options { transform_path: Some(Box::new(|c: Vec<String>| c.into_iter().take(1).collect())), ..Options::default() };
    assert_eq!(nopt("https://example.com/api/v1/users", opts()), "https://example.com/api");
    assert_eq!(nopt("https://example.com/path/to/page", opts()), "https://example.com/path");
    assert_eq!(nopt("https://example.com/", opts()), "https://example.com");

    // Remove specific component
    let opts2 = || Options { transform_path: Some(Box::new(|c: Vec<String>| c.into_iter().filter(|s| s != "admin").collect())), ..Options::default() };
    assert_eq!(nopt("https://example.com/admin/users", opts2()), "https://example.com/users");
    assert_eq!(nopt("https://example.com/path/admin/page", opts2()), "https://example.com/path/page");

    // Empty result
    let opts3 = || Options { transform_path: Some(Box::new(|_| vec![])), ..Options::default() };
    assert_eq!(nopt("https://example.com/path", opts3()), "https://example.com");
}

// ============================================================
// emptyQueryValue option
// ============================================================

#[test]
fn test_empty_query_value_preserve() {
    assert_eq!(n("https://example.com?key"), "https://example.com/?key");
    assert_eq!(n("https://example.com?key="), "https://example.com/?key=");
    assert_eq!(n("https://example.com?a&b=&c=1"), "https://example.com/?a&b=&c=1");
}

#[test]
fn test_empty_query_value_always() {
    let opts = || Options { empty_query_value: EmptyQueryValue::Always, ..Options::default() };
    assert_eq!(nopt("https://example.com?key", opts()), "https://example.com/?key=");
    assert_eq!(nopt("https://example.com?key=", opts()), "https://example.com/?key=");
    assert_eq!(nopt("https://example.com?a&b=&c=1", opts()), "https://example.com/?a=&b=&c=1");
    assert_eq!(nopt("https://example.com?foo&bar&baz=value", opts()), "https://example.com/?bar=&baz=value&foo=");
}

#[test]
fn test_empty_query_value_never() {
    let opts = || Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() };
    assert_eq!(nopt("https://example.com?key", opts()), "https://example.com/?key");
    assert_eq!(nopt("https://example.com?key=", opts()), "https://example.com/?key");
    assert_eq!(nopt("https://example.com?a&b=&c=1", opts()), "https://example.com/?a&b&c=1");
    assert_eq!(nopt("https://example.com?foo=&bar=&baz=value", opts()), "https://example.com/?bar&baz=value&foo");
}

#[test]
fn test_empty_query_value_with_sort_disabled() {
    assert_eq!(nopt("https://example.com?b&a=", Options { empty_query_value: EmptyQueryValue::Always, sort_query_parameters: false, ..Options::default() }), "https://example.com/?b=&a=");
    assert_eq!(nopt("https://example.com?b=&a", Options { empty_query_value: EmptyQueryValue::Never, sort_query_parameters: false, ..Options::default() }), "https://example.com/?b&a");
}

#[test]
fn test_empty_query_value_never_preserves_values_with_equals() {
    let opts = || Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() };
    assert_eq!(nopt("https://example.com?key==", opts()), "https://example.com/?key==");
    assert_eq!(nopt("https://example.com?key=value=", opts()), "https://example.com/?key=value=");
}

#[test]
fn test_empty_query_value_duplicate_keys() {
    assert_eq!(n("https://example.com?a&a="), "https://example.com/?a&a");
    assert_eq!(n("https://example.com?a=&a"), "https://example.com/?a&a");
    assert_eq!(n("https://example.com?a&a&a="), "https://example.com/?a&a&a");
    assert_eq!(n("https://example.com?a=&a=&a"), "https://example.com/?a&a&a");
}

#[test]
fn test_empty_query_value_plus_in_keys() {
    assert_eq!(n("https://example.com?foo+bar"), "https://example.com/?foo%20bar");
    assert_eq!(n("https://example.com?foo+bar="), "https://example.com/?foo%20bar=");
    assert_eq!(nopt("https://example.com?foo+bar=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?foo%20bar");
    assert_eq!(n("https://example.com?foo+bar=value"), "https://example.com/?foo%20bar=value");
}

#[test]
fn test_empty_query_value_multiple_equals() {
    assert_eq!(n("https://example.com?key=a=b=c"), "https://example.com/?key=a=b=c");
    assert_eq!(n("https://example.com?data=abc=="), "https://example.com/?data=abc==");
}

#[test]
fn test_single_param_edge_cases() {
    assert_eq!(n("https://example.com?=value"), "https://example.com/?=value");
    assert_eq!(n("https://example.com?="), "https://example.com/?=");
    assert_eq!(nopt("https://example.com?=&a=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?=&a");
    assert_eq!(nopt("https://example.com?=&=", Options { empty_query_value: EmptyQueryValue::Always, ..Options::default() }), "https://example.com/?=&=");
    assert_eq!(nopt("https://example.com?=&=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?=&=");
    assert_eq!(nopt("https://example.com?=&a", Options { empty_query_value: EmptyQueryValue::Always, ..Options::default() }), "https://example.com/?=&a=");
    assert_eq!(nopt("https://example.com?&", Options { empty_query_value: EmptyQueryValue::Always, ..Options::default() }), "https://example.com");
    assert_eq!(nopt("https://example.com?&", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com");
}

#[test]
fn test_empty_segments_removed() {
    let opts = || Options { sort_query_parameters: false, ..Options::default() };
    assert_eq!(nopt("https://example.com?foo&&bar", opts()), "https://example.com/?foo&bar");
    assert_eq!(nopt("https://example.com?foo&&bar", Options { sort_query_parameters: false, empty_query_value: EmptyQueryValue::Always, ..Options::default() }), "https://example.com/?foo=&bar=");
    assert_eq!(nopt("https://example.com?foo&&bar", Options { sort_query_parameters: false, empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?foo&bar");
    assert_eq!(nopt("https://example.com?&&", opts()), "https://example.com");
}

#[test]
fn test_hash_does_not_affect_query() {
    assert_eq!(n("https://example.com?key#hash"), "https://example.com/?key#hash");
    assert_eq!(n("https://example.com?key=#hash"), "https://example.com/?key=#hash");
}

// ============================================================
// sortQueryParameters with encoded reserved characters
// ============================================================

#[test]
fn test_encoded_reserved_chars_preserved() {
    assert_eq!(n("https://example.com/?token=a%2Fb%2Fc"), "https://example.com/?token=a%2Fb%2Fc");
    assert_eq!(n("https://example.com/?token=a%2fb%2fc"), "https://example.com/?token=a%2Fb%2Fc");
}

#[test]
fn test_encoded_reserved_survive_sort() {
    assert_eq!(n("https://example.com/?z=1&token=a%2Fb"), "https://example.com/?token=a%2Fb&z=1");
}

#[test]
fn test_encoded_reserved_in_keys() {
    assert_eq!(n("https://example.com/?foo%3Abar=1&a=2"), "https://example.com/?a=2&foo%3Abar=1");
    assert_eq!(n("https://example.com/?b%26c=1&a=2"), "https://example.com/?a=2&b%26c=1");
}

#[test]
fn test_encoded_reserved_sort_order() {
    // / (0x2F=47) < : (0x3A=58)
    assert_eq!(n("https://example.com/?%3A=1&%2F=2"), "https://example.com/?%2F=2&%3A=1");
    // : (58) < [ (0x5B=91)
    assert_eq!(n("https://example.com/?%5B=1&%3A=2"), "https://example.com/?%3A=2&%5B=1");
}

#[test]
fn test_encoded_reserved_in_keys_and_values() {
    assert_eq!(n("https://example.com/?z%3A=val%2F&a%2F=val%3A"), "https://example.com/?a%2F=val%3A&z%3A=val%2F");
}

#[test]
fn test_multiple_encoded_reserved_in_value() {
    assert_eq!(n("https://example.com/?q=%3A%2F%3F"), "https://example.com/?q=%3A%2F%3F");
}

#[test]
fn test_encoded_reserved_with_remove_params() {
    assert_eq!(n("https://example.com/?utm_source=test&token=a%2Fb"), "https://example.com/?token=a%2Fb");
}

#[test]
fn test_encoded_reserved_without_sort() {
    let opts = || Options { sort_query_parameters: false, ..Options::default() };
    assert_eq!(nopt("https://example.com/?token=a%2Fb", opts()), "https://example.com/?token=a%2Fb");
}

#[test]
fn test_all_encoded_reserved_chars() {
    let chars = &["%3A", "%2F", "%3F", "%23", "%5B", "%5D", "%40", "%21", "%24", "%26", "%27", "%28", "%29", "%2A", "%2B", "%2C", "%3B", "%3D"];
    for enc in chars {
        let url = format!("https://example.com/?value={enc}");
        assert_eq!(n(&url), format!("https://example.com/?value={enc}"), "Failed for {enc}");
    }
}

// ============================================================
// path-like query strings
// ============================================================

#[test]
fn test_path_like_query_preserved() {
    assert_eq!(n("https://example.com/index.php?/Some/Route/To/Path/12345"), "https://example.com/index.php?/Some/Route/To/Path/12345");
    assert_eq!(n("https://example.com/script.php?/api/v1/users/123"), "https://example.com/script.php?/api/v1/users/123");
}

#[test]
fn test_params_without_values_no_equals() {
    assert_eq!(n("https://example.com/index.php?key"), "https://example.com/index.php?key");
    assert_eq!(n("https://example.com/index.php?a&b&c"), "https://example.com/index.php?a&b&c");
}

#[test]
fn test_params_with_empty_values_keep_equals() {
    assert_eq!(n("https://example.com/index.php?key="), "https://example.com/index.php?key=");
    assert_eq!(n("https://example.com/index.php?key=&another="), "https://example.com/index.php?another=&key=");
}

// ============================================================
// Additional edge cases
// ============================================================

#[test]
fn test_all_params_removed_clears_query() {
    assert_eq!(n("https://example.com?utm_source=test&utm_medium=web"), "https://example.com");
    assert_eq!(nopt("https://example.com?key", Options { remove_query_parameters: RemoveQueryParameters::All, ..Options::default() }), "https://example.com");
}

#[test]
fn test_deep_path_normalization() {
    assert_eq!(n("http://example.com/a/b/c/../d/./e"), "http://example.com/a/b/d/e");
}

#[test]
fn test_uppercase_scheme() {
    assert_eq!(n("HTTP://example.com"), "http://example.com");
    assert_eq!(n("HTTPS://example.com"), "https://example.com");
}

// ============================================================
// Additional removeDirectoryIndex tests
// ============================================================

#[test]
fn test_remove_directory_index_string_filter() {
    let opts = || Options {
        remove_directory_index: RemoveDirectoryIndex::List(vec![
            QueryFilter::Exact("index.html".to_string()),
            QueryFilter::Exact("index.php".to_string()),
        ]),
        ..Options::default()
    };
    assert_eq!(nopt("http://sindresorhus.com/index.php", opts()), "http://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com/path/index.php", opts()), "http://sindresorhus.com/path");
    assert_eq!(nopt("http://sindresorhus.com/path/index.htm", opts()), "http://sindresorhus.com/path/index.htm");
}

#[test]
fn test_remove_directory_index_fr() {
    let opts = || Options {
        remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Exact("fr".to_string())]),
        ..Options::default()
    };
    assert_eq!(nopt("http://example.com/fr", opts()), "http://example.com");
    assert_eq!(nopt("http://example.com/path/fr", opts()), "http://example.com/path");
}

#[test]
fn test_remove_trailing_slash_and_directory_index() {
    let opts = || Options { remove_trailing_slash: true, remove_directory_index: RemoveDirectoryIndex::Default, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com/path/", opts()), "http://sindresorhus.com/path");
    assert_eq!(nopt("http://sindresorhus.com/path/index.html", opts()), "http://sindresorhus.com/path");
    // Hash with trailing slash preserved
    assert_eq!(nopt("http://sindresorhus.com/#/path/", opts()), "http://sindresorhus.com/#/path/");

    let opts2 = || Options { remove_trailing_slash: false, remove_directory_index: RemoveDirectoryIndex::Default, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com/path/", opts2()), "http://sindresorhus.com/path/");
}

// ============================================================
// Encoded backslashes
// ============================================================

#[test]
fn test_encoded_backslash_preserved() {
    assert_eq!(n("https://foo.com/something%5Celse/great"), "https://foo.com/something%5Celse/great");
}

// ============================================================
// sortQueryParameters idempotency
// ============================================================

#[test]
fn test_sort_idempotency() {
    let sorted = Options { sort_query_parameters: true, ..Options::default() };
    let unsorted = Options { sort_query_parameters: false, ..Options::default() };
    assert_eq!(
        nopt("http://sindresorhus.com/?a=/path", sorted),
        nopt("http://sindresorhus.com/?a=/path", unsorted)
    );
}

// ============================================================
// More emptyQueryValue edge cases
// ============================================================

#[test]
fn test_empty_query_value_preserve_with_utm() {
    assert_eq!(n("https://example.com?key&utm_source=test"), "https://example.com/?key");
    let opts = Options { sort_query_parameters: false, ..Options::default() };
    assert_eq!(nopt("https://example.com?key&utm_source=test", opts), "https://example.com/?key");
}

#[test]
fn test_empty_query_value_preserve_with_spaces() {
    assert_eq!(n("https://example.com?foo%20bar"), "https://example.com/?foo%20bar");
    assert_eq!(n("https://example.com?foo%20bar="), "https://example.com/?foo%20bar=");
    assert_eq!(nopt("https://example.com?foo%20bar=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?foo%20bar");
}

#[test]
fn test_empty_query_value_unsorted_preserve() {
    let opts = || Options { sort_query_parameters: false, ..Options::default() };
    assert_eq!(nopt("https://example.com?a&a=", opts()), "https://example.com/?a&a");
    assert_eq!(nopt("https://example.com?a=&a", opts()), "https://example.com/?a&a");
}

#[test]
fn test_empty_query_value_plus_unsorted() {
    let opts = || Options { sort_query_parameters: false, ..Options::default() };
    assert_eq!(nopt("https://example.com?foo+bar", opts()), "https://example.com/?foo%20bar");
    assert_eq!(nopt("https://example.com?foo+bar=", opts()), "https://example.com/?foo%20bar=");
    assert_eq!(nopt("https://example.com?foo+bar&baz+qux=", opts()), "https://example.com/?foo%20bar&baz%20qux=");
}

// ============================================================
// %FAIL% passthrough
// ============================================================

#[test]
fn test_invalid_percent_encoding_passthrough() {
    assert_eq!(n("https://foo.com/%FAIL%/07/94/ca/55.jpg"), "https://foo.com/%FAIL%/07/94/ca/55.jpg");
}

// ============================================================
// Encoded reserved chars: removeQueryParameters interaction
// ============================================================

#[test]
fn test_encoded_reserved_with_keep_params() {
    let opts = Options {
        remove_query_parameters: RemoveQueryParameters::None,
        keep_query_parameters: Some(vec![QueryFilter::Exact("foo:bar".to_string())]),
        ..Options::default()
    };
    assert_eq!(nopt("https://example.com/?foo%3Abar=1&baz=2", opts), "https://example.com/?foo%3Abar=1");
}

#[test]
fn test_encoded_reserved_with_remove_specific() {
    let opts = Options {
        remove_query_parameters: RemoveQueryParameters::List(vec![QueryFilter::Exact("foo:bar".to_string())]),
        ..Options::default()
    };
    assert_eq!(nopt("https://example.com/?foo%3Abar=1&baz=2", opts), "https://example.com/?baz=2");
}

#[test]
fn test_backslash_conversion() {
    assert_eq!(n("https://foo.com/something\\else/great"), "https://foo.com/something/else/great");
}

// ============================================================
// Space-to-plus in query values (JS URLSearchParams behavior)
// ============================================================

#[test]
fn test_space_to_plus_in_query_value() {
    assert_eq!(n("sindresorhus.com/?foo=bar baz"), "http://sindresorhus.com/?foo=bar+baz");
}

// ============================================================
// Special character encoding in query values
// ============================================================

#[test]
fn test_special_char_encoding_in_query() {
    assert_eq!(n("http://sindresorhus.com/?foo=bar*|<>:\""), "http://sindresorhus.com/?foo=bar*|%3C%3E:%22");
}

// ============================================================
// Unicode query keys
// ============================================================

#[test]
fn test_unicode_query_keys() {
    assert_eq!(n("https://example.com?café"), "https://example.com/?caf%C3%A9");
    assert_eq!(n("https://example.com?café="), "https://example.com/?caf%C3%A9=");
    let opts = Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() };
    assert_eq!(nopt("https://example.com?café=", opts), "https://example.com/?caf%C3%A9");
}

// ============================================================
// Encoded delimiters in keys
// ============================================================

#[test]
fn test_encoded_delimiters_in_keys() {
    let never = || Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() };
    assert_eq!(nopt("https://example.com?foo%26bar=", never()), "https://example.com/?foo%26bar");
    assert_eq!(nopt("https://example.com?foo%3Dbar=", never()), "https://example.com/?foo%3Dbar");
    assert_eq!(n("https://example.com?foo%26bar&utm_source=test"), "https://example.com/?foo%26bar");
    assert_eq!(n("https://example.com?foo%2526bar="), "https://example.com/?foo%2526bar=");
    assert_eq!(nopt("https://example.com?foo%2526bar=", never()), "https://example.com/?foo%2526bar");
}

// ============================================================
// Multiple keys with mixed formats
// ============================================================

#[test]
fn test_multiple_keys_mixed_formats() {
    assert_eq!(n("https://example.com?a&b=&c=1"), "https://example.com/?a&b=&c=1");
    assert_eq!(n("https://example.com?foo%20bar&baz"), "https://example.com/?baz&foo%20bar");
}

// ============================================================
// Duplicate keys with sorting - mixed formats normalize
// ============================================================

#[test]
fn test_duplicate_keys_sorted_mixed() {
    assert_eq!(n("https://example.com?a&a="), "https://example.com/?a&a");
    assert_eq!(n("https://example.com?a=&a"), "https://example.com/?a&a");
    assert_eq!(n("https://example.com?a&a&a="), "https://example.com/?a&a&a");
    assert_eq!(n("https://example.com?a=&a=&a"), "https://example.com/?a&a&a");
}

// ============================================================
// Multiple = in values (only first = is delimiter)
// ============================================================

#[test]
fn test_multiple_equals_in_values() {
    assert_eq!(n("https://example.com?key=a=b=c"), "https://example.com/?key=a=b=c");
    assert_eq!(n("https://example.com?data=abc=="), "https://example.com/?data=abc==");
}

// ============================================================
// Encoded = (%3D) in values preserved
// ============================================================

#[test]
fn test_encoded_equals_in_values() {
    assert_eq!(n("https://example.com?key=val%3Due"), "https://example.com/?key=val%3Due");
    assert_eq!(n("https://example.com?key=%3D"), "https://example.com/?key=%3D");
    let unsorted = Options { sort_query_parameters: false, ..Options::default() };
    assert_eq!(nopt("https://example.com?key=val%3Due", unsorted), "https://example.com/?key=val%3Due");
}

// ============================================================
// ?& and ?&& produce no query string
// ============================================================

#[test]
fn test_empty_ampersand_query() {
    assert_eq!(n("https://example.com?&"), "https://example.com");
    let never = Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() };
    assert_eq!(nopt("https://example.com?&", never), "https://example.com");
    let unsorted = Options { sort_query_parameters: false, ..Options::default() };
    assert_eq!(nopt("https://example.com?&&", unsorted), "https://example.com");
}

// ============================================================
// Empty segments removed even with sort disabled
// ============================================================

#[test]
fn test_empty_segments_unsorted() {
    let unsorted = || Options { sort_query_parameters: false, ..Options::default() };
    assert_eq!(nopt("https://example.com?foo&&bar", unsorted()), "https://example.com/?foo&bar");
    let always_unsorted = || Options { sort_query_parameters: false, empty_query_value: EmptyQueryValue::Always, ..Options::default() };
    assert_eq!(nopt("https://example.com?foo&&bar", always_unsorted()), "https://example.com/?foo=&bar=");
    let never_unsorted = || Options { sort_query_parameters: false, empty_query_value: EmptyQueryValue::Never, ..Options::default() };
    assert_eq!(nopt("https://example.com?foo&&bar", never_unsorted()), "https://example.com/?foo&bar");
}

// ============================================================
// Encoded backslash in path stays encoded
// ============================================================

#[test]
fn test_encoded_backslash_all() {
    assert_eq!(n("https://foo.com/something%5Celse/great"), "https://foo.com/something%5Celse/great");
    assert_eq!(n("https://foo.com/something\\else/great"), "https://foo.com/something/else/great");
    assert_eq!(n("https://foo.com/something\\else%5Cgreat"), "https://foo.com/something/else%5Cgreat");
}

// ============================================================  
// Additional defaultProtocol tests (legacy : suffix)
// ============================================================

#[test]
fn test_default_protocol_legacy_colon() {
    let https = Options { default_protocol: Protocol::Https, ..Options::default() };
    assert_eq!(nopt("sindresorhus.com", https), "https://sindresorhus.com");
}

// ============================================================
// Additional removeQueryParameters tests
// ============================================================

#[test]
fn test_remove_query_parameters_regex_like() {
    // Test removal with predicate that acts like regex
    let opts = Options {
        remove_query_parameters: RemoveQueryParameters::List(vec![
            QueryFilter::Predicate(Box::new(|key| key.starts_with("utm_"))),
            QueryFilter::Exact("ref".to_string()),
        ]),
        ..Options::default()
    };
    assert_eq!(nopt("http://sindresorhus.com/?a=b&utm_source=google&ref=home&c=d", opts),
        "http://sindresorhus.com/?a=b&c=d");
}

#[test]
fn test_remove_query_parameters_false() {
    // removeQueryParameters: None means don't remove any (not even utm_*)
    let opts = Options {
        remove_query_parameters: RemoveQueryParameters::None,
        ..Options::default()
    };
    assert_eq!(nopt("http://sindresorhus.com/?utm_source=google&a=b", opts),
        "http://sindresorhus.com/?a=b&utm_source=google");
}

// ============================================================
// Additional keepQueryParameters tests
// ============================================================

#[test]
fn test_keep_query_parameters_extended() {
    let opts = || Options {
        remove_query_parameters: RemoveQueryParameters::None,
        keep_query_parameters: Some(vec![QueryFilter::Exact("a".to_string()), QueryFilter::Exact("b".to_string())]),
        ..Options::default()
    };
    assert_eq!(nopt("http://sindresorhus.com/?a=1&b=2&c=3", opts()), "http://sindresorhus.com/?a=1&b=2");
    assert_eq!(nopt("http://sindresorhus.com/?a=1&b=2&c=3&d=4", opts()), "http://sindresorhus.com/?a=1&b=2");
}

#[test]
fn test_keep_query_parameters_with_predicate() {
    let opts = Options {
        remove_query_parameters: RemoveQueryParameters::None,
        keep_query_parameters: Some(vec![
            QueryFilter::Predicate(Box::new(|key| key.starts_with("a"))),
        ]),
        ..Options::default()
    };
    assert_eq!(nopt("http://sindresorhus.com/?ab=1&ac=2&bc=3", opts), "http://sindresorhus.com/?ab=1&ac=2");
}

// ============================================================
// Additional stripTextFragment tests
// ============================================================

#[test]
fn test_strip_text_fragment_extended() {
    assert_eq!(n("http://sindresorhus.com/foo#bar:~:text=hello%20world"), "http://sindresorhus.com/foo#bar");
    assert_eq!(n("http://example.com/#:~:text=hello"), "http://example.com");
}

// ============================================================
// More removeDirectoryIndex tests
// ============================================================

#[test]
fn test_remove_directory_index_extended() {
    let default = || Options { remove_directory_index: RemoveDirectoryIndex::Default, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com/index.html", default()), "http://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com/index.htm", default()), "http://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com/index.php", default()), "http://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com/index.asp", default()), "http://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com/index.aspx", default()), "http://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com/index.cgi", default()), "http://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com/index.jsp", default()), "http://sindresorhus.com");
    assert_eq!(nopt("http://sindresorhus.com/path/index.html", default()), "http://sindresorhus.com/path");
    assert_eq!(nopt("http://sindresorhus.com/path/index.htm", default()), "http://sindresorhus.com/path");
}

// ============================================================
// removeTrailingSlash + removeDirectoryIndex combined
// ============================================================

#[test]
fn test_trailing_slash_directory_index_combined() {
    let opts = || Options {
        remove_trailing_slash: true,
        remove_directory_index: RemoveDirectoryIndex::Default,
        ..Options::default()
    };
    assert_eq!(nopt("http://sindresorhus.com/path/index.html", opts()), "http://sindresorhus.com/path");
    assert_eq!(nopt("http://sindresorhus.com/path/", opts()), "http://sindresorhus.com/path");
    assert_eq!(nopt("http://sindresorhus.com/", opts()), "http://sindresorhus.com");
}

// ============================================================
// Additional sortQueryParameters tests
// ============================================================

#[test]
fn test_sort_query_parameters_extended() {
    assert_eq!(n("http://sindresorhus.com/?a=a&b=b"), "http://sindresorhus.com/?a=a&b=b");
    assert_eq!(n("http://sindresorhus.com/?b=b&a=a"), "http://sindresorhus.com/?a=a&b=b");
    let unsorted = Options { sort_query_parameters: false, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com/?b=b&a=a", unsorted), "http://sindresorhus.com/?b=b&a=a");
}

// ============================================================
// Additional customProtocols tests
// ============================================================

#[test]
fn test_custom_protocols_extended() {
    // Custom protocol with port
    let opts = || Options { custom_protocols: vec!["myproto".to_string()], ..Options::default() };
    assert_eq!(nopt("myproto://example.com:8080/path", opts()), "myproto://example.com:8080/path");
    // Custom protocol with query
    assert_eq!(nopt("myproto://example.com/path?a=1&b=2", opts()), "myproto://example.com/path?a=1&b=2");
    // Custom protocol case insensitive
    assert_eq!(nopt("MYPROTO://example.com", opts()), "myproto://example.com");
    // Custom protocol with hash
    assert_eq!(nopt("myproto://example.com#hash", opts()), "myproto://example.com#hash");
    assert_eq!(nopt("myproto://example.com#hash", Options { custom_protocols: vec!["myproto".to_string()], strip_hash: true, ..Options::default() }), "myproto://example.com");
}

// ============================================================
// Additional transformPath tests
// ============================================================

#[test]
fn test_transform_path_extended() {
    // transformPath returning empty vec
    let opts = Options {
        transform_path: Some(Box::new(|_parts| vec![])),
        ..Options::default()
    };
    assert_eq!(nopt("http://sindresorhus.com/foo/bar", opts), "http://sindresorhus.com");

    // transformPath filtering
    let opts = Options {
        transform_path: Some(Box::new(|parts| parts.into_iter().filter(|p| p != "baz").collect())),
        ..Options::default()
    };
    assert_eq!(nopt("http://sindresorhus.com/foo/baz/bar", opts), "http://sindresorhus.com/foo/bar");

    // transformPath with trailing slash preserved in hash
    let opts = Options {
        transform_path: Some(Box::new(|parts| parts)),
        ..Options::default()
    };
    assert_eq!(nopt("http://sindresorhus.com/foo/bar", opts), "http://sindresorhus.com/foo/bar");
}

// ============================================================
// Additional removeTrailingSlash tests
// ============================================================

#[test]
fn test_remove_trailing_slash_extended() {
    assert_eq!(n("http://sindresorhus.com/foo/bar/"), "http://sindresorhus.com/foo/bar");
    assert_eq!(n("http://sindresorhus.com/foo/bar/baz/"), "http://sindresorhus.com/foo/bar/baz");
    // Trailing slash not removed with hash
    let opts_no_trail = Options { remove_trailing_slash: true, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com/#/path/", opts_no_trail), "http://sindresorhus.com/#/path/");
}

// ============================================================
// Additional path-like query strings
// ============================================================

#[test]
fn test_path_like_query_strings_extended() {
    assert_eq!(n("https://example.com?key"), "https://example.com/?key");
    assert_eq!(n("https://example.com?key="), "https://example.com/?key=");
    assert_eq!(n("https://example.com?key&key2"), "https://example.com/?key&key2");
    assert_eq!(n("https://example.com?key=&key2"), "https://example.com/?key=&key2");
    assert_eq!(n("https://example.com?key&key2="), "https://example.com/?key&key2=");
    assert_eq!(n("https://example.com?key=value&key2"), "https://example.com/?key=value&key2");
    assert_eq!(n("https://example.com?key&key2=value"), "https://example.com/?key&key2=value");
    assert_eq!(n("https://example.com?a=1&b"), "https://example.com/?a=1&b");
    assert_eq!(n("https://example.com?b&a=1"), "https://example.com/?a=1&b");
    assert_eq!(n("https://example.com?key&utm_source=test"), "https://example.com/?key");
}

// ============================================================
// Additional data URL tests
// ============================================================

#[test]
fn test_data_url_extended() {
    // Uppercase MIME
    assert_eq!(n("data:TEXT/HTML,hello"), "data:text/html,hello");
    // Empty data URL
    assert_eq!(n("data:,"), "data:,");
    // Base64 with text/plain - text/plain is kept because base64 attribute is present
    assert_eq!(n("data:text/plain;base64,SGVsbG8="), "data:text/plain;base64,SGVsbG8=");
    // Strip hash on data URL
    assert_eq!(nopt("data:text/html,hello#world", Options { strip_hash: true, ..Options::default() }), "data:text/html,hello");
}

// ============================================================
// Additional encoded reserved in query values tests
// ============================================================

#[test]
fn test_encoded_reserved_in_values_extended() {
    assert_eq!(n("https://example.com?foo=bar%2Fbaz"), "https://example.com/?foo=bar%2Fbaz");
    assert_eq!(n("https://example.com?foo=bar%3Abaz"), "https://example.com/?foo=bar%3Abaz");
    assert_eq!(n("https://example.com?foo=bar%23baz"), "https://example.com/?foo=bar%23baz");
    assert_eq!(n("https://example.com?foo=bar%3Fbaz"), "https://example.com/?foo=bar%3Fbaz");
    assert_eq!(n("https://example.com?foo=bar%40baz"), "https://example.com/?foo=bar%40baz");
}

// ============================================================
// Additional emptyQueryValue tests
// ============================================================

#[test]
fn test_empty_query_value_empty_key_edge_cases() {
    // =&a with always
    let always = || Options { empty_query_value: EmptyQueryValue::Always, ..Options::default() };
    assert_eq!(nopt("https://example.com?=&a", always()), "https://example.com/?=&a=");
    // =&= with always
    assert_eq!(nopt("https://example.com?=&=", always()), "https://example.com/?=&=");
    // =&= with never
    let never = || Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() };
    assert_eq!(nopt("https://example.com?=&=", never()), "https://example.com/?=&=");
    // =&a with never 
    assert_eq!(nopt("https://example.com?=&a=", never()), "https://example.com/?=&a");
}

// ============================================================
// Vimeo CDN URL with query containing full URLs
// ============================================================

#[test]
fn test_vimeo_cdn_url() {
    assert_eq!(n("https://i.vimeocdn.com/filter/overlay?src0=https://i.vimeocdn.com/video/598160082_1280x720.jpg&src1=https://f.vimeocdn.com/images_v6/share/play_icon_overlay.png"),
        "https://i.vimeocdn.com/filter/overlay?src0=https://i.vimeocdn.com/video/598160082_1280x720.jpg&src1=https://f.vimeocdn.com/images_v6/share/play_icon_overlay.png");
}

// ============================================================
// %2B (encoded +) in query keys - should stay as %2B
// ============================================================

#[test]
fn test_encoded_plus_in_keys() {
    assert_eq!(n("https://example.com?foo%2Bbar=1"), "https://example.com/?foo%2Bbar=1");
    assert_eq!(n("https://example.com?foo%2Bbar="), "https://example.com/?foo%2Bbar=");
    let never = Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() };
    assert_eq!(nopt("https://example.com?foo%2Bbar=", never), "https://example.com/?foo%2Bbar");
}

// ============================================================
// More emptyQueryValue always/never with sort
// ============================================================

#[test]
fn test_empty_query_always_with_sort() {
    let always = || Options { empty_query_value: EmptyQueryValue::Always, ..Options::default() };
    assert_eq!(nopt("https://example.com?key", always()), "https://example.com/?key=");
    assert_eq!(nopt("https://example.com?key=", always()), "https://example.com/?key=");
    assert_eq!(nopt("https://example.com?foo&bar&baz=value", always()), "https://example.com/?bar=&baz=value&foo=");
}

#[test]
fn test_empty_query_never_with_sort() {
    let never = || Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() };
    assert_eq!(nopt("https://example.com?key", never()), "https://example.com/?key");
    assert_eq!(nopt("https://example.com?key=", never()), "https://example.com/?key");
    assert_eq!(nopt("https://example.com?a&b=&c=1", never()), "https://example.com/?a&b&c=1");
    assert_eq!(nopt("https://example.com?foo=&bar=&baz=value", never()), "https://example.com/?bar&baz=value&foo");
}

#[test]
fn test_empty_query_always_nosort() {
    let opts = Options { empty_query_value: EmptyQueryValue::Always, sort_query_parameters: false, ..Options::default() };
    assert_eq!(nopt("https://example.com?b&a=", opts), "https://example.com/?b=&a=");
}

#[test]
fn test_empty_query_never_nosort() {
    let opts = Options { empty_query_value: EmptyQueryValue::Never, sort_query_parameters: false, ..Options::default() };
    assert_eq!(nopt("https://example.com?b=&a", opts), "https://example.com/?b&a");
}

#[test]
fn test_empty_query_never_with_multi_equals() {
    let never = || Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() };
    assert_eq!(nopt("https://example.com?key==", never()), "https://example.com/?key==");
    assert_eq!(nopt("https://example.com?key=value=", never()), "https://example.com/?key=value=");
}

// ============================================================
// +/space in values with default
// ============================================================

#[test]
fn test_plus_in_values_with_sort() {
    assert_eq!(n("https://example.com?foo+bar=value"), "https://example.com/?foo%20bar=value");
    let unsorted = Options { sort_query_parameters: false, ..Options::default() };
    assert_eq!(nopt("https://example.com?foo+bar=value", unsorted), "https://example.com/?foo%20bar=value");
}

// ============================================================
// Additional removeDirectoryIndex assertions from JS
// ============================================================

#[test]
fn test_remove_directory_index_comprehensive() {
    let default = || Options { remove_directory_index: RemoveDirectoryIndex::Default, ..Options::default() };
    
    // Not removed by default (RemoveDirectoryIndex::None)
    assert_eq!(n("http://sindresorhus.com/index.html"), "http://sindresorhus.com/index.html");
    
    // Removed with Default
    assert_eq!(nopt("http://sindresorhus.com/path/index.html", default()), "http://sindresorhus.com/path");
    assert_eq!(nopt("http://sindresorhus.com/path/index.htm", default()), "http://sindresorhus.com/path");
    assert_eq!(nopt("http://sindresorhus.com/index.html?a=1", default()), "http://sindresorhus.com/?a=1");
    assert_eq!(nopt("http://sindresorhus.com/path/index.html?a=1", default()), "http://sindresorhus.com/path?a=1");
    
    // Non-matching index files not removed (no extension after .)
    assert_eq!(nopt("http://sindresorhus.com/path/index.", default()), "http://sindresorhus.com/path/index.");
    
    // index.foo IS removed by default (matches index.[a-z]+)
    assert_eq!(nopt("http://sindresorhus.com/path/index.foo", default()), "http://sindresorhus.com/path");
    
    // Custom regex-like predicate
    let custom = Options {
        remove_directory_index: RemoveDirectoryIndex::List(vec![
            QueryFilter::Predicate(Box::new(|name| name.starts_with("index."))),
        ]),
        ..Options::default()
    };
    assert_eq!(nopt("http://sindresorhus.com/path/index.foo", custom), "http://sindresorhus.com/path");
}

// ============================================================
// Additional sortQueryParameters from JS
// ============================================================

#[test]
fn test_sort_query_disabled() {
    let unsorted = || Options { sort_query_parameters: false, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com/?b=b&a=a", unsorted()), "http://sindresorhus.com/?b=b&a=a");
    // Already sorted stays the same
    assert_eq!(nopt("http://sindresorhus.com/?a=a&b=b", unsorted()), "http://sindresorhus.com/?a=a&b=b");
}

// ============================================================
// Additional removeTrailingSlash from JS
// ============================================================

#[test]
fn test_remove_trailing_slash_disabled() {
    let no_trail = || Options { remove_trailing_slash: false, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com/foo/bar/", no_trail()), "http://sindresorhus.com/foo/bar/");
    // Root "/" is still removed by removeSingleSlash (default true)
    assert_eq!(nopt("http://sindresorhus.com/", no_trail()), "http://sindresorhus.com");
}

// ============================================================
// Additional encoded reserved chars from JS
// ============================================================

#[test]  
fn test_encoded_reserved_comprehensive() {
    // Multiple encoded reserved chars in both key and value
    assert_eq!(n("https://example.com/?foo%3Abar=baz%2Fqux"), "https://example.com/?foo%3Abar=baz%2Fqux");
    // Sorting with encoded reserved chars doesn't corrupt them
    assert_eq!(n("https://example.com/?b%3Ax=2&a%3Ay=1"), "https://example.com/?a%3Ay=1&b%3Ax=2");
    // Mixed encoded and plain params sort correctly  
    assert_eq!(n("https://example.com/?z=3&a%3Ab=1&m=2"), "https://example.com/?a%3Ab=1&m=2&z=3");
}

// ============================================================
// Additional path-like query strings from JS
// ============================================================

#[test]
fn test_path_like_queries_comprehensive() {
    // Multiple path-like params sorted
    assert_eq!(n("https://example.com?b&a=1"), "https://example.com/?a=1&b");
    // With utm removal
    assert_eq!(n("https://example.com?key&utm_source=test"), "https://example.com/?key");
    // Path-like after sort
    assert_eq!(n("https://example.com?key=value&key2"), "https://example.com/?key=value&key2");
    assert_eq!(n("https://example.com?key&key2=value"), "https://example.com/?key&key2=value");
    assert_eq!(n("https://example.com?a=1&b"), "https://example.com/?a=1&b");
}

// ============================================================
// Additional customProtocols from JS  
// ============================================================

#[test]
fn test_custom_protocols_comprehensive() {
    // Lowercase custom protocol
    let cp = |p: &str| Options { custom_protocols: vec![p.to_string()], ..Options::default() };
    
    assert_eq!(nopt("sindre://user:password@sorhus.com?a=one&b=two", cp("sindre")), "sindre://sorhus.com?a=one&b=two");
    assert_eq!(nopt("SINDRE://sorhus.com", cp("sindre")), "sindre://sorhus.com");
    assert_eq!(nopt("sindre://www.sorhus.com", cp("sindre")), "sindre://sorhus.com");
    assert_eq!(nopt("sindre://sorhus.com/foo/", cp("sindre")), "sindre://sorhus.com/foo");
}

// ============================================================
// Additional removePath from JS
// ============================================================

#[test]
fn test_remove_path_extended() {
    let opts = Options { remove_path: true, ..Options::default() };
    assert_eq!(nopt("http://sindresorhus.com/path?query=value", opts), "http://sindresorhus.com/?query=value");
}

// ============================================================
// Malformed percent-encoding → U+FFFD replacement
// ============================================================

#[test]
fn test_malformed_percent_encoding_replacement() {
    // %E0%A4 is an incomplete UTF-8 sequence → replaced with U+FFFD (%EF%BF%BD)
    assert_eq!(n("https://example.com?%E0%A4"), "https://example.com/?%EF%BF%BD");
    assert_eq!(n("https://example.com?%E0%A4="), "https://example.com/?%EF%BF%BD=");
    assert_eq!(n("https://example.com?%E0%A4&%E0%A4="), "https://example.com/?%EF%BF%BD&%EF%BF%BD");
    assert_eq!(n("https://example.com?%E0%A4=&%EF%BF%BD="), "https://example.com/?%EF%BF%BD=&%EF%BF%BD=");
}
