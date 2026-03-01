//! Auto-generated 1:1 parity test from JS normalize-url v9.0.0 test.js

use url_normalize::{normalize_url, Options, Protocol, EmptyQueryValue,
    RemoveQueryParameters, RemoveDirectoryIndex, QueryFilter};

fn n(url: &str) -> String { normalize_url(url, &Options::default()).unwrap() }
fn nopt(url: &str, opts: Options) -> String { normalize_url(url, &opts).unwrap() }
#[allow(dead_code)]
fn nerr(url: &str) -> bool { normalize_url(url, &Options::default()).is_err() }

#[test]
fn test_js_main() {
    assert_eq!(n("sindresorhus.com"), "http://sindresorhus.com", "line 5");
    assert_eq!(n("sindresorhus.com "), "http://sindresorhus.com", "line 6");
    assert_eq!(n("sindresorhus.com."), "http://sindresorhus.com", "line 7");
    assert_eq!(n("SindreSorhus.com"), "http://sindresorhus.com", "line 8");
    assert_eq!(n("HTTP://sindresorhus.com"), "http://sindresorhus.com", "line 9");
    assert_eq!(n("//sindresorhus.com"), "http://sindresorhus.com", "line 10");
    assert_eq!(n("http://sindresorhus.com"), "http://sindresorhus.com", "line 11");
    assert_eq!(n("http://sindresorhus.com:80"), "http://sindresorhus.com", "line 12");
    assert_eq!(n("https://sindresorhus.com:443"), "https://sindresorhus.com", "line 13");
    assert_eq!(n("http://www.sindresorhus.com"), "http://sindresorhus.com", "line 14");
    assert_eq!(n("www.com"), "http://www.com", "line 15");
    assert_eq!(n("http://www.www.sindresorhus.com"), "http://www.www.sindresorhus.com", "line 16");
    assert_eq!(n("www.sindresorhus.com"), "http://sindresorhus.com", "line 17");
    assert_eq!(n("http://sindresorhus.com/foo/"), "http://sindresorhus.com/foo", "line 18");
    assert_eq!(n("sindresorhus.com/?foo=bar baz"), "http://sindresorhus.com/?foo=bar+baz", "line 19");
    assert_eq!(n("https://foo.com/https://bar.com"), "https://foo.com/https://bar.com", "line 20");
    assert_eq!(n("https://foo.com/https://bar.com/foo//bar"), "https://foo.com/https://bar.com/foo/bar", "line 21");
    assert_eq!(n("https://foo.com/http://bar.com"), "https://foo.com/http://bar.com", "line 22");
    assert_eq!(n("https://foo.com/http://bar.com/foo//bar"), "https://foo.com/http://bar.com/foo/bar", "line 23");
    assert_eq!(n("https://foo.com/%FAIL%/07/94/ca/55.jpg"), "https://foo.com/%FAIL%/07/94/ca/55.jpg", "line 25");
    assert_eq!(n("http://sindresorhus.com/?"), "http://sindresorhus.com", "line 26");
    assert_eq!(n("êxample.com"), "http://xn--xample-hva.com", "line 27");
    assert_eq!(n("http://sindresorhus.com/?b=bar&a=foo"), "http://sindresorhus.com/?a=foo&b=bar", "line 28");
    assert_eq!(n("http://sindresorhus.com/?foo=bar*|<>:\""), "http://sindresorhus.com/?foo=bar*|%3C%3E:%22", "line 29");
    assert_eq!(n("http://sindresorhus.com:5000"), "http://sindresorhus.com:5000", "line 30");
    assert_eq!(nopt("//sindresorhus.com/", Options { normalize_protocol: false, ..Options::default() }), "//sindresorhus.com", "line 31");
    assert_eq!(nopt("//sindresorhus.com:80/", Options { normalize_protocol: false, ..Options::default() }), "//sindresorhus.com", "line 32");
    assert_eq!(n("http://sindresorhus.com/foo#bar"), "http://sindresorhus.com/foo#bar", "line 33");
    assert_eq!(nopt("http://sindresorhus.com/foo#bar", Options { strip_hash: true, ..Options::default() }), "http://sindresorhus.com/foo", "line 34");
    assert_eq!(nopt("http://sindresorhus.com/foo#bar:~:text=hello%20world", Options { strip_hash: true, ..Options::default() }), "http://sindresorhus.com/foo", "line 35");
    assert_eq!(n("http://sindresorhus.com/foo/bar/../baz"), "http://sindresorhus.com/foo/baz", "line 36");
    assert_eq!(n("http://sindresorhus.com/foo/bar/./baz"), "http://sindresorhus.com/foo/bar/baz", "line 37");
    assert_eq!(n("https://i.vimeocdn.com/filter/overlay?src0=https://i.vimeocdn.com/video/598160082_1280x720.jpg&src1=https://f.vimeocdn.com/images_v6/share/play_icon_overlay.png"), "https://i.vimeocdn.com/filter/overlay?src0=https://i.vimeocdn.com/video/598160082_1280x720.jpg&src1=https://f.vimeocdn.com/images_v6/share/play_icon_overlay.png", "line 38");
    assert_eq!(n("sindresorhus.com:123"), "http://sindresorhus.com:123", "line 39");
}

#[test]
fn test_js_defaultprotocol_option() {
    assert_eq!(nopt("sindresorhus.com", Options { default_protocol: Protocol::Https, ..Options::default() }), "https://sindresorhus.com", "line 43");
    assert_eq!(nopt("sindresorhus.com", Options { default_protocol: Protocol::Http, ..Options::default() }), "http://sindresorhus.com", "line 44");
    assert_eq!(nopt("sindresorhus.com", Options { default_protocol: Protocol::Https, ..Options::default() }), "https://sindresorhus.com", "line 47");
    assert_eq!(nopt("sindresorhus.com", Options { default_protocol: Protocol::Http, ..Options::default() }), "http://sindresorhus.com", "line 48");
}

#[test]
fn test_js_stripauthentication_option() {
    assert_eq!(n("http://user:password@www.sindresorhus.com"), "http://sindresorhus.com", "line 52");
    assert_eq!(n("https://user:password@www.sindresorhus.com"), "https://sindresorhus.com", "line 53");
    assert_eq!(n("https://user:password@www.sindresorhus.com/@user"), "https://sindresorhus.com/@user", "line 54");
    assert_eq!(n("http://user:password@www.êxample.com"), "http://xn--xample-hva.com", "line 55");
    assert_eq!(nopt("http://user:password@www.sindresorhus.com", Options { strip_authentication: false, ..Options::default() }), "http://user:password@sindresorhus.com", "line 58");
    assert_eq!(nopt("https://user:password@www.sindresorhus.com", Options { strip_authentication: false, ..Options::default() }), "https://user:password@sindresorhus.com", "line 59");
    assert_eq!(nopt("https://user:password@www.sindresorhus.com/@user", Options { strip_authentication: false, ..Options::default() }), "https://user:password@sindresorhus.com/@user", "line 60");
    assert_eq!(nopt("http://user:password@www.êxample.com", Options { strip_authentication: false, ..Options::default() }), "http://user:password@xn--xample-hva.com", "line 61");
}

#[test]
fn test_js_stripprotocol_option() {
    assert_eq!(nopt("http://www.sindresorhus.com", Options { strip_protocol: true, ..Options::default() }), "sindresorhus.com", "line 66");
    assert_eq!(nopt("http://sindresorhus.com", Options { strip_protocol: true, ..Options::default() }), "sindresorhus.com", "line 67");
    assert_eq!(nopt("https://www.sindresorhus.com", Options { strip_protocol: true, ..Options::default() }), "sindresorhus.com", "line 68");
    assert_eq!(nopt("//www.sindresorhus.com", Options { strip_protocol: true, ..Options::default() }), "sindresorhus.com", "line 69");
}

