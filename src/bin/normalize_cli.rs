//! Thin CLI wrapper for cross-language parity testing.
//! Reads JSON lines from stdin, outputs normalized URLs to stdout.
//!
//! Input format (one JSON per line):
//!   {"url": "...", "options": { ... }}
//!
//! Output format (one JSON per line):
//!   {"ok": "normalized_url"} or {"err": "error message"}

use url_normalize::*;
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }

        let parsed: serde_json::Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => {
                let _ = writeln!(out, r#"{{"err":"parse error: {e}"}}"#);
                continue;
            }
        };

        let url = parsed["url"].as_str().unwrap_or("");
        let opts_val = &parsed["options"];
        let opts = json_to_options(opts_val);

        match normalize_url(url, &opts) {
            Ok(result) => {
                let json_str = serde_json::to_string(&result).unwrap();
                let _ = writeln!(out, r#"{{"ok":{json_str}}}"#);
            }
            Err(e) => {
                // Use JS-style option names in error messages for test compatibility
                let msg = e.to_string().replace("force_http", "forceHttp").replace("force_https", "forceHttps");
                let _ = writeln!(
                    out,
                    r#"{{"err":{}}}"#,
                    serde_json::to_string(&msg).unwrap()
                );
            }
        }
    }
}

/// Convert a JSON regex descriptor `{"$regex": "pattern", "$flags": "flags"}` to a Rust predicate.
fn json_to_filter(v: &serde_json::Value) -> Option<QueryFilter> {
    if let Some(s) = v.as_str() {
        return Some(QueryFilter::Exact(s.to_string()));
    }
    if let Some(obj) = v.as_object() {
        if let Some(pattern) = obj.get("$regex").and_then(|v| v.as_str()) {
            let flags = obj.get("$flags").and_then(|v| v.as_str()).unwrap_or("");
            let case_insensitive = flags.contains('i');
            let pattern = pattern.to_string();
            return Some(QueryFilter::Predicate(Box::new(move |name: &str| {
                simple_regex_match(name, &pattern, case_insensitive)
            })));
        }
    }
    // Skip non-string, non-regex values (e.g. numbers)
    None
}

/// Minimal regex matcher supporting: `^prefix`, `suffix$`, `^exact$`, and plain substring.
/// Also handles `\w+` as `[a-zA-Z0-9_]+` and `\.` as literal `.`.
fn simple_regex_match(input: &str, pattern: &str, case_insensitive: bool) -> bool {
    let input_check: String;
    let pattern_check: String;

    let (inp, pat) = if case_insensitive {
        input_check = input.to_lowercase();
        pattern_check = pattern.to_lowercase();
        (input_check.as_str(), pattern_check.as_str())
    } else {
        (input, pattern)
    };

    // Expand regex pattern to a simpler form
    let expanded = expand_regex(pat);

    let anchored_start = expanded.starts_with('^');
    let anchored_end = expanded.ends_with('$') && !expanded.ends_with("\\$");

    let core = expanded
        .strip_prefix('^')
        .unwrap_or(&expanded);
    let core = core
        .strip_suffix('$')
        .unwrap_or(core);

    if anchored_start && anchored_end {
        regex_core_match(inp, core)
    } else if anchored_start {
        regex_core_match_prefix(inp, core)
    } else if anchored_end {
        regex_core_match_suffix(inp, core)
    } else {
        // Substring match — try at every position
        for i in 0..=inp.len() {
            if regex_core_match_prefix(&inp[i..], core) {
                return true;
            }
        }
        false
    }
}

fn expand_regex(pattern: &str) -> String {
    pattern.to_string()
}

/// Match `core` regex pattern against the full input string.
fn regex_core_match(input: &str, core: &str) -> bool {
    regex_core_match_inner(input, core, true)
}

/// Match `core` regex pattern as a prefix of input.
fn regex_core_match_prefix(input: &str, core: &str) -> bool {
    regex_core_match_inner(input, core, false)
}

/// Match `core` regex pattern as a suffix of input.
fn regex_core_match_suffix(input: &str, core: &str) -> bool {
    for i in 0..=input.len() {
        if regex_core_match_inner(&input[i..], core, true) {
            return true;
        }
    }
    false
}

