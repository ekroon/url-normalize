/**
 * Cross-language parity test harness.
 *
 * Runs hundreds of URL + options combinations through BOTH:
 *   1. The JS `normalize-url` package (ground truth)
 *   2. Our Rust `normalize_cli` binary (via stdin/stdout JSON lines)
 *
 * Reports any mismatches. This is the gold standard: we test against
 * the actual JS implementation, not our interpretation of the docs.
 *
 * Usage:
 *   cd tests/js_bridge && npm install && node parity_test.mjs [path/to/normalize_cli]
 */

import { spawn } from 'node:child_process';
import { createInterface } from 'node:readline';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import normalizeUrl from 'normalize-url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const cliBinary = process.argv[2] || resolve(__dirname, '../../target/debug/normalize_cli');

// ── Test case definitions ──────────────────────────────────────────
// Each test case: { input, options?, expectThrow? }
// We run normalizeUrl(input, options) in both JS and Rust, then compare.

function generateTestCases() {
  const cases = [];
  let id = 0;

  function add(input, options, desc) {
    cases.push({ id: ++id, input, options: options || {}, desc: desc || '' });
  }

  // ══════════════════════════════════════════════════════════════════
  // All assertions from sindresorhus/normalize-url test.js v9.0.0
  // ══════════════════════════════════════════════════════════════════

  // ── main ──
  add('sindresorhus.com');
  add('sindresorhus.com ');
  add('sindresorhus.com.');
  add('SindreSorhus.com');
  add('HTTP://sindresorhus.com');
  add('//sindresorhus.com');
  add('http://sindresorhus.com');
  add('http://sindresorhus.com:80');
  add('https://sindresorhus.com:443');
  add('http://www.sindresorhus.com');
  add('www.com');
  add('http://www.www.sindresorhus.com');
  add('www.sindresorhus.com');
  add('http://sindresorhus.com/foo/');
  add('sindresorhus.com/?foo=bar baz');
  add('https://foo.com/https://bar.com');
  add('https://foo.com/https://bar.com/http://baz.com');
  add('http://sindresorhus.com:5000');
  add('http://sindresorhus.com/foo');
  add('http://sindresorhus.com/foo/bar/../baz');
  add('http://sindresorhus.com/foo/bar/./baz');
  add('sindresorhus.com/foo/bar/../baz');
  add('http://sindresorhus.com/#');
  add('http://sindresorhus.com/?');
  add('http://sindresorhus.com/?#');
  add('http://sindresorhus.com/?a=b#');
  add('http://êxample.com');
  add('http://xn--xample-9ua.com');
  add('http://sindresorhus.com/?');
  add('êxample.com');
  add('http://sindresorhus.com/?b=bar&a=foo');
  add('http://sindresorhus.com/?foo=bar*|<>:"');
  add('http://sindresorhus.com', { defaultProtocol: 'https:' });
  add('https://sindresorhus.com');
  add('//sindresorhus.com', { normalizeProtocol: false });
  add('//sindresorhus.com:80', { normalizeProtocol: false });
  add('http://sindresorhus.com/foo#bar');
  add('http://sindresorhus.com/foo#bar', { stripHash: true });
  add('http://sindresorhus.com/foo#bar:~:text=hello');
  add('http://sindresorhus.com/foo#bar:~:text=hello', { stripTextFragment: false });
  add('http://sindresorhus.com/foo#:~:text=hello');
  add('http://sindresorhus.com');
  add('http://sindresorhus.com/', { removeSingleSlash: false });
  add('http://sindresorhus.com', { removeSingleSlash: false });
  add('http://sindresorhus.com/redirect?url=http%3A%2F%2Fsindresorhus.com%2Ffoo%23bar');
  add('sindresorhus.com/?foo=bar%20baz');
  add('ftp://sindresorhus.com:21');

  // ── removeQueryParameters ──
  add('http://sindresorhus.com?foo=bar&utm_medium=test');
  add('http://sindresorhus.com?foo=bar&utm_medium=test&utm_source=test');
  add('http://sindresorhus.com?foo=bar');
  add('http://sindresorhus.com?foo=bar&utm_medium=test', { removeQueryParameters: false });
  add('http://sindresorhus.com?foo=bar&ref=test_ref', { removeQueryParameters: ['ref'] });
  add('http://sindresorhus.com?foo=bar&utm_medium=test', { removeQueryParameters: ['utm_medium'] });
  add('http://sindresorhus.com?foo=bar', { removeQueryParameters: true });
  add('http://sindresorhus.com?foo=bar&baz=qux', { removeQueryParameters: true });
  // Regex-based removal — we skip regexes in Rust (only exact match supported via CLI bridge)
  // add('http://sindresorhus.com?foo=bar&utm_medium=test', { removeQueryParameters: [/utm_\w+/i] });

  // ── keepQueryParameters ──
  add('http://sindresorhus.com?foo=bar&utm_medium=test&ref=test_ref', { keepQueryParameters: ['foo'] });
  add('http://sindresorhus.com?foo=bar&baz=qux&utm_medium=test', { keepQueryParameters: ['foo', 'baz'] });
  add('http://sindresorhus.com?foo=bar&baz=qux', { keepQueryParameters: [] });
  // Regex-based keep — skip in CLI bridge

  // ── forceHttp ──
  add('https://sindresorhus.com', { forceHttp: true });
  add('http://sindresorhus.com', { forceHttp: true });
  add('https://sindresorhus.com/foo', { forceHttp: true });

  // ── forceHttps ──
  add('http://sindresorhus.com', { forceHttps: true });
  add('https://sindresorhus.com', { forceHttps: true });
  add('http://sindresorhus.com/foo', { forceHttps: true });

  // ── stripAuthentication ──
  add('http://user:password@sindresorhus.com');
  add('http://user:password@sindresorhus.com/@user');
  add('http://user@sindresorhus.com');
  add('http://user:password@sindresorhus.com', { stripAuthentication: false });

  // ── stripProtocol ──
  add('http://sindresorhus.com', { stripProtocol: true });
  add('https://sindresorhus.com', { stripProtocol: true });
  add('//sindresorhus.com', { stripProtocol: true });

  // ── stripWWW ──
  add('http://www.sindresorhus.com', { stripWWW: false });
  add('http://www.êxample.com', { stripWWW: false });
  add('http://www.www.sindresorhus.com');
  add('http://www.sindresorhus.com');
  add('http://www.sindresorhus.com', { stripWWW: true });
  add('http://www.xn--xample-9ua.com', { stripWWW: true });
  add('http://www.example.a');
  add('http://www.example.a.b');
  add('http://www.example.foo');

  // ── removeTrailingSlash ──
  add('http://sindresorhus.com/', { removeTrailingSlash: false });
  add('http://sindresorhus.com/foo/', { removeTrailingSlash: false });

  // ── removeSingleSlash ──
  add('https://sindresorhus.com/', { removeSingleSlash: false });
  add('https://sindresorhus.com', { removeSingleSlash: false });
  add('https://sindresorhus.com/foo', { removeSingleSlash: false });

  // ── removeDirectoryIndex ──
  add('http://sindresorhus.com/index.html', { removeDirectoryIndex: true });
  add('http://sindresorhus.com/index.htm', { removeDirectoryIndex: true });
  add('http://sindresorhus.com/index.php', { removeDirectoryIndex: true });
  add('http://sindresorhus.com/path/index.html', { removeDirectoryIndex: true });
  add('http://sindresorhus.com/path/index.htm', { removeDirectoryIndex: true });
  add('http://sindresorhus.com/path/index.php', { removeDirectoryIndex: true });
  add('http://sindresorhus.com/index.html', { removeDirectoryIndex: false });
  add('http://sindresorhus.com/path/not-index.html', { removeDirectoryIndex: true });
  add('http://sindresorhus.com/path/index.', { removeDirectoryIndex: true });

  // ── removeExplicitPort ──
  add('http://sindresorhus.com:8080', { removeExplicitPort: true });
  add('https://sindresorhus.com:443', { removeExplicitPort: true });
  add('http://sindresorhus.com:80', { removeExplicitPort: true });

  // ── sortQueryParameters ──
  add('http://sindresorhus.com?a=Z&b=Y&c=X&d=W');
  add('http://sindresorhus.com?b=Y&a=Z&c=X&d=W');
  add('http://sindresorhus.com?a=Z&b=Y&c=X&d=W', { sortQueryParameters: false });
  add('http://sindresorhus.com?b=Y&a=Z', { sortQueryParameters: false });
  add('http://sindresorhus.com?a=%2F&b=%25&c=%3A', { sortQueryParameters: true });

  // ── data URLs ──
  add('data:text/html;charset=utf-8;foo=bar;base64,PGh0bWw+');
  add('data:text/html;base64,PGh0bWw+');
  add('data:,Hello%2C%20World!');
  add('data:text/plain,Hello%2C%20World!');
  add('data:text/html,%3Ch1%3EHello%2C%20World!%3C%2Fh1%3E');
  add('data:TEXT/HTML,%3Ch1%3EHello%2C%20World!%3C%2Fh1%3E');
  add('data:text/plain;base64,SGVsbG8=');
  add('data:text/plain;base64,SGVsbG8=#hash');
  add('data:text/plain;base64,SGVsbG8=#');
  add('data:text/plain;base64,SGVsbG8=', { stripHash: true });

  // ── customProtocols ──
  const customOpts = { customProtocols: ['sindre'] };
  add('sindre://www.sorhus.com', customOpts);
  add('SINDRE://www.sorhus.com', customOpts);
  add('sindre://user:password@www.sorhus.com', customOpts);
  add('sindre://sorhus.com', { stripProtocol: true });
  add('sindre://sorhus.com:8080', customOpts);
  add('sindre://sorhus.com:8080/foo', { ...customOpts, removeExplicitPort: true });
  add('sindre://www.sorhus.com', { customProtocols: ['SINDRE'] });
  add('sindre://www.sorhus.com', { customProtocols: ['Sindre'] });
  add('sindre://www.sorhus.com', { customProtocols: [' sindre '] });
  add('sindre://www.sorhus.com', { customProtocols: ['sindre', 123] });
  add('sindre://www.sorhus.com', { customProtocols: 'sindre' });
  add('sindre://sorhus.com/foo/../bar', customOpts);
  add('sindre://sorhus.com/foo/./bar', customOpts);
  add('sindre://user:password@www.sorhus.com', { ...customOpts, stripAuthentication: false });
  add('sindre://www.sorhus.com', { ...customOpts, stripWWW: false });
  add('sindre://sorhus.com/foo/', { ...customOpts, removeTrailingSlash: false });
  add('sindre://sorhus.com?foo=bar', { ...customOpts, removeQueryParameters: true });
  add('sindre://sorhus.com?foo=bar&baz=qux', { ...customOpts, keepQueryParameters: ['foo'] });
  add('http://www.sorhus.com', customOpts);
  add('https://www.sorhus.com', customOpts);

  // ── encoded backslashes ──
  add('https://foo.com/some%5Bthing%5Celse/that-is%40great@coding');
  add('https://foo.com/something%5Celse/great');

  // ── removePath ──
  add('http://sindresorhus.com/foo/bar', { removePath: true });
  add('http://sindresorhus.com/foo/bar?baz=qux', { removePath: true });
  add('http://sindresorhus.com', { removePath: true });

  // ── backslash to forward slash ──
  add('http://sindresorhus.com\\foo\\bar');
  add('http://sindresorhus.com\\\\foo\\\\bar');

  // ── relative URLs ──
  add('./relative/path/');
  add('../relative/path/');

  // ══════════════════════════════════════════════════════════════════
  // Additional edge cases beyond the JS test suite
  // ══════════════════════════════════════════════════════════════════

  // Edge cases: query encoding
  add('http://example.com?key=hello%20world');
  add('http://example.com?key=hello+world');
  add('http://example.com?a=%C3%A9&b=2');
  add('http://example.com?key=<script>');
  add('http://example.com?key=a"b');
  add('http://example.com?utm_source=google&utm_medium=cpc&ref=abc');
  add('http://example.com?a=1&a=2&b=3');

  // Edge cases: domain normalization
  add('HTTP://WWW.EXAMPLE.COM/PATH');
  add('http://www.example.com.');
  add('http://example.com:80/path?query=1#hash');
  add('https://example.com:443/path');

  // Edge cases: paths
  add('http://example.com/a/b/c/../d/./e');
  add('http://example.com/a//b///c');
  add('http://example.com/');
  add('http://example.com');

  // Edge cases: fragments
  add('http://example.com/path#');
  add('http://example.com/path#fragment');
  add('http://example.com/path#:~:text=some%20text');

  // Edge cases: authentication
  add('http://user@example.com/path');
  add('http://user:pass@example.com/path');
  add('http://user:pass@example.com/path', { stripAuthentication: false });

  // Edge cases: protocol handling
  add('//example.com/path');
  add('//example.com/path', { normalizeProtocol: false });
  add('http://example.com', { forceHttps: true });
  add('https://example.com', { forceHttp: true });

  // Edge cases: combined options
  add('http://www.example.com/foo/?bar=baz#hash', {
    stripHash: true,
    stripWWW: true,
    removeTrailingSlash: true,
  });
  add('https://www.example.com:443/index.html?utm_source=test', {
    removeDirectoryIndex: true,
    stripWWW: true,
  });

  // Edge cases: unicode domains
  add('http://例え.jp/path');
  add('http://münchen.de');
  add('http://www.münchen.de');

  // Edge cases: query sorting with encoded chars
  add('http://example.com?z=%26&a=%3D&m=%3F');
  add('http://example.com?key=%2F%2F&other=val');

  // Edge cases: data URLs with various MIME types
  add('data:application/json,{"key":"value"}');
  add('data:image/png;base64,iVBOR');
  add('data:;base64,SGVsbG8=');

  // Edge cases: port handling
  add('http://example.com:8080/path', { removeExplicitPort: true });
  add('https://example.com:8443/path', { removeExplicitPort: true });
  add('http://example.com:80');
  add('https://example.com:443');

  // Edge cases: multiple option combinations
  add('HTTP://WWW.EXAMPLE.COM:80/index.html?utm_source=test&b=2&a=1#hash', {
    stripHash: true,
    stripWWW: true,
    removeDirectoryIndex: true,
    sortQueryParameters: true,
  });

  return cases;
}