#[test]
fn test_js_striptextfragment_option() {
    assert_eq!(n("http://sindresorhus.com"), "http://sindresorhus.com", "line 73");
    assert_eq!(n("http://sindresorhus.com/about#"), "http://sindresorhus.com/about", "line 74");
    assert_eq!(n("http://sindresorhus.com/about#:~:text=hello"), "http://sindresorhus.com/about", "line 75");
    assert_eq!(n("http://sindresorhus.com/about#main"), "http://sindresorhus.com/about#main", "line 76");
    assert_eq!(n("http://sindresorhus.com/about#main:~:text=hello"), "http://sindresorhus.com/about#main", "line 77");
    assert_eq!(n("http://sindresorhus.com/about#main:~:text=hello%20world"), "http://sindresorhus.com/about#main", "line 78");
    assert_eq!(nopt("http://sindresorhus.com", Options { strip_text_fragment: false, ..Options::default() }), "http://sindresorhus.com", "line 81");
    assert_eq!(nopt("http://sindresorhus.com/about#:~:text=hello", Options { strip_text_fragment: false, ..Options::default() }), "http://sindresorhus.com/about#:~:text=hello", "line 82");
    assert_eq!(nopt("http://sindresorhus.com/about#main", Options { strip_text_fragment: false, ..Options::default() }), "http://sindresorhus.com/about#main", "line 83");
    assert_eq!(nopt("http://sindresorhus.com/about#main:~:text=hello", Options { strip_text_fragment: false, ..Options::default() }), "http://sindresorhus.com/about#main:~:text=hello", "line 84");
    assert_eq!(nopt("http://sindresorhus.com/about#main:~:text=hello%20world", Options { strip_text_fragment: false, ..Options::default() }), "http://sindresorhus.com/about#main:~:text=hello%20world", "line 85");
    assert_eq!(nopt("http://sindresorhus.com", Options { strip_hash: true, ..Options::default() }), "http://sindresorhus.com", "line 88");
    assert_eq!(nopt("http://sindresorhus.com/about#:~:text=hello", Options { strip_hash: true, ..Options::default() }), "http://sindresorhus.com/about", "line 89");
    assert_eq!(nopt("http://sindresorhus.com/about#main", Options { strip_hash: true, ..Options::default() }), "http://sindresorhus.com/about", "line 90");
    assert_eq!(nopt("http://sindresorhus.com/about#main:~:text=hello", Options { strip_hash: true, ..Options::default() }), "http://sindresorhus.com/about", "line 91");
    assert_eq!(nopt("http://sindresorhus.com/about#main:~:text=hello%20world", Options { strip_hash: true, ..Options::default() }), "http://sindresorhus.com/about", "line 92");
}

#[test]
fn test_js_stripwww_option() {
    assert_eq!(nopt("http://www.sindresorhus.com", Options { strip_www: false, ..Options::default() }), "http://www.sindresorhus.com", "line 97");
    assert_eq!(nopt("www.sindresorhus.com", Options { strip_www: false, ..Options::default() }), "http://www.sindresorhus.com", "line 98");
    assert_eq!(nopt("http://www.êxample.com", Options { strip_www: false, ..Options::default() }), "http://www.xn--xample-hva.com", "line 99");
    assert_eq!(nopt("http://www.vue.amsterdam", Options { strip_www: true, ..Options::default() }), "http://vue.amsterdam", "line 102");
    assert_eq!(nopt("http://www.sorhus.xx--bck1b9a5dre4c", Options { strip_www: true, ..Options::default() }), "http://sorhus.xx--bck1b9a5dre4c", "line 103");
    assert_eq!(n("www.unix.stackexchange.com"), "http://unix.stackexchange.com", "line 109");
    assert_eq!(n("https://www.unix.stackexchange.com"), "https://unix.stackexchange.com", "line 110");
    assert_eq!(n("www.api.example.com"), "http://api.example.com", "line 111");
    assert_eq!(n("www.com"), "http://www.com", "line 114");
    assert_eq!(n("https://www.com"), "https://www.com", "line 115");
    assert_eq!(n("www.www.com"), "http://www.www.com", "line 118");
    assert_eq!(n("www.www.example.com"), "http://www.www.example.com", "line 119");
}

#[test]
fn test_js_removequeryparameters_option() {
    assert_eq!(n("www.sindresorhus.com?foo=bar&utm_medium=test"), "http://sindresorhus.com/?foo=bar", "line 127");
    assert_eq!(nopt("http://www.sindresorhus.com", Options { strip_www: false, remove_query_parameters: RemoveQueryParameters::List(vec![QueryFilter::Predicate(Box::new(|k| k.to_lowercase().starts_with("utm_"))), QueryFilter::Exact("ref".to_string())]), ..Options::default() }), "http://www.sindresorhus.com", "line 128");
    assert_eq!(nopt("www.sindresorhus.com?foo=bar", Options { strip_www: false, remove_query_parameters: RemoveQueryParameters::List(vec![QueryFilter::Predicate(Box::new(|k| k.to_lowercase().starts_with("utm_"))), QueryFilter::Exact("ref".to_string())]), ..Options::default() }), "http://www.sindresorhus.com/?foo=bar", "line 129");
    assert_eq!(nopt("www.sindresorhus.com?foo=bar&utm_medium=test&ref=test_ref", Options { strip_www: false, remove_query_parameters: RemoveQueryParameters::List(vec![QueryFilter::Predicate(Box::new(|k| k.to_lowercase().starts_with("utm_"))), QueryFilter::Exact("ref".to_string())]), ..Options::default() }), "http://www.sindresorhus.com/?foo=bar", "line 130");
    assert_eq!(nopt("https://example.com?foo=1&foo2=2&bar=3", Options { remove_query_parameters: RemoveQueryParameters::List(vec![QueryFilter::Predicate(Box::new(|k| k.starts_with("foo")))]), ..Options::default() }), "https://example.com/?bar=3", "line 131");
    assert_eq!(nopt("https://example.com?foo=1&foo2=2&bar=3", Options { remove_query_parameters: RemoveQueryParameters::List(vec![QueryFilter::Predicate(Box::new(|k| k.starts_with("foo")))]), ..Options::default() }), "https://example.com/?bar=3", "line 132");
    assert_eq!(nopt("https://example.com?foo=1&foo2=2&bar=3", Options { remove_query_parameters: RemoveQueryParameters::List(vec![QueryFilter::Exact("foo".to_string()), QueryFilter::Predicate(Box::new(|k| k.starts_with("foo2")))]), ..Options::default() }), "https://example.com/?bar=3", "line 133");
    assert_eq!(nopt("https://example.com?foo=1&bar=2", Options { remove_query_parameters: RemoveQueryParameters::List(vec![QueryFilter::Predicate(Box::new(|k| k.starts_with("foo")))]), ..Options::default() }), "https://example.com/?bar=2", "line 136");
}

#[test]
fn test_js_removequeryparameters_boolean_true_option() {
    assert_eq!(nopt("http://www.sindresorhus.com", Options { strip_www: false, remove_query_parameters: RemoveQueryParameters::All, ..Options::default() }), "http://www.sindresorhus.com", "line 145");
    assert_eq!(nopt("www.sindresorhus.com?foo=bar", Options { strip_www: false, remove_query_parameters: RemoveQueryParameters::All, ..Options::default() }), "http://www.sindresorhus.com", "line 146");
    assert_eq!(nopt("www.sindresorhus.com?foo=bar&utm_medium=test&ref=test_ref", Options { strip_www: false, remove_query_parameters: RemoveQueryParameters::All, ..Options::default() }), "http://www.sindresorhus.com", "line 147");
}

#[test]
fn test_js_removequeryparameters_boolean_false_option() {
    assert_eq!(nopt("http://www.sindresorhus.com", Options { strip_www: false, remove_query_parameters: RemoveQueryParameters::None, ..Options::default() }), "http://www.sindresorhus.com", "line 156");
    assert_eq!(nopt("www.sindresorhus.com?foo=bar", Options { strip_www: false, remove_query_parameters: RemoveQueryParameters::None, ..Options::default() }), "http://www.sindresorhus.com/?foo=bar", "line 157");
    assert_eq!(nopt("www.sindresorhus.com?foo=bar&utm_medium=test&ref=test_ref", Options { strip_www: false, remove_query_parameters: RemoveQueryParameters::None, ..Options::default() }), "http://www.sindresorhus.com/?foo=bar&ref=test_ref&utm_medium=test", "line 158");
}

