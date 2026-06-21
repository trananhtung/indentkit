//! # indentkit — detect and convert indentation
//!
//! Figure out whether a string is indented with tabs or *N* spaces, and
//! re-indent it from one style to another. Like Node's `detect-indent`, but with
//! conversion built in — and zero dependencies, `#![no_std]`.
//!
//! ```
//! use indentkit::{detect, reindent, IndentStyle};
//!
//! let code = "fn main() {\n  println!(\"hi\");\n}\n";
//! assert_eq!(detect(code), IndentStyle::Spaces(2));
//!
//! let tabbed = reindent(code, IndentStyle::Tabs);
//! assert_eq!(tabbed, "fn main() {\n\tprintln!(\"hi\");\n}\n");
//! ```
//!
//! Great for formatters, linters, code generators, and editor tooling.

#![no_std]
#![doc(html_root_url = "https://docs.rs/indentkit/0.1.0")]

extern crate alloc;

use alloc::string::String;
use core::fmt;

mod detect;
mod reindent;

/// The indentation style of a block of text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndentStyle {
    /// Indented with tab characters.
    Tabs,
    /// Indented with the given number of spaces per level.
    Spaces(usize),
    /// No indentation could be detected.
    Unknown,
}

impl IndentStyle {
    /// The string for a single indentation level: `"\t"`, `"    "`, or `""`.
    ///
    /// ```
    /// # use indentkit::IndentStyle;
    /// assert_eq!(IndentStyle::Spaces(2).unit(), "  ");
    /// assert_eq!(IndentStyle::Tabs.unit(), "\t");
    /// ```
    #[must_use]
    pub fn unit(self) -> String {
        match self {
            IndentStyle::Tabs => String::from("\t"),
            IndentStyle::Spaces(n) => " ".repeat(n),
            IndentStyle::Unknown => String::new(),
        }
    }
}

impl fmt::Display for IndentStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndentStyle::Tabs => f.write_str("tab"),
            IndentStyle::Spaces(1) => f.write_str("1 space"),
            IndentStyle::Spaces(n) => write!(f, "{n} spaces"),
            IndentStyle::Unknown => f.write_str("unknown"),
        }
    }
}

/// Detect the indentation style of `text`.
///
/// Returns [`IndentStyle::Unknown`] when no indentation is present.
///
/// ```
/// # use indentkit::{detect, IndentStyle};
/// assert_eq!(detect("a\n    b\n"), IndentStyle::Spaces(4));
/// ```
#[must_use]
pub fn detect(text: &str) -> IndentStyle {
    detect::detect(text)
}

/// Re-indent `text` to the target style `to`, auto-detecting the source style.
///
/// If the source has no detectable indentation, or `to` is
/// [`IndentStyle::Unknown`], the text is returned unchanged.
///
/// ```
/// # use indentkit::{reindent, IndentStyle};
/// assert_eq!(reindent("a\n  b\n", IndentStyle::Tabs), "a\n\tb\n");
/// ```
#[must_use]
pub fn reindent(text: &str, to: IndentStyle) -> String {
    reindent::reindent(text, to)
}

/// Build the indentation string for `level` nesting levels in `style`.
///
/// ```
/// # use indentkit::{indent, IndentStyle};
/// assert_eq!(indent(2, IndentStyle::Spaces(4)), "        ");
/// ```
#[must_use]
pub fn indent(level: usize, style: IndentStyle) -> String {
    style.unit().repeat(level)
}
