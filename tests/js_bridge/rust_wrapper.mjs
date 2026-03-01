/**
 * Wrapper module that replaces `normalize-url` with our Rust CLI.
 * Used by the actual JS test suite to test our implementation.
 */

import { execFileSync } from 'node:child_process';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const CLI_PATH = resolve(__dirname, '../../target/debug/normalize_cli');

function serializeOptions(options) {
  if (options === null || options === undefined) return {};
  const result = {};
  for (const [key, value] of Object.entries(options)) {
    if (value instanceof RegExp) {
      result[key] = { $regex: value.source, $flags: value.flags };
    } else if (Array.isArray(value)) {
      result[key] = value.map(item => {
        if (item instanceof RegExp) {
          return { $regex: item.source, $flags: item.flags };
        }
        return item;
      });
    } else if (typeof value === 'function') {
      // Functions can't be serialized — encode as a named function reference
      // We pass the function source code for simple arrow functions
      result[key] = { $fn: value.toString() };
    } else {
      result[key] = value;
    }
  }
  return result;
}

export default function normalizeUrl(url, options = {}) {
  const input = JSON.stringify({ url, options: serializeOptions(options) }) + '\n';
  let result;
  try {
    result = execFileSync(CLI_PATH, [], {
      input,
      encoding: 'utf8',
      timeout: 5000,
    });
  } catch (e) {
    throw new TypeError(`Rust CLI error: ${e.message}`);
  }

  const parsed = JSON.parse(result.trim());
  if (parsed.err) {
    throw new TypeError(parsed.err);
  }
  return parsed.ok;
}