#[test]
fn test_js_keepqueryparameters_option() {
    assert_eq!(nopt("https://sindresorhus.com", Options { strip_www: false, remove_query_parameters: RemoveQueryParameters::None, keep_query_parameters: Some(vec![QueryFilter::Predicate(Box::new(|k| k.to_lowercase().starts_with("utm_"))), QueryFilter::Exact("ref".to_string())]), ..Options::default() }), "https://sindresorhus.com", "line 167");
    assert_eq!(nopt("www.sindresorhus.com?foo=bar", Options { strip_www: false, remove_query_parameters: RemoveQueryParameters::None, keep_query_parameters: Some(vec![QueryFilter::Predicate(Box::new(|k| k.to_lowercase().starts_with("utm_"))), QueryFilter::Exact("ref".to_string())]), ..Options::default() }), "http://www.sindresorhus.com", "line 168");
    assert_eq!(nopt("www.sindresorhus.com?foo=bar&utm_medium=test&ref=test_ref", Options { strip_www: false, remove_query_parameters: RemoveQueryParameters::None, keep_query_parameters: Some(vec![QueryFilter::Predicate(Box::new(|k| k.to_lowercase().starts_with("utm_"))), QueryFilter::Exact("ref".to_string())]), ..Options::default() }), "http://www.sindresorhus.com/?ref=test_ref&utm_medium=test", "line 169");
}

#[test]
fn test_js_forcehttp_option() {
    assert_eq!(n("https://sindresorhus.com"), "https://sindresorhus.com", "line 206");
    assert_eq!(nopt("http://sindresorhus.com", Options { force_http: true, ..Options::default() }), "http://sindresorhus.com", "line 207");
    assert_eq!(nopt("https://www.sindresorhus.com", Options { force_http: true, ..Options::default() }), "http://sindresorhus.com", "line 208");
    assert_eq!(nopt("//sindresorhus.com", Options { force_http: true, ..Options::default() }), "http://sindresorhus.com", "line 209");
}

#[test]
fn test_js_forcehttps_option() {
    assert_eq!(n("https://sindresorhus.com"), "https://sindresorhus.com", "line 222");
    assert_eq!(nopt("http://sindresorhus.com", Options { force_https: true, ..Options::default() }), "https://sindresorhus.com", "line 223");
    assert_eq!(nopt("https://www.sindresorhus.com", Options { force_https: true, ..Options::default() }), "https://sindresorhus.com", "line 224");
    assert_eq!(nopt("//sindresorhus.com", Options { force_https: true, ..Options::default() }), "https://sindresorhus.com", "line 225");
}

#[test]
fn test_js_removetrailingslash_option() {
    assert_eq!(n("http://sindresorhus.com"), "http://sindresorhus.com", "line 230");
    assert_eq!(n("http://sindresorhus.com/"), "http://sindresorhus.com", "line 231");
    assert_eq!(nopt("http://sindresorhus.com", Options { remove_trailing_slash: false, ..Options::default() }), "http://sindresorhus.com", "line 232");
    assert_eq!(nopt("http://sindresorhus.com/", Options { remove_trailing_slash: false, ..Options::default() }), "http://sindresorhus.com", "line 233");
    assert_eq!(n("http://sindresorhus.com/redirect"), "http://sindresorhus.com/redirect", "line 234");
    assert_eq!(n("http://sindresorhus.com/redirect/"), "http://sindresorhus.com/redirect", "line 235");
    assert_eq!(nopt("http://sindresorhus.com/redirect/", Options { remove_trailing_slash: false, ..Options::default() }), "http://sindresorhus.com/redirect/", "line 236");
    assert_eq!(nopt("http://sindresorhus.com/redirect/", Options { remove_trailing_slash: false, ..Options::default() }), "http://sindresorhus.com/redirect/", "line 237");
    assert_eq!(n("http://sindresorhus.com/#/"), "http://sindresorhus.com/#/", "line 238");
    assert_eq!(nopt("http://sindresorhus.com/#/", Options { remove_trailing_slash: false, ..Options::default() }), "http://sindresorhus.com/#/", "line 239");
    assert_eq!(n("http://sindresorhus.com/?unicorns=true"), "http://sindresorhus.com/?unicorns=true", "line 240");
    assert_eq!(nopt("http://sindresorhus.com/?unicorns=true", Options { remove_trailing_slash: false, ..Options::default() }), "http://sindresorhus.com/?unicorns=true", "line 241");
}

#[test]
fn test_js_removeexplicitport_option() {
    assert_eq!(nopt("http://sindresorhus.com:123", Options { remove_explicit_port: true, ..Options::default() }), "http://sindresorhus.com", "line 246");
    assert_eq!(nopt("https://sindresorhus.com:123", Options { remove_explicit_port: true, ..Options::default() }), "https://sindresorhus.com", "line 247");
    assert_eq!(nopt("http://sindresorhus.com:443", Options { remove_explicit_port: true, ..Options::default() }), "http://sindresorhus.com", "line 248");
    assert_eq!(nopt("https://sindresorhus.com:80", Options { remove_explicit_port: true, ..Options::default() }), "https://sindresorhus.com", "line 249");
}

#[test]
fn test_js_removesingleslash_option() {
    assert_eq!(nopt("https://sindresorhus.com", Options { remove_single_slash: false, ..Options::default() }), "https://sindresorhus.com", "line 254");
    assert_eq!(nopt("https://sindresorhus.com/", Options { remove_single_slash: false, ..Options::default() }), "https://sindresorhus.com/", "line 255");
    assert_eq!(nopt("https://sindresorhus.com/redirect", Options { remove_single_slash: false, ..Options::default() }), "https://sindresorhus.com/redirect", "line 256");
    assert_eq!(nopt("https://sindresorhus.com/redirect/", Options { remove_single_slash: false, ..Options::default() }), "https://sindresorhus.com/redirect", "line 257");
    assert_eq!(nopt("https://sindresorhus.com/#/", Options { remove_single_slash: false, ..Options::default() }), "https://sindresorhus.com/#/", "line 258");
    assert_eq!(nopt("https://sindresorhus.com/?unicorns=true", Options { remove_single_slash: false, ..Options::default() }), "https://sindresorhus.com/?unicorns=true", "line 259");
}

#[test]
fn test_js_removesingleslash_option_combined_with_removetrailingslash_option() {
    assert_eq!(nopt("https://sindresorhus.com", Options { remove_single_slash: false, remove_trailing_slash: false, ..Options::default() }), "https://sindresorhus.com", "line 264");
    assert_eq!(nopt("https://sindresorhus.com/", Options { remove_single_slash: false, remove_trailing_slash: false, ..Options::default() }), "https://sindresorhus.com/", "line 265");
    assert_eq!(nopt("https://sindresorhus.com/redirect", Options { remove_single_slash: false, remove_trailing_slash: false, ..Options::default() }), "https://sindresorhus.com/redirect", "line 266");
    assert_eq!(nopt("https://sindresorhus.com/redirect/", Options { remove_single_slash: false, remove_trailing_slash: false, ..Options::default() }), "https://sindresorhus.com/redirect/", "line 267");
    assert_eq!(nopt("https://sindresorhus.com/#/", Options { remove_single_slash: false, remove_trailing_slash: false, ..Options::default() }), "https://sindresorhus.com/#/", "line 268");
    assert_eq!(nopt("https://sindresorhus.com/?unicorns=true", Options { remove_single_slash: false, remove_trailing_slash: false, ..Options::default() }), "https://sindresorhus.com/?unicorns=true", "line 269");
}

