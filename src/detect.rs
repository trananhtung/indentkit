//! Indentation detection.
//!
//! Uses the same robust approach as Node's `detect-indent`: for each indented
//! line, record the *step* (the change in leading-whitespace width from the
//! previous line) keyed by type, and pick the most frequently used step. This is
//! resistant to alignment/continuation lines and block-comment `*` runs that
//! would fool a naive "smallest indentation" heuristic.

use alloc::vec::Vec;

use crate::IndentStyle;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ws {
    Tab,
    Space,
}

/// One observed indentation step: `(type, amount, count, weight)`.
type Entry = (Ws, usize, usize, usize);

pub(crate) fn detect(text: &str) -> IndentStyle {
    // First pass ignores lone single spaces (usually comment/markup alignment);
    // fall back to counting them if that leaves nothing.
    let entries = indents_map(text, true);
    let entries = if entries.is_empty() {
        indents_map(text, false)
    } else {
        entries
    };

    match most_used(&entries) {
        None => IndentStyle::Unknown,
        Some((Ws::Tab, _)) => IndentStyle::Tabs,
        Some((Ws::Space, amount)) => IndentStyle::Spaces(amount),
    }
}

fn indents_map(text: &str, ignore_single_space: bool) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    let mut previous_size = 0usize;
    let mut previous_type: Option<Ws> = None;
    let mut current: Option<(Ws, usize)> = None;

    for line in text.split('\n') {
        if line.is_empty() {
            previous_size = 0;
            previous_type = None;
            continue;
        }
        let (ws, width) = match line.as_bytes()[0] {
            b'\t' => (Ws::Tab, line.bytes().take_while(|&b| b == b'\t').count()),
            b' ' => (Ws::Space, line.bytes().take_while(|&b| b == b' ').count()),
            _ => {
                // unindented content line
                previous_size = 0;
                previous_type = None;
                continue;
            }
        };
        // A whitespace-only line carries no indentation signal.
        if line.trim().is_empty() {
            previous_size = 0;
            previous_type = None;
            continue;
        }
        if ignore_single_space && ws == Ws::Space && width == 1 {
            continue;
        }

        if previous_type != Some(ws) {
            previous_size = 0;
        }
        previous_type = Some(ws);

        let (same, abs_diff) = if width >= previous_size {
            (width == previous_size, width - previous_size)
        } else {
            (false, previous_size - width)
        };
        previous_size = width;

        let mut weight = 0;
        if same {
            // Same indentation as the previous line reinforces the current step.
            weight = 1;
        } else {
            current = Some((ws, abs_diff));
        }
        if let Some(key) = current {
            upsert(&mut entries, key, weight);
        }
    }
    entries
}

fn upsert(entries: &mut Vec<Entry>, key: (Ws, usize), weight: usize) {
    for e in entries.iter_mut() {
        if e.0 == key.0 && e.1 == key.1 {
            e.2 += 1;
            e.3 += weight;
            return;
        }
    }
    entries.push((key.0, key.1, 1, weight));
}

/// The most-used step by count, then weight; ties resolve to the first seen.
fn most_used(entries: &[Entry]) -> Option<(Ws, usize)> {
    let mut best: Option<Entry> = None;
    for &e in entries {
        match best {
            Some(b) if b.2 > e.2 || (b.2 == e.2 && b.3 >= e.3) => {}
            _ => best = Some(e),
        }
    }
    best.map(|e| (e.0, e.1))
}
