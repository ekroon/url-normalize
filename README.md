# url-normalize

Normalize a URL — faithful Rust port of [sindresorhus/normalize-url](https://github.com/sindresorhus/normalize-url) v9.0.0.

Useful when you need to display, store, deduplicate, sort, compare, etc., URLs.

## Usage

```rust
use url_normalize::{normalize_url, Options};

// Basic usage with default options
let result = normalize_url("HTTP://www.Example.com/foo/", &Options::default()).unwrap();
assert_eq!(result, "http://example.com/foo");

// Custom options
let opts = Options {
    strip_hash: true,
    force_https: true,
    ..Options::default()
};
let result = normalize_url("http://www.example.com/path#fragment", &opts).unwrap();
assert_eq!(result, "https://example.com/path");
```

Add to your `Cargo.toml`:

```toml
[dependencies]
url-normalize = "0.1"
```

## What it does

- Lowercases the protocol and hostname
- Removes default ports (80 for HTTP, 443 for HTTPS)
- Resolves relative paths (`/../`, `/./`)
- Removes duplicate slashes in paths
- Removes `www.` from the hostname
- Removes trailing slashes
- Removes URL fragments (optional)
- Removes known tracking parameters (e.g., `utm_*`)
- Sorts query parameters alphabetically
- Decodes unnecessarily encoded URI octets
- Encodes query values like `URLSearchParams`
- Handles international domain names (IDNA/Punycode)
- Handles data URLs
- Handles custom protocol schemes

## Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `default_protocol` | `Protocol` | `Http` | Protocol to use for protocol-relative URLs |
| `custom_protocols` | `Vec<String>` | `[]` | Additional protocols to normalize |
| `normalize_protocol` | `bool` | `true` | Prepend default protocol to `//` URLs |
| `force_http` | `bool` | `false` | Convert HTTPS → HTTP |
| `force_https` | `bool` | `false` | Convert HTTP → HTTPS |
| `strip_authentication` | `bool` | `true` | Remove `user:password@` |
| `strip_hash` | `bool` | `false` | Remove `#fragment` |
| `strip_protocol` | `bool` | `false` | Remove `https://` prefix |
| `strip_text_fragment` | `bool` | `true` | Remove `#:~:text=` fragments |
| `strip_www` | `bool` | `true` | Remove `www.` from hostname |
| `remove_query_parameters` | `RemoveQueryParameters` | UTM filter | Remove matching query params |
| `keep_query_parameters` | `Option<Vec<QueryFilter>>` | `None` | Keep only matching query params |
| `remove_trailing_slash` | `bool` | `true` | Remove trailing `/` from path |
| `remove_single_slash` | `bool` | `true` | Remove sole `/` path |
| `remove_directory_index` | `RemoveDirectoryIndex` | `None` | Remove `index.html` etc. |
| `remove_explicit_port` | `bool` | `false` | Remove all port numbers |
| `sort_query_parameters` | `bool` | `true` | Sort query params by key |
| `empty_query_value` | `EmptyQueryValue` | `Preserve` | How to handle `?key` vs `?key=` |
| `remove_path` | `bool` | `false` | Remove the entire URL path |
| `transform_path` | `Option<Box<dyn Fn(...)>>` | `None` | Custom path transformation |

## Examples

### Remove tracking parameters

```rust
use url_normalize::{normalize_url, Options};

let url = "https://example.com/page?utm_source=google&utm_medium=cpc&id=123";
let result = normalize_url(url, &Options::default()).unwrap();
assert_eq!(result, "https://example.com/page?id=123");
```

### Remove all query parameters

```rust
use url_normalize::{normalize_url, Options, RemoveQueryParameters};

let opts = Options {
    remove_query_parameters: RemoveQueryParameters::All,
    ..Options::default()
};
let result = normalize_url("https://example.com?a=1&b=2", &opts).unwrap();
assert_eq!(result, "https://example.com");
```

### Custom protocols

```rust
use url_normalize::{normalize_url, Options};

let opts = Options {
    custom_protocols: vec!["myapp".to_string()],
    ..Options::default()
};
let result = normalize_url("myapp://www.example.com/path/", &opts).unwrap();
assert_eq!(result, "myapp://example.com/path");
```

### Directory index removal

```rust
use url_normalize::{normalize_url, Options, RemoveDirectoryIndex};

let opts = Options {
    remove_directory_index: RemoveDirectoryIndex::Default,
    ..Options::default()
};
let result = normalize_url("https://example.com/page/index.html", &opts).unwrap();
assert_eq!(result, "https://example.com/page");
```

## Dependencies

Minimal by design:

- [`idna`](https://crates.io/crates/idna) — International domain name encoding (IDNA/Punycode)

Everything else (URL parsing, percent-encoding, pattern matching) is hand-rolled.

## Testing

The implementation is verified against the **actual JavaScript test suite** from `sindresorhus/normalize-url` v9.0.0:

```bash
# Rust tests (175 tests)
cargo test

# Run the actual JS test.js against our implementation
cargo build --features cli --bin normalize_cli
cd tests/js_bridge && npm install && npx ava actual_test.mjs --timeout=60s
```

## Attribution

This is a Rust port of [normalize-url](https://github.com/sindresorhus/normalize-url) by [Sindre Sorhus](https://sindresorhus.com), licensed under the MIT License.

## License

MIT