#[test]
fn test_js_removedirectoryindex_option() {
    assert_eq!(n("http://sindresorhus.com/index.html"), "http://sindresorhus.com/index.html", "line 274");
    assert_eq!(nopt("http://sindresorhus.com/index.html", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Exact("index.html".to_string()), QueryFilter::Exact("index.php".to_string())]), ..Options::default() }), "http://sindresorhus.com", "line 275");
    assert_eq!(nopt("http://sindresorhus.com/index.htm", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Exact("index.html".to_string()), QueryFilter::Exact("index.php".to_string())]), ..Options::default() }), "http://sindresorhus.com/index.htm", "line 276");
    assert_eq!(nopt("http://sindresorhus.com/index.php", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Exact("index.html".to_string()), QueryFilter::Exact("index.php".to_string())]), ..Options::default() }), "http://sindresorhus.com", "line 277");
    assert_eq!(n("http://sindresorhus.com/path/index.html"), "http://sindresorhus.com/path/index.html", "line 278");
    assert_eq!(nopt("http://sindresorhus.com/path/index.html", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Exact("index.html".to_string()), QueryFilter::Exact("index.php".to_string())]), ..Options::default() }), "http://sindresorhus.com/path", "line 279");
    assert_eq!(nopt("http://sindresorhus.com/path/index.htm", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Exact("index.html".to_string()), QueryFilter::Exact("index.php".to_string())]), ..Options::default() }), "http://sindresorhus.com/path/index.htm", "line 280");
    assert_eq!(nopt("http://sindresorhus.com/path/index.php", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Exact("index.html".to_string()), QueryFilter::Exact("index.php".to_string())]), ..Options::default() }), "http://sindresorhus.com/path", "line 281");
    assert_eq!(nopt("http://sindresorhus.com/foo/bar/index.html", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Exact("index.html".to_string()), QueryFilter::Exact("index.php".to_string())]), ..Options::default() }), "http://sindresorhus.com/foo/bar", "line 282");
    assert_eq!(n("http://sindresorhus.com/index.html"), "http://sindresorhus.com/index.html", "line 285");
    assert_eq!(nopt("http://sindresorhus.com/index.html", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Predicate(Box::new(|name| { let re_match = name.starts_with("index.") && name.len() > 6 && name[6..].bytes().all(|b| b.is_ascii_lowercase()); re_match })), QueryFilter::Exact("remove.html".to_string())]), ..Options::default() }), "http://sindresorhus.com", "line 286");
    assert_eq!(nopt("http://sindresorhus.com/index/index.html", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Predicate(Box::new(|name| { let re_match = name.starts_with("index.") && name.len() > 6 && name[6..].bytes().all(|b| b.is_ascii_lowercase()); re_match })), QueryFilter::Exact("remove.html".to_string())]), ..Options::default() }), "http://sindresorhus.com/index", "line 287");
    assert_eq!(nopt("http://sindresorhus.com/remove.html", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Predicate(Box::new(|name| { let re_match = name.starts_with("index.") && name.len() > 6 && name[6..].bytes().all(|b| b.is_ascii_lowercase()); re_match })), QueryFilter::Exact("remove.html".to_string())]), ..Options::default() }), "http://sindresorhus.com", "line 288");
    assert_eq!(nopt("http://sindresorhus.com/default.htm", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Predicate(Box::new(|name| { let re_match = name.starts_with("index.") && name.len() > 6 && name[6..].bytes().all(|b| b.is_ascii_lowercase()); re_match })), QueryFilter::Exact("remove.html".to_string())]), ..Options::default() }), "http://sindresorhus.com/default.htm", "line 289");
    assert_eq!(nopt("http://sindresorhus.com/index.php", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Predicate(Box::new(|name| { let re_match = name.starts_with("index.") && name.len() > 6 && name[6..].bytes().all(|b| b.is_ascii_lowercase()); re_match })), QueryFilter::Exact("remove.html".to_string())]), ..Options::default() }), "http://sindresorhus.com", "line 290");
    assert_eq!(nopt("http://sindresorhus.com/index.html", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Predicate(Box::new(|name| { let re_match = name.starts_with("index.") && name.len() > 6 && name[6..].bytes().all(|b| b.is_ascii_lowercase()); re_match })), QueryFilter::Exact("remove.html".to_string())]), remove_trailing_slash: false, ..Options::default() }), "http://sindresorhus.com", "line 295");
    assert_eq!(nopt("http://sindresorhus.com/index.php", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Predicate(Box::new(|name| { let re_match = name.starts_with("index.") && name.len() > 6 && name[6..].bytes().all(|b| b.is_ascii_lowercase()); re_match })), QueryFilter::Exact("remove.html".to_string())]), remove_trailing_slash: false, ..Options::default() }), "http://sindresorhus.com", "line 296");
    assert_eq!(n("http://sindresorhus.com/index.html"), "http://sindresorhus.com/index.html", "line 299");
    assert_eq!(nopt("http://sindresorhus.com/index.html", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Predicate(Box::new(|name| name.starts_with("index.")))]), ..Options::default() }), "http://sindresorhus.com", "line 300");
    assert_eq!(nopt("http://sindresorhus.com/index.htm", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Predicate(Box::new(|name| name.starts_with("index.")))]), ..Options::default() }), "http://sindresorhus.com", "line 301");
    assert_eq!(nopt("http://sindresorhus.com/index.php", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Predicate(Box::new(|name| name.starts_with("index.")))]), ..Options::default() }), "http://sindresorhus.com", "line 302");
    assert_eq!(nopt("http://example.com/fr", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Exact("fr".to_string())]), ..Options::default() }), "http://example.com", "line 306");
    assert_eq!(nopt("http://example.com/fr/", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Exact("fr".to_string())]), ..Options::default() }), "http://example.com", "line 307");
    assert_eq!(nopt("http://example.com/path/fr", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Exact("fr".to_string())]), ..Options::default() }), "http://example.com/path", "line 308");
    assert_eq!(nopt("http://example.com/path/fr/", Options { remove_directory_index: RemoveDirectoryIndex::List(vec![QueryFilter::Exact("fr".to_string())]), ..Options::default() }), "http://example.com/path", "line 309");
}

#[test]
fn test_js_removetrailingslash_and_removedirectoryindex_options() {
    assert_eq!(nopt("http://sindresorhus.com/path/", Options { remove_trailing_slash: true, remove_directory_index: RemoveDirectoryIndex::Default, ..Options::default() }), "http://sindresorhus.com/path", "line 317");
    assert_eq!(nopt("http://sindresorhus.com/path/index.html", Options { remove_trailing_slash: true, remove_directory_index: RemoveDirectoryIndex::Default, ..Options::default() }), "http://sindresorhus.com/path", "line 318");
    assert_eq!(nopt("http://sindresorhus.com/#/path/", Options { remove_trailing_slash: true, remove_directory_index: RemoveDirectoryIndex::Default, ..Options::default() }), "http://sindresorhus.com/#/path/", "line 319");
    assert_eq!(nopt("http://sindresorhus.com/foo/#/bar/", Options { remove_trailing_slash: true, remove_directory_index: RemoveDirectoryIndex::Default, ..Options::default() }), "http://sindresorhus.com/foo#/bar/", "line 320");
    assert_eq!(nopt("http://sindresorhus.com/path/", Options { remove_trailing_slash: false, remove_directory_index: RemoveDirectoryIndex::Default, ..Options::default() }), "http://sindresorhus.com/path/", "line 326");
    assert_eq!(nopt("http://sindresorhus.com/path/index.html", Options { remove_trailing_slash: false, remove_directory_index: RemoveDirectoryIndex::Default, ..Options::default() }), "http://sindresorhus.com/path/", "line 327");
    assert_eq!(nopt("http://sindresorhus.com/#/path/", Options { remove_trailing_slash: false, remove_directory_index: RemoveDirectoryIndex::Default, ..Options::default() }), "http://sindresorhus.com/#/path/", "line 328");
}

