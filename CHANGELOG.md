# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-21

### Added

- Initial release.
- `detect` — detect a string's indentation style (tabs vs *N* spaces).
- `reindent` — convert text between indentation styles (auto-detects source).
- `indent` — build the indentation string for a nesting level.
- `IndentStyle` enum (`Tabs` / `Spaces(n)` / `Unknown`) with `unit()` and `Display`.
- `#![no_std]` support (requires `alloc`); zero dependencies.

[0.1.0]: https://github.com/trananhtung/indentkit/releases/tag/v0.1.0