/// Recursive regex matcher for simple patterns.
fn regex_core_match_inner(input: &str, pattern: &str, must_consume_all: bool) -> bool {
    if pattern.is_empty() {
        return if must_consume_all { input.is_empty() } else { true };
    }

    let pat_bytes = pattern.as_bytes();

    // Handle escape sequences
    if pat_bytes[0] == b'\\' && pattern.len() >= 2 {
        let escaped = pat_bytes[1];
        let rest_pat = &pattern[2..];

        match escaped {
            b'w' => {
                // \w+ or \w
                if rest_pat.starts_with('+') {
                    // \w+ : match one or more word chars (greedy)
                    let rest_pat = &rest_pat[1..];
                    let word_len = input.bytes().take_while(|b| b.is_ascii_alphanumeric() || *b == b'_').count();
                    if word_len == 0 {
                        return false;
                    }
                    // Try greedy then shorter
                    for len in (1..=word_len).rev() {
                        if regex_core_match_inner(&input[len..], rest_pat, must_consume_all) {
                            return true;
                        }
                    }
                    return false;
                } else {
                    // \w : match exactly one word char
                    if let Some(first) = input.bytes().next() {
                        if first.is_ascii_alphanumeric() || first == b'_' {
                            return regex_core_match_inner(&input[1..], rest_pat, must_consume_all);
                        }
                    }
                    return false;
                }
            }
            b'd' => {
                if let Some(first) = input.bytes().next() {
                    if first.is_ascii_digit() {
                        return regex_core_match_inner(&input[1..], rest_pat, must_consume_all);
                    }
                }
                return false;
            }
            _ => {
                // Literal escape: \. matches .
                if let Some(first) = input.bytes().next() {
                    if first == escaped {
                        return regex_core_match_inner(&input[1..], rest_pat, must_consume_all);
                    }
                }
                return false;
            }
        }
    }

    // Handle character classes [a-z]
    if pat_bytes[0] == b'[' {
        if let Some(close) = pattern.find(']') {
            let class_content = &pattern[1..close];
            let rest_pat = &pattern[close + 1..];
            // Handle + quantifier
            let (rest_pat, is_plus) = if rest_pat.starts_with('+') {
                (&rest_pat[1..], true)
            } else {
                (rest_pat, false)
            };

            if let Some(first) = input.bytes().next() {
                if char_class_matches(first, class_content) {
                    if is_plus {
                        // Greedy match
                        let match_len = input.bytes().take_while(|b| char_class_matches(*b, class_content)).count();
                        for len in (1..=match_len).rev() {
                            if regex_core_match_inner(&input[len..], rest_pat, must_consume_all) {
                                return true;
                            }
                        }
                        return false;
                    }
                    return regex_core_match_inner(&input[1..], rest_pat, must_consume_all);
                }
            }
            return false;
        }
    }

    // Handle . (any char)
    if pat_bytes[0] == b'.' {
        let rest_pat = &pattern[1..];
        if let Some(first) = input.chars().next() {
            return regex_core_match_inner(&input[first.len_utf8()..], rest_pat, must_consume_all);
        }
        return false;
    }

    // Literal character
    if let Some(first) = input.bytes().next() {
        if first == pat_bytes[0] {
            return regex_core_match_inner(&input[1..], &pattern[1..], must_consume_all);
        }
    }
    false
}

fn char_class_matches(byte: u8, class_content: &str) -> bool {
    let chars: Vec<u8> = class_content.bytes().collect();
    let mut i = 0;
    while i < chars.len() {
        if i + 2 < chars.len() && chars[i + 1] == b'-' {
            if byte >= chars[i] && byte <= chars[i + 2] {
                return true;
            }
            i += 3;
        } else {
            if byte == chars[i] {
                return true;
            }
            i += 1;
        }
    }
    false
}

