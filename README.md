# indentkit

[![Crates.io](https://img.shields.io/crates/v/indentkit.svg)](https://crates.io/crates/indentkit)
[![Documentation](https://docs.rs/indentkit/badge.svg)](https://docs.rs/indentkit)
[![CI](https://github.com/trananhtung/indentkit/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/indentkit/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/indentkit.svg)](#license)
[![no_std](https://img.shields.io/badge/no__std-yes-brightgreen.svg)](#no_std)

**Detect and convert indentation.** Figure out whether a string is indented with
tabs or *N* spaces — like Node's [`detect-indent`](https://www.npmjs.com/package/detect-indent)
(a Prettier dependency) — and **re-indent** between styles. Zero dependencies,
`#![no_std]`.

```rust
use indentkit::{detect, reindent, indent, IndentStyle};

let code = "fn main() {\n  println!(\"hi\");\n}\n";

// Detect
assert_eq!(detect(code), IndentStyle::Spaces(2));

// Convert 2-space -> tabs
assert_eq!(reindent(code, IndentStyle::Tabs), "fn main() {\n\tprintln!(\"hi\");\n}\n");

// Build indentation for code generation
assert_eq!(indent(2, IndentStyle::Spaces(4)), "        ");
```

## Why indentkit?

The Rust ecosystem's only `detect-indent` port is a stale `v0.1.0` from 2023 and
**detects only**. `indentkit` is a small, well-tested toolkit that also
**converts** between styles — useful for formatters, linters, code generators,
and editor tooling — and runs in `no_std` environments with no dependencies.

| | detects | converts | no_std | deps |
| --- | :-: | :-: | :-: | :-: |
| `indentkit` | ✅ | ✅ | ✅ | 0 |

## Install

```toml
[dependencies]
indentkit = "0.1"
```

## API

| Function | Purpose |
| --- | --- |
| `detect(text) -> IndentStyle` | Detect tabs vs *N* spaces (or `Unknown`) |
| `reindent(text, to) -> String` | Re-indent to a target style (auto-detects source) |
| `indent(level, style) -> String` | Build indentation for a nesting level (codegen) |
| `IndentStyle::unit() -> String` | The one-level indent string (`"\t"`, `"  "`, …) |

`IndentStyle` is `Tabs`, `Spaces(usize)`, or `Unknown`, and implements `Display`
(`"tab"`, `"4 spaces"`, `"unknown"`).

## Behavior notes

- Detection counts each indentation **step** (the change in width between lines)
  by type and picks the most frequent — robust against alignment, continuation,
  and block-comment `*` lines. Ties resolve to the step seen first.
- Blank and whitespace-only lines are ignored when detecting, and emitted as
  clean empty lines when re-indenting (no trailing whitespace).
- `reindent` rewrites **leading** indentation by nesting level; sub-level
  alignment after the indent is preserved as-is. Lines mixing leading tabs and
  spaces are converted on a best-effort basis.
- If the source has no detectable indentation, or the target is `Unknown` (or
  `Spaces(0)`), `reindent` returns the text unchanged.

## no_std

`indentkit` is `#![no_std]` and only needs `alloc`. It builds for bare-metal
targets such as `thumbv7em-none-eabi`.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at
your option.
