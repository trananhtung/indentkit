//! Re-indentation between styles.

use alloc::string::String;

use crate::detect::detect;
use crate::IndentStyle;

pub(crate) fn reindent(text: &str, to: IndentStyle) -> String {
    let from = detect(text);
    let from_unit = match from {
        IndentStyle::Tabs => 1, // one tab == one level
        IndentStyle::Spaces(n) if n > 0 => n,
        _ => return String::from(text), // unknown source: nothing to convert
    };
    // A zero-width or unknown target has no meaningful indentation to apply.
    let target = match to {
        IndentStyle::Unknown | IndentStyle::Spaces(0) => return String::from(text),
        other => other.unit(),
    };

    let mut out = String::new();
    let mut lines = text.split('\n').peekable();
    while let Some(line) = lines.next() {
        if line.is_empty() || line.trim().is_empty() {
            // Blank / whitespace-only line: emit a clean empty line (no indent).
        } else {
            let (level, rest) = split_indent(line, from, from_unit);
            for _ in 0..level {
                out.push_str(&target);
            }
            out.push_str(rest);
        }
        if lines.peek().is_some() {
            out.push('\n');
        }
    }
    out
}

/// Split a line into its indentation level and the remaining content. Any
/// sub-level remainder (e.g. alignment spaces) stays with the content.
fn split_indent(line: &str, from: IndentStyle, from_unit: usize) -> (usize, &str) {
    match from {
        IndentStyle::Tabs => {
            let tabs = line.bytes().take_while(|&b| b == b'\t').count();
            (tabs, &line[tabs..])
        }
        IndentStyle::Spaces(_) => {
            let spaces = line.bytes().take_while(|&b| b == b' ').count();
            let level = spaces / from_unit;
            (level, &line[level * from_unit..])
        }
        IndentStyle::Unknown => (0, line),
    }
}