fn json_to_options(v: &serde_json::Value) -> Options {
    let mut opts = Options::default();

    if let Some(dp) = v.get("defaultProtocol").and_then(|v| v.as_str()) {
        opts.default_protocol = match dp {
            "https:" | "https" => Protocol::Https,
            _ => Protocol::Http,
        };
    }

    if let Some(arr) = v.get("customProtocols").and_then(|v| v.as_array()) {
        opts.custom_protocols = arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
    }

    if let Some(b) = v.get("normalizeProtocol").and_then(|v| v.as_bool()) {
        opts.normalize_protocol = b;
    }
    if let Some(b) = v.get("forceHttp").and_then(|v| v.as_bool()) {
        opts.force_http = b;
    }
    if let Some(b) = v.get("forceHttps").and_then(|v| v.as_bool()) {
        opts.force_https = b;
    }
    if let Some(b) = v.get("stripAuthentication").and_then(|v| v.as_bool()) {
        opts.strip_authentication = b;
    }
    if let Some(b) = v.get("stripHash").and_then(|v| v.as_bool()) {
        opts.strip_hash = b;
    }
    if let Some(b) = v.get("stripProtocol").and_then(|v| v.as_bool()) {
        opts.strip_protocol = b;
    }
    if let Some(b) = v.get("stripTextFragment").and_then(|v| v.as_bool()) {
        opts.strip_text_fragment = b;
    }
    if let Some(b) = v.get("stripWWW").and_then(|v| v.as_bool()) {
        opts.strip_www = b;
    }
    if let Some(b) = v.get("removeTrailingSlash").and_then(|v| v.as_bool()) {
        opts.remove_trailing_slash = b;
    }
    if let Some(b) = v.get("removeSingleSlash").and_then(|v| v.as_bool()) {
        opts.remove_single_slash = b;
    }
    if let Some(b) = v.get("removeExplicitPort").and_then(|v| v.as_bool()) {
        opts.remove_explicit_port = b;
    }
    if let Some(b) = v.get("sortQueryParameters").and_then(|v| v.as_bool()) {
        opts.sort_query_parameters = b;
    }
    if let Some(b) = v.get("removePath").and_then(|v| v.as_bool()) {
        opts.remove_path = b;
    }

    // emptyQueryValue: "preserve" | "always" | "never"
    if let Some(eqv) = v.get("emptyQueryValue").and_then(|v| v.as_str()) {
        opts.empty_query_value = match eqv {
            "always" => EmptyQueryValue::Always,
            "never" => EmptyQueryValue::Never,
            _ => EmptyQueryValue::Preserve,
        };
    }

    // removeQueryParameters: true | false | array of strings/regexes
    if let Some(rqp) = v.get("removeQueryParameters") {
        if rqp.is_boolean() {
            if rqp.as_bool().unwrap() {
                opts.remove_query_parameters = RemoveQueryParameters::All;
            } else {
                opts.remove_query_parameters = RemoveQueryParameters::None;
            }
        } else if let Some(arr) = rqp.as_array() {
            let filters: Vec<QueryFilter> = arr.iter().filter_map(json_to_filter).collect();
            opts.remove_query_parameters = RemoveQueryParameters::List(filters);
        }
    }

    // keepQueryParameters: array of strings/regexes
    if let Some(kqp) = v.get("keepQueryParameters") {
        if let Some(arr) = kqp.as_array() {
            let filters: Vec<QueryFilter> = arr.iter().filter_map(json_to_filter).collect();
            opts.keep_query_parameters = Some(filters);
        }
    }

    // removeDirectoryIndex: true | array of regexes/strings
    if let Some(rdi) = v.get("removeDirectoryIndex") {
        if rdi.is_boolean() && rdi.as_bool().unwrap() {
            opts.remove_directory_index = RemoveDirectoryIndex::Default;
        } else if let Some(arr) = rdi.as_array() {
            let filters: Vec<QueryFilter> = arr.iter().filter_map(json_to_filter).collect();
            opts.remove_directory_index = RemoveDirectoryIndex::List(filters);
        }
    }

    // transformPath: function (passed as {$fn: "source"})
    if let Some(tp) = v.get("transformPath") {
        if let Some(obj) = tp.as_object() {
            if let Some(fn_src) = obj.get("$fn").and_then(|v| v.as_str()) {
                let fn_src = fn_src.to_string();
                opts.transform_path = Some(parse_transform_path(&fn_src));
            }
        }
    }

    opts
}