// ── Rust CLI bridge ────────────────────────────────────────────────

async function runRustCli(cases) {
  return new Promise((resolve, reject) => {
    const proc = spawn(cliBinary, [], {
      stdio: ['pipe', 'pipe', 'pipe'],
    });

    const results = [];
    const rl = createInterface({ input: proc.stdout });

    rl.on('line', (line) => {
      try {
        results.push(JSON.parse(line));
      } catch {
        results.push({ err: `unparseable: ${line}` });
      }
    });

    let stderr = '';
    proc.stderr.on('data', (d) => { stderr += d; });

    proc.on('close', (code) => {
      if (results.length !== cases.length) {
        reject(new Error(`Expected ${cases.length} results, got ${results.length}. stderr: ${stderr}`));
      } else {
        resolve(results);
      }
    });

    proc.on('error', (err) => {
      reject(new Error(`Failed to spawn ${cliBinary}: ${err.message}`));
    });

    // Send all test cases
    for (const c of cases) {
      proc.stdin.write(JSON.stringify({ url: c.input, options: c.options }) + '\n');
    }
    proc.stdin.end();
  });
}

// ── Main ───────────────────────────────────────────────────────────

async function main() {
  const cases = generateTestCases();
  console.log(`\n🔍 Running ${cases.length} parity tests...\n`);

  // Run JS
  const jsResults = cases.map((c) => {
    try {
      return { ok: normalizeUrl(c.input, c.options) };
    } catch (e) {
      return { err: e.message };
    }
  });

  // Run Rust
  let rustResults;
  try {
    rustResults = await runRustCli(cases);
  } catch (e) {
    console.error(`❌ Failed to run Rust CLI: ${e.message}`);
    process.exit(1);
  }

  // Compare
  let passed = 0;
  let failed = 0;
  const failures = [];

  for (let i = 0; i < cases.length; i++) {
    const c = cases[i];
    const js = jsResults[i];
    const rs = rustResults[i];

    const jsIsErr = 'err' in js;
    const rsIsErr = 'err' in rs;

    if (jsIsErr && rsIsErr) {
      // Both threw — match
      passed++;
    } else if (!jsIsErr && !rsIsErr && js.ok === rs.ok) {
      passed++;
    } else {
      failed++;
      failures.push({
        id: c.id,
        input: c.input,
        options: c.options,
        js: jsIsErr ? `THROW: ${js.err}` : js.ok,
        rust: rsIsErr ? `THROW: ${rs.err}` : rs.ok,
      });
    }
  }

  // Report
  if (failures.length > 0) {
    console.log(`❌ ${failed} FAILURES:\n`);
    for (const f of failures) {
      console.log(`  #${f.id}: normalizeUrl(${JSON.stringify(f.input)}, ${JSON.stringify(f.options)})`);
      console.log(`    JS:   ${f.js}`);
      console.log(`    Rust: ${f.rust}`);
      console.log();
    }
  }

  console.log(`\n✅ ${passed} passed, ❌ ${failed} failed (${cases.length} total)\n`);
  process.exit(failed > 0 ? 1 : 0);
}

main();