#[test]
fn test_js_sortqueryparameters_option() {
    assert_eq!(nopt("http://sindresorhus.com/?a=Z&b=Y&c=X&d=W", Options { sort_query_parameters: true, ..Options::default() }), "http://sindresorhus.com/?a=Z&b=Y&c=X&d=W", "line 335");
    assert_eq!(nopt("http://sindresorhus.com/?b=Y&c=X&a=Z&d=W", Options { sort_query_parameters: true, ..Options::default() }), "http://sindresorhus.com/?a=Z&b=Y&c=X&d=W", "line 336");
    assert_eq!(nopt("http://sindresorhus.com/?a=Z&d=W&b=Y&c=X", Options { sort_query_parameters: true, ..Options::default() }), "http://sindresorhus.com/?a=Z&b=Y&c=X&d=W", "line 337");
    assert_eq!(nopt("http://sindresorhus.com/", Options { sort_query_parameters: true, ..Options::default() }), "http://sindresorhus.com", "line 338");
    assert_eq!(nopt("http://sindresorhus.com/?a=Z&b=Y&c=X&d=W", Options { sort_query_parameters: false, ..Options::default() }), "http://sindresorhus.com/?a=Z&b=Y&c=X&d=W", "line 343");
    assert_eq!(nopt("http://sindresorhus.com/?b=Y&c=X&a=Z&d=W", Options { sort_query_parameters: false, ..Options::default() }), "http://sindresorhus.com/?b=Y&c=X&a=Z&d=W", "line 344");
    assert_eq!(nopt("http://sindresorhus.com/?a=Z&d=W&b=Y&c=X", Options { sort_query_parameters: false, ..Options::default() }), "http://sindresorhus.com/?a=Z&d=W&b=Y&c=X", "line 345");
    assert_eq!(nopt("http://sindresorhus.com/", Options { sort_query_parameters: false, ..Options::default() }), "http://sindresorhus.com", "line 346");
}

#[test]
fn test_js_remove_duplicate_pathname_slashes() {
    assert_eq!(n("http://sindresorhus.com////foo/bar"), "http://sindresorhus.com/foo/bar", "line 371");
    assert_eq!(n("http://sindresorhus.com////foo////bar"), "http://sindresorhus.com/foo/bar", "line 372");
    assert_eq!(nopt("//sindresorhus.com//foo", Options { normalize_protocol: false, ..Options::default() }), "//sindresorhus.com/foo", "line 373");
    assert_eq!(n("http://sindresorhus.com:5000///foo"), "http://sindresorhus.com:5000/foo", "line 374");
    assert_eq!(n("http://sindresorhus.com///foo"), "http://sindresorhus.com/foo", "line 375");
    assert_eq!(n("http://sindresorhus.com:5000//foo"), "http://sindresorhus.com:5000/foo", "line 376");
    assert_eq!(n("http://sindresorhus.com//foo"), "http://sindresorhus.com/foo", "line 377");
    assert_eq!(n("http://sindresorhus.com/s3://sindresorhus.com"), "http://sindresorhus.com/s3://sindresorhus.com", "line 378");
    assert_eq!(n("http://sindresorhus.com/s3://sindresorhus.com//foo"), "http://sindresorhus.com/s3://sindresorhus.com/foo", "line 379");
    assert_eq!(n("http://sindresorhus.com//foo/s3://sindresorhus.com"), "http://sindresorhus.com/foo/s3://sindresorhus.com", "line 380");
    assert_eq!(n("http://sindresorhus.com/git://sindresorhus.com"), "http://sindresorhus.com/git://sindresorhus.com", "line 381");
    assert_eq!(n("http://sindresorhus.com/git://sindresorhus.com//foo"), "http://sindresorhus.com/git://sindresorhus.com/foo", "line 382");
    assert_eq!(n("http://sindresorhus.com//foo/git://sindresorhus.com//foo"), "http://sindresorhus.com/foo/git://sindresorhus.com/foo", "line 383");
    assert_eq!(n("http://sindresorhus.com/a://sindresorhus.com//foo"), "http://sindresorhus.com/a:/sindresorhus.com/foo", "line 384");
    assert_eq!(n("http://sindresorhus.com/alongprotocolwithin50charlimitxxxxxxxxxxxxxxxxxxxx://sindresorhus.com//foo"), "http://sindresorhus.com/alongprotocolwithin50charlimitxxxxxxxxxxxxxxxxxxxx://sindresorhus.com/foo", "line 385");
    assert_eq!(n("http://sindresorhus.com/alongprotocolexceeds50charlimitxxxxxxxxxxxxxxxxxxxxx://sindresorhus.com//foo"), "http://sindresorhus.com/alongprotocolexceeds50charlimitxxxxxxxxxxxxxxxxxxxxx:/sindresorhus.com/foo", "line 386");
    assert_eq!(n("http://sindresorhus.com/a2-.+://sindresorhus.com"), "http://sindresorhus.com/a2-.+://sindresorhus.com", "line 387");
    assert_eq!(n("http://sindresorhus.com/a2-.+_://sindresorhus.com"), "http://sindresorhus.com/a2-.+_:/sindresorhus.com", "line 388");
    assert_eq!(n("http://sindresorhus.com/2abc://sindresorhus.com"), "http://sindresorhus.com/2abc:/sindresorhus.com", "line 389");
}

#[test]
fn test_js_data_url() {
    assert_eq!(n("data:text/plain,foo"), "data:,foo", "line 401");
    assert_eq!(n("data:;charset=us-ascii,foo"), "data:,foo", "line 404");
    assert_eq!(n("data:;charset=UTF-8;,foo"), "data:;charset=utf-8,foo", "line 407");
    assert_eq!(n("data:,"), "data:,", "line 410");
    assert_eq!(n("data:;charset=utf-8,foo"), "data:;charset=utf-8,foo", "line 413");
    assert_eq!(n("data:TEXT/HTML,foo"), "data:text/html,foo", "line 416");
    assert_eq!(n("data:,foo# "), "data:,foo", "line 419");
    assert_eq!(n("data:;foo=;bar,"), "data:;foo;bar,", "line 422");
    assert_eq!(n("data:;charset=UTF-8,foo"), "data:;charset=utf-8,foo", "line 425");
    assert_eq!(n("data:;base64, Zm9v #foo #bar"), "data:;base64,Zm9v#foo #bar", "line 428");
    assert_eq!(n("data:, foo #bar"), "data:, foo #bar", "line 431");
    assert_eq!(nopt("data:,sindresorhus.com/", Options { strip_hash: true, ..Options::default() }), "data:,sindresorhus.com/", "line 446");
    assert_eq!(nopt("data:,sindresorhus.com/index.html", Options { strip_hash: true, ..Options::default() }), "data:,sindresorhus.com/index.html", "line 447");
    assert_eq!(nopt("data:,sindresorhus.com?foo=bar&a=a&utm_medium=test", Options { strip_hash: true, ..Options::default() }), "data:,sindresorhus.com?foo=bar&a=a&utm_medium=test", "line 448");
    assert_eq!(nopt("data:,foo#bar", Options { strip_hash: true, ..Options::default() }), "data:,foo", "line 449");
    assert_eq!(nopt("data:,www.sindresorhus.com", Options { strip_hash: true, ..Options::default() }), "data:,www.sindresorhus.com", "line 450");
}

#[test]
fn test_js_prevents_homograph_attack() {
    assert_eq!(n("https://ebаy.com"), "https://xn--eby-7cd.com", "line 455");
}