/// Parse simple JS arrow functions into Rust closures.
fn parse_transform_path(src: &str) -> Box<dyn Fn(Vec<String>) -> Vec<String>> {
    let src = src.trim();

    // "() => []" or "() => null" or "() => undefined"
    if src.starts_with("()") && (src.contains("=> []") || src.contains("=> null") || src.contains("=> undefined")) {
        return Box::new(|_: Vec<String>| vec![]);
    }

    // "pathComponents => { if (pathComponents[0] === 'api') { return pathComponents.slice(0, 1); } return []; }"
    if src.contains("if (") && src.contains("[0] ===") && src.contains(".slice(") && src.contains("return []") {
        // Extract the condition value and slice args
        if let (Some(cond_val), Some(slice_args)) = (extract_if_condition_value(src), extract_slice_args(src)) {
            let (start, end) = slice_args;
            return Box::new(move |components: Vec<String>| {
                if components.first().map(|s| s.as_str()) == Some(cond_val.as_str()) {
                    let len = components.len() as i64;
                    let s = normalize_index(start, len);
                    let e = if let Some(e) = end { normalize_index(e, len) } else { len as usize };
                    if s >= e { return vec![]; }
                    components[s..e].to_vec()
                } else {
                    vec![]
                }
            });
        }
    }

    // "pathComponents => pathComponents.slice(0, 1)"
    if src.contains(".slice(") {
        if let Some(args) = extract_slice_args(src) {
            let (start, end) = args;
            return Box::new(move |components: Vec<String>| {
                let len = components.len() as i64;
                let s = normalize_index(start, len);
                let e = if let Some(e) = end { normalize_index(e, len) } else { len as usize };
                if s >= e { return vec![]; }
                components[s..e].to_vec()
            });
        }
    }

    // "pathComponents => pathComponents.filter(c => c !== 'admin')"
    if src.contains(".filter(") {
        if let Some(exclude) = extract_filter_exclude(src) {
            return Box::new(move |components: Vec<String>| {
                components.into_iter().filter(|c| c != &exclude).collect()
            });
        }
    }

    // "pathComponents => [...pathComponents, 'v2']"
    if src.contains("[...") && src.contains(", '") {
        if let Some(suffix) = extract_spread_append(src) {
            return Box::new(move |mut components: Vec<String>| {
                components.push(suffix.clone());
                components
            });
        }
    }

    // "pathComponents => pathComponents.map(c => c.toLowerCase())"
    if src.contains(".map(") && src.contains("toLowerCase") {
        return Box::new(|components: Vec<String>| {
            components.into_iter().map(|c| c.to_lowercase()).collect()
        });
    }

    // "_ => ['custom', 'path']"
    if src.contains("=> [") {
        if let Some(items) = extract_array_literal(src) {
            return Box::new(move |_: Vec<String>| items.clone());
        }
    }

    // Fallback: identity
    Box::new(|components: Vec<String>| components)
}

fn normalize_index(idx: i64, len: i64) -> usize {
    if idx < 0 {
        (len + idx).max(0) as usize
    } else {
        idx.min(len) as usize
    }
}

fn extract_slice_args(src: &str) -> Option<(i64, Option<i64>)> {
    let slice_start = src.find(".slice(")?;
    let args_start = slice_start + 7;
    let args_end = src[args_start..].find(')')? + args_start;
    let args_str = &src[args_start..args_end];
    let parts: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();
    let start: i64 = parts.first()?.parse().ok()?;
    let end: Option<i64> = parts.get(1).and_then(|s| s.parse().ok());
    Some((start, end))
}

fn extract_filter_exclude(src: &str) -> Option<String> {
    // Look for pattern: c !== 'value' or c !== "value"
    let idx = src.find("!==")?;
    let after = src[idx + 3..].trim();
    let quote = after.chars().next()?;
    if quote != '\'' && quote != '"' { return None; }
    let end = after[1..].find(quote)?;
    Some(after[1..1 + end].to_string())
}

fn extract_if_condition_value(src: &str) -> Option<String> {
    // Extract value from: if (pathComponents[0] === 'api')
    let idx = src.find("=== '")?;
    let start = idx + 5;
    let end = src[start..].find('\'')?;
    Some(src[start..start + end].to_string())
}

fn extract_spread_append(src: &str) -> Option<String> {
    let idx = src.find(", '")?;
    let start = idx + 3;
    let end = src[start..].find('\'')?;
    Some(src[start..start + end].to_string())
}

fn extract_array_literal(src: &str) -> Option<Vec<String>> {
    let idx = src.find("=> [")?;
    let start = idx + 4;
    let end = src[start..].find(']')?;
    let content = &src[start..start + end];
    let items: Vec<String> = content
        .split(',')
        .filter_map(|s| {
            let s = s.trim();
            if (s.starts_with('\'') && s.ends_with('\'')) || (s.starts_with('"') && s.ends_with('"')) {
                Some(s[1..s.len() - 1].to_string())
            } else {
                None
            }
        })
        .collect();
    Some(items)
}