#[test]
fn test_js_ignore_custom_schemes() {
    assert_eq!(n("tel:004346382763"), "tel:004346382763", "line 473");
    assert_eq!(n("mailto:office@foo.com"), "mailto:office@foo.com", "line 474");
    assert_eq!(n("sindre://www.sindresorhus.com"), "sindre://www.sindresorhus.com", "line 475");
    assert_eq!(n("foo.bar://www.example.com"), "foo.bar://www.example.com", "line 476");
    assert_eq!(n("foo:bar"), "foo:bar", "line 477");
    assert_eq!(nopt("sindre://www.sindresorhus.com", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "sindre://sindresorhus.com", "line 480");
}

#[test]
fn test_js_customprotocols_option() {
    assert_eq!(nopt("sindre://www.sorhus.com", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "sindre://sorhus.com", "line 487");
    assert_eq!(nopt("sindre://www.sorhus.com/", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "sindre://sorhus.com", "line 488");
    assert_eq!(nopt("sindre://www.sorhus.com/foo/bar", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "sindre://sorhus.com/foo/bar", "line 489");
    assert_eq!(nopt("sindre://user:password@www.sorhus.com", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "sindre://sorhus.com", "line 492");
    assert_eq!(nopt("sindre://sorhus.com/foo/", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "sindre://sorhus.com/foo", "line 495");
    assert_eq!(nopt("sindre://sorhus.com?b=two&a=one", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "sindre://sorhus.com?a=one&b=two", "line 498");
    assert_eq!(nopt("sindre://sorhus.com/foo#bar", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "sindre://sorhus.com/foo#bar", "line 501");
    assert_eq!(nopt("sindre://sorhus.com/foo#bar", Options { custom_protocols: vec!["sindre".to_string()], strip_hash: true, ..Options::default() }), "sindre://sorhus.com/foo", "line 502");
    assert_eq!(nopt("sindre://sorhus.com?foo=bar&utm_source=test", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "sindre://sorhus.com?foo=bar", "line 505");
    assert_eq!(nopt("sindre://sorhus.com//foo//bar", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "sindre://sorhus.com/foo/bar", "line 508");
    assert_eq!(nopt("sindre://sorhus.com/%7Efoo/", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "sindre://sorhus.com/~foo", "line 511");
    assert_eq!(nopt("sindre://www.sorhus.com", Options { custom_protocols: vec![], ..Options::default() }), "sindre://www.sorhus.com", "line 514");
    assert_eq!(nopt("other://www.sorhus.com", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "other://www.sorhus.com", "line 517");
    assert_eq!(nopt("tel:004346382763", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "tel:004346382763", "line 518");
    assert_eq!(nopt("sindre://www.sorhus.com", Options { custom_protocols: vec!["sindre".to_string(), "app".to_string()], ..Options::default() }), "sindre://sorhus.com", "line 522");
    assert_eq!(nopt("app://www.sorhus.com", Options { custom_protocols: vec!["sindre".to_string(), "app".to_string()], ..Options::default() }), "app://sorhus.com", "line 523");
    assert_eq!(nopt("other://www.sorhus.com", Options { custom_protocols: vec!["sindre".to_string(), "app".to_string()], ..Options::default() }), "other://www.sorhus.com", "line 524");
    assert_eq!(nopt("foo.bar://www.example.com", Options { custom_protocols: vec!["foo.bar".to_string()], ..Options::default() }), "foo.bar://example.com", "line 527");
    assert_eq!(nopt("FOO.BAR://www.example.com", Options { custom_protocols: vec!["foo.bar".to_string()], ..Options::default() }), "foo.bar://example.com", "line 528");
    assert_eq!(nopt("sindre://sorhus.com", Options { force_http: true, ..Options::default() }), "sindre://sorhus.com", "line 531");
    assert_eq!(nopt("sindre://sorhus.com", Options { force_https: true, ..Options::default() }), "sindre://sorhus.com", "line 532");
    assert_eq!(nopt("sindre://sorhus.com", Options { strip_protocol: true, ..Options::default() }), "sindre://sorhus.com", "line 535");
    assert_eq!(nopt("sindre://sorhus.com:8080", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "sindre://sorhus.com:8080", "line 538");
    assert_eq!(nopt("sindre://sorhus.com:8080/foo", Options { custom_protocols: vec!["sindre".to_string()], remove_explicit_port: true, ..Options::default() }), "sindre://sorhus.com/foo", "line 539");
    assert_eq!(nopt("sindre://www.sorhus.com", Options { custom_protocols: vec!["SINDRE".to_string()], ..Options::default() }), "sindre://sorhus.com", "line 542");
    assert_eq!(nopt("sindre://www.sorhus.com", Options { custom_protocols: vec!["Sindre".to_string()], ..Options::default() }), "sindre://sorhus.com", "line 543");
    assert_eq!(nopt("sindre://www.sorhus.com", Options { custom_protocols: vec!["sindre:".to_string()], ..Options::default() }), "sindre://sorhus.com", "line 544");
    assert_eq!(nopt("sindre://www.sorhus.com", Options { custom_protocols: vec![" sindre ".to_string()], ..Options::default() }), "sindre://sorhus.com", "line 545");
    assert_eq!(nopt("sindre://www.sorhus.com", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "sindre://sorhus.com", "line 548");
    assert_eq!(nopt("sindre://www.sorhus.com", Options { custom_protocols: vec![], ..Options::default() }), "sindre://www.sorhus.com", "line 551");
    assert_eq!(nopt("sindre://sorhus.com/foo/../bar", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "sindre://sorhus.com/bar", "line 554");
    assert_eq!(nopt("sindre://sorhus.com/foo/./bar", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "sindre://sorhus.com/foo/bar", "line 555");
    assert_eq!(nopt("sindre://user:password@www.sorhus.com", Options { custom_protocols: vec!["sindre".to_string()], strip_authentication: false, ..Options::default() }), "sindre://user:password@sorhus.com", "line 558");
    assert_eq!(nopt("sindre://www.sorhus.com", Options { custom_protocols: vec!["sindre".to_string()], strip_www: false, ..Options::default() }), "sindre://www.sorhus.com", "line 561");
    assert_eq!(nopt("sindre://sorhus.com/foo/", Options { custom_protocols: vec!["sindre".to_string()], remove_trailing_slash: false, ..Options::default() }), "sindre://sorhus.com/foo/", "line 564");
    assert_eq!(nopt("sindre://sorhus.com?foo=bar", Options { custom_protocols: vec!["sindre".to_string()], remove_query_parameters: RemoveQueryParameters::All, ..Options::default() }), "sindre://sorhus.com", "line 567");
    assert_eq!(nopt("sindre://sorhus.com?foo=bar&baz=qux", Options { custom_protocols: vec!["sindre".to_string()], keep_query_parameters: Some(vec![QueryFilter::Exact("foo".to_string())]), ..Options::default() }), "sindre://sorhus.com?foo=bar", "line 570");
    assert_eq!(nopt("http://www.sorhus.com", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "http://sorhus.com", "line 573");
    assert_eq!(nopt("https://www.sorhus.com", Options { custom_protocols: vec!["sindre".to_string()], ..Options::default() }), "https://sorhus.com", "line 574");
}

#[test]
fn test_js_encoded_backslashes_do_not_get_decoded() {
    assert_eq!(n("https://foo.com/some%5Bthing%5Celse/that-is%40great@coding"), "https://foo.com/some[thing%5Celse/that-is%40great@coding", "line 578");
    assert_eq!(n("https://foo.com/something%5Celse/great"), "https://foo.com/something%5Celse/great", "line 579");
}

#[test]
fn test_js_removepath_option() {
    assert_eq!(nopt("https://example.com/path/to/page", Options { remove_path: true, ..Options::default() }), "https://example.com", "line 587");
    assert_eq!(nopt("https://example.com/path/to/page?query=1", Options { remove_path: true, ..Options::default() }), "https://example.com/?query=1", "line 588");
    assert_eq!(nopt("https://example.com/path/to/page#hash", Options { remove_path: true, ..Options::default() }), "https://example.com/#hash", "line 589");
    assert_eq!(nopt("https://example.com/", Options { remove_path: true, ..Options::default() }), "https://example.com", "line 590");
    assert_eq!(nopt("https://example.com", Options { remove_path: true, ..Options::default() }), "https://example.com", "line 591");
    assert_eq!(nopt("https://example.com/path/", Options { remove_trailing_slash: true, remove_path: true, ..Options::default() }), "https://example.com", "line 594");
    assert_eq!(nopt("https://www.example.com/path", Options { remove_path: true, ..Options::default() }), "https://example.com", "line 595");
}

#[test]
fn test_js_transformpath_option() {
    assert_eq!(nopt("https://example.com/api/v1/users", Options { transform_path: Some(Box::new(|parts| if parts.is_empty() { vec![] } else { vec![parts[0].clone()] })), ..Options::default() }), "https://example.com/api", "line 601");
    assert_eq!(nopt("https://example.com/path/to/page", Options { transform_path: Some(Box::new(|parts| if parts.is_empty() { vec![] } else { vec![parts[0].clone()] })), ..Options::default() }), "https://example.com/path", "line 602");
    assert_eq!(nopt("https://example.com/", Options { transform_path: Some(Box::new(|parts| if parts.is_empty() { vec![] } else { vec![parts[0].clone()] })), ..Options::default() }), "https://example.com", "line 603");
    assert_eq!(nopt("https://example.com/admin/users", Options { transform_path: Some(Box::new(|parts| parts.into_iter().filter(|p| p != "admin").collect())), ..Options::default() }), "https://example.com/users", "line 607");
    assert_eq!(nopt("https://example.com/path/admin/page", Options { transform_path: Some(Box::new(|parts| parts.into_iter().filter(|p| p != "admin").collect())), ..Options::default() }), "https://example.com/path/page", "line 608");
    assert_eq!(nopt("https://example.com/api/v1/users", Options { transform_path: Some(Box::new(|parts| if parts.first().map(|s| s.as_str()) == Some("api") { vec![parts[0].clone()] } else { vec![] })), ..Options::default() }), "https://example.com/api", "line 619");
    assert_eq!(nopt("https://example.com/other/path", Options { transform_path: Some(Box::new(|parts| if parts.first().map(|s| s.as_str()) == Some("api") { vec![parts[0].clone()] } else { vec![] })), ..Options::default() }), "https://example.com", "line 620");
    assert_eq!(nopt("https://example.com/path", Options { transform_path: Some(Box::new(|_| vec![])), ..Options::default() }), "https://example.com", "line 623");
    assert_eq!(nopt("https://example.com/path", Options { transform_path: Some(Box::new(|_| vec![])), ..Options::default() }), "https://example.com", "line 624");
    assert_eq!(nopt("https://example.com/path", Options { transform_path: Some(Box::new(|_| vec![])), ..Options::default() }), "https://example.com", "line 625");
}

#[test]
fn test_js_path_like_query_strings_without_equals_signs_are_preserved() {
    assert_eq!(n("https://example.com/index.php?/Some/Route/To/Path/12345"), "https://example.com/index.php?/Some/Route/To/Path/12345", "line 636");
    assert_eq!(n("https://example.com/script.php?/api/v1/users/123"), "https://example.com/script.php?/api/v1/users/123", "line 637");
    assert_eq!(n("https://example.com/app.php?/admin/dashboard"), "https://example.com/app.php?/admin/dashboard", "line 638");
    assert_eq!(n("https://example.com/index.php?/path/"), "https://example.com/index.php?/path", "line 640");
    assert_eq!(nopt("https://example.com/index.php?/path/", Options { remove_trailing_slash: false, ..Options::default() }), "https://example.com/index.php?/path/", "line 642");
    assert_eq!(n("https://example.com/index.php?b=2&/path/to/resource&a=1"), "https://example.com/index.php?/path/to/resource&a=1&b=2", "line 645");
    assert_eq!(n("https://example.com/index.php?/path&param=value"), "https://example.com/index.php?/path&param=value", "line 646");
    assert_eq!(n("https://example.com/index.php?key="), "https://example.com/index.php?key=", "line 649");
    assert_eq!(n("https://example.com/index.php?key=&another="), "https://example.com/index.php?another=&key=", "line 650");
    assert_eq!(n("https://example.com/index.php?key"), "https://example.com/index.php?key", "line 653");
    assert_eq!(n("https://example.com/index.php?a&b&c"), "https://example.com/index.php?a&b&c", "line 654");
    assert_eq!(nopt("https://example.com/index.php?/Some/Route/To/Path/12345", Options { sort_query_parameters: false, ..Options::default() }), "https://example.com/index.php?/Some/Route/To/Path/12345", "line 657");
    assert_eq!(nopt("https://example.com/index.php?key", Options { sort_query_parameters: false, ..Options::default() }), "https://example.com/index.php?key", "line 658");
    assert_eq!(n("https://example.com/index.php?/path&/longpath"), "https://example.com/index.php?/longpath&/path", "line 661");
    assert_eq!(n("https://example.com/index.php?key&anotherkey"), "https://example.com/index.php?anotherkey&key", "line 662");
    assert_eq!(n("https://example.com/index.php?/api&/api/v1/users"), "https://example.com/index.php?/api&/api/v1/users", "line 663");
}

#[test]
fn test_js_sortqueryparameters_should_preserve_encoded_reserved_characters_in_query_values() {
    assert_eq!(n("https://example.com/?token=a%2Fb%2Fc"), "https://example.com/?token=a%2Fb%2Fc", "line 672");
    assert_eq!(n("https://example.com/?token=a%2fb%2fc"), "https://example.com/?token=a%2Fb%2Fc", "line 673");
    assert_eq!(n("https://example.com/?value=:@[];,"), "https://example.com/?value=:@[];,", "line 680");
    assert_eq!(n("https://example.com/?z=1&token=a%2Fb"), "https://example.com/?token=a%2Fb&z=1", "line 683");
    assert_eq!(n("https://example.com/?A=1&%3A=2"), "https://example.com/?%3A=2&A=1", "line 686");
    assert_eq!(n("https://example.com/?foo%3Abar=1&a=2"), "https://example.com/?a=2&foo%3Abar=1", "line 687");
    assert_eq!(n("https://example.com/?foo%3Dbar=1&a=2"), "https://example.com/?a=2&foo%3Dbar=1", "line 688");
    assert_eq!(n("https://example.com/?b%26c=1&a=2"), "https://example.com/?a=2&b%26c=1", "line 689");
    assert_eq!(n("https://example.com/?%3A=1&%2F=2"), "https://example.com/?%2F=2&%3A=1", "line 692");
    assert_eq!(n("https://example.com/?%5B=1&%3A=2"), "https://example.com/?%3A=2&%5B=1", "line 693");
    assert_eq!(n("https://example.com/?z%3A=val%2F&a%2F=val%3A"), "https://example.com/?a%2F=val%3A&z%3A=val%2F", "line 696");
    assert_eq!(n("https://example.com/?q=%3A%2F%3F"), "https://example.com/?q=%3A%2F%3F", "line 699");
    assert_eq!(n("https://example.com/?utm_source=test&token=a%2Fb"), "https://example.com/?token=a%2Fb", "line 702");
    assert_eq!(n("https://example.com/?token=a/b&utm_source=test"), "https://example.com/?token=a/b", "line 703");
    assert_eq!(nopt("https://example.com/?token=a%2Fb", Options { sort_query_parameters: false, ..Options::default() }), "https://example.com/?token=a%2Fb", "line 714");
    assert_eq!(nopt("https://example.com/?token=%3A%2F%3F", Options { sort_query_parameters: false, ..Options::default() }), "https://example.com/?token=%3A%2F%3F", "line 715");
}

#[test]
fn test_js_emptyqueryvalue_option() {
    assert_eq!(n("https://example.com?key"), "https://example.com/?key", "line 736");
    assert_eq!(n("https://example.com?key="), "https://example.com/?key=", "line 737");
    assert_eq!(n("https://example.com?a&b=&c=1"), "https://example.com/?a&b=&c=1", "line 738");
    assert_eq!(nopt("https://example.com?key", Options { empty_query_value: EmptyQueryValue::Always, ..Options::default() }), "https://example.com/?key=", "line 742");
    assert_eq!(nopt("https://example.com?key=", Options { empty_query_value: EmptyQueryValue::Always, ..Options::default() }), "https://example.com/?key=", "line 743");
    assert_eq!(nopt("https://example.com?a&b=&c=1", Options { empty_query_value: EmptyQueryValue::Always, ..Options::default() }), "https://example.com/?a=&b=&c=1", "line 744");
    assert_eq!(nopt("https://example.com?foo&bar&baz=value", Options { empty_query_value: EmptyQueryValue::Always, ..Options::default() }), "https://example.com/?bar=&baz=value&foo=", "line 745");
    assert_eq!(nopt("https://example.com?key", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?key", "line 749");
    assert_eq!(nopt("https://example.com?key=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?key", "line 750");
    assert_eq!(nopt("https://example.com?a&b=&c=1", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?a&b&c=1", "line 751");
    assert_eq!(nopt("https://example.com?foo=&bar=&baz=value", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?bar&baz=value&foo", "line 752");
    assert_eq!(nopt("https://example.com?b&a=", Options { sort_query_parameters: false, empty_query_value: EmptyQueryValue::Always, ..Options::default() }), "https://example.com/?b=&a=", "line 755");
    assert_eq!(nopt("https://example.com?b=&a", Options { sort_query_parameters: false, empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?b&a", "line 756");
    assert_eq!(nopt("https://example.com?key==", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?key==", "line 759");
    assert_eq!(nopt("https://example.com?key=value=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?key=value=", "line 760");
    assert_eq!(n("https://example.com?key&utm_source=test"), "https://example.com/?key", "line 763");
    assert_eq!(nopt("https://example.com?key&utm_source=test", Options { sort_query_parameters: false, ..Options::default() }), "https://example.com/?key", "line 764");
    assert_eq!(n("https://example.com?foo%20bar"), "https://example.com/?foo%20bar", "line 767");
    assert_eq!(n("https://example.com?foo%20bar="), "https://example.com/?foo%20bar=", "line 768");
    assert_eq!(nopt("https://example.com?foo%20bar=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?foo%20bar", "line 769");
    assert_eq!(nopt("https://example.com?a&a=", Options { sort_query_parameters: false, ..Options::default() }), "https://example.com/?a&a", "line 772");
    assert_eq!(nopt("https://example.com?a=&a", Options { sort_query_parameters: false, ..Options::default() }), "https://example.com/?a&a", "line 773");
    assert_eq!(n("https://example.com?foo+bar"), "https://example.com/?foo%20bar", "line 776");
    assert_eq!(n("https://example.com?foo+bar="), "https://example.com/?foo%20bar=", "line 777");
    assert_eq!(nopt("https://example.com?foo+bar=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?foo%20bar", "line 778");
    assert_eq!(n("https://example.com?foo+bar=value"), "https://example.com/?foo%20bar=value", "line 779");
    assert_eq!(nopt("https://example.com?foo+bar=value", Options { sort_query_parameters: false, ..Options::default() }), "https://example.com/?foo%20bar=value", "line 780");
    assert_eq!(n("https://example.com?foo%2Bbar=1"), "https://example.com/?foo%2Bbar=1", "line 783");
    assert_eq!(n("https://example.com?foo%2Bbar="), "https://example.com/?foo%2Bbar=", "line 784");
    assert_eq!(nopt("https://example.com?foo%2Bbar=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?foo%2Bbar", "line 785");
    assert_eq!(n("https://example.com?%E0%A4"), "https://example.com/?%EF%BF%BD", "line 788");
    assert_eq!(n("https://example.com?%E0%A4="), "https://example.com/?%EF%BF%BD=", "line 789");
    assert_eq!(n("https://example.com?%E0%A4&%E0%A4="), "https://example.com/?%EF%BF%BD&%EF%BF%BD", "line 790");
    assert_eq!(n("https://example.com?%E0%A4=&%EF%BF%BD="), "https://example.com/?%EF%BF%BD=&%EF%BF%BD=", "line 791");
    assert_eq!(nopt("https://example.com?foo+bar", Options { sort_query_parameters: false, ..Options::default() }), "https://example.com/?foo%20bar", "line 794");
    assert_eq!(nopt("https://example.com?foo+bar=", Options { sort_query_parameters: false, ..Options::default() }), "https://example.com/?foo%20bar=", "line 795");
    assert_eq!(nopt("https://example.com?foo+bar&baz+qux=", Options { sort_query_parameters: false, ..Options::default() }), "https://example.com/?foo%20bar&baz%20qux=", "line 796");
    assert_eq!(n("https://example.com?café"), "https://example.com/?caf%C3%A9", "line 799");
    assert_eq!(n("https://example.com?café="), "https://example.com/?caf%C3%A9=", "line 800");
    assert_eq!(nopt("https://example.com?café=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?caf%C3%A9", "line 801");
    assert_eq!(nopt("https://example.com?foo%26bar=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?foo%26bar", "line 804");
    assert_eq!(nopt("https://example.com?foo%3Dbar=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?foo%3Dbar", "line 805");
    assert_eq!(n("https://example.com?foo%26bar&utm_source=test"), "https://example.com/?foo%26bar", "line 806");
    assert_eq!(n("https://example.com?foo%2526bar="), "https://example.com/?foo%2526bar=", "line 807");
    assert_eq!(nopt("https://example.com?foo%2526bar=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?foo%2526bar", "line 808");
    assert_eq!(n("https://example.com?a&b=&c=1"), "https://example.com/?a&b=&c=1", "line 811");
    assert_eq!(n("https://example.com?foo%20bar&baz"), "https://example.com/?baz&foo%20bar", "line 812");
    assert_eq!(n("https://example.com?a&a="), "https://example.com/?a&a", "line 815");
    assert_eq!(n("https://example.com?a=&a"), "https://example.com/?a&a", "line 816");
    assert_eq!(n("https://example.com?a&a&a="), "https://example.com/?a&a&a", "line 817");
    assert_eq!(n("https://example.com?a=&a=&a"), "https://example.com/?a&a&a", "line 818");
    assert_eq!(n("https://example.com?key=a=b=c"), "https://example.com/?key=a=b=c", "line 821");
    assert_eq!(n("https://example.com?data=abc=="), "https://example.com/?data=abc==", "line 822");
    assert_eq!(n("https://example.com?key=val%3Due"), "https://example.com/?key=val%3Due", "line 825");
    assert_eq!(n("https://example.com?key=%3D"), "https://example.com/?key=%3D", "line 826");
    assert_eq!(nopt("https://example.com?key=val%3Due", Options { sort_query_parameters: false, ..Options::default() }), "https://example.com/?key=val%3Due", "line 827");
    assert_eq!(n("https://example.com?utm_source=test&utm_medium=web"), "https://example.com", "line 830");
    assert_eq!(nopt("https://example.com?key", Options { remove_query_parameters: RemoveQueryParameters::All, ..Options::default() }), "https://example.com", "line 831");
    assert_eq!(n("https://example.com?=value"), "https://example.com/?=value", "line 834");
    assert_eq!(n("https://example.com?="), "https://example.com/?=", "line 835");
    assert_eq!(nopt("https://example.com?=value", Options { empty_query_value: EmptyQueryValue::Always, ..Options::default() }), "https://example.com/?=value", "line 836");
    assert_eq!(nopt("https://example.com?=value", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?=value", "line 837");
    assert_eq!(nopt("https://example.com?=", Options { empty_query_value: EmptyQueryValue::Always, ..Options::default() }), "https://example.com/?=", "line 838");
    assert_eq!(nopt("https://example.com?=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?=", "line 839");
    assert_eq!(nopt("https://example.com?=&a=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?=&a", "line 840");
    assert_eq!(nopt("https://example.com?=&=", Options { empty_query_value: EmptyQueryValue::Always, ..Options::default() }), "https://example.com/?=&=", "line 841");
    assert_eq!(nopt("https://example.com?=&=", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?=&=", "line 842");
    assert_eq!(nopt("https://example.com?=&a", Options { empty_query_value: EmptyQueryValue::Always, ..Options::default() }), "https://example.com/?=&a=", "line 843");
    assert_eq!(nopt("https://example.com?&", Options { empty_query_value: EmptyQueryValue::Always, ..Options::default() }), "https://example.com", "line 844");
    assert_eq!(nopt("https://example.com?&", Options { empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com", "line 845");
    assert_eq!(nopt("https://example.com?foo&&bar", Options { sort_query_parameters: false, ..Options::default() }), "https://example.com/?foo&bar", "line 848");
    assert_eq!(nopt("https://example.com?foo&&bar", Options { sort_query_parameters: false, empty_query_value: EmptyQueryValue::Always, ..Options::default() }), "https://example.com/?foo=&bar=", "line 849");
    assert_eq!(nopt("https://example.com?foo&&bar", Options { sort_query_parameters: false, empty_query_value: EmptyQueryValue::Never, ..Options::default() }), "https://example.com/?foo&bar", "line 850");
    assert_eq!(nopt("https://example.com?&&", Options { sort_query_parameters: false, ..Options::default() }), "https://example.com", "line 851");
    assert_eq!(n("https://example.com?key#hash"), "https://example.com/?key#hash", "line 854");
    assert_eq!(n("https://example.com?key=#hash"), "https://example.com/?key=#hash", "line 855");
}
