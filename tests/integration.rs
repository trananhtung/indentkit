//! End-to-end behavioral spec for the public `indentkit` API.

use indentkit::{detect, indent, reindent, IndentStyle};

// ---------------------------------------------------------------------------
// detect()
// ---------------------------------------------------------------------------

#[test]
fn detects_two_space_indent() {
    let code = "function f() {\n  if (x) {\n    return 1;\n  }\n}\n";
    assert_eq!(detect(code), IndentStyle::Spaces(2));
}

#[test]
fn detects_four_space_indent() {
    let code = "def f():\n    if x:\n        return 1\n";
    assert_eq!(detect(code), IndentStyle::Spaces(4));
}

#[test]
fn detects_three_space_indent() {
    let code = "foo:\n   bar:\n      baz\n";
    assert_eq!(detect(code), IndentStyle::Spaces(3));
}

#[test]
fn detects_tab_indent() {
    let code = "if x:\n\tdo()\n\t\tmore()\n";
    assert_eq!(detect(code), IndentStyle::Tabs);
}

#[test]
fn unindented_or_empty_is_unknown() {
    assert_eq!(detect(""), IndentStyle::Unknown);
    assert_eq!(detect("single line"), IndentStyle::Unknown);
    assert_eq!(detect("no indent\njust text\n"), IndentStyle::Unknown);
}

#[test]
fn majority_wins_when_mixed() {
    // three space-indented lines (steps of 2), one stray tab line
    let code = "a\n  b\n    c\n\td\n";
    assert_eq!(detect(code), IndentStyle::Spaces(2));
}

// ---------------------------------------------------------------------------
// reindent()
// ---------------------------------------------------------------------------

#[test]
fn reindent_spaces_to_tabs() {
    let src = "if x:\n    do()\n        more()\n";
    assert_eq!(
        reindent(src, IndentStyle::Tabs),
        "if x:\n\tdo()\n\t\tmore()\n"
    );
}

#[test]
fn reindent_tabs_to_spaces() {
    let src = "if x:\n\tdo()\n\t\tmore()\n";
    assert_eq!(
        reindent(src, IndentStyle::Spaces(2)),
        "if x:\n  do()\n    more()\n"
    );
}

#[test]
fn reindent_two_to_four_spaces() {
    let src = "a\n  b\n    c\n";
    assert_eq!(
        reindent(src, IndentStyle::Spaces(4)),
        "a\n    b\n        c\n"
    );
}

#[test]
fn reindent_preserves_blank_and_flush_lines() {
    let src = "a\n\n  b\n";
    assert_eq!(reindent(src, IndentStyle::Tabs), "a\n\n\tb\n");
}

#[test]
fn reindent_unconvertible_source_is_unchanged() {
    assert_eq!(reindent("no indent\n", IndentStyle::Tabs), "no indent\n");
    assert_eq!(reindent("", IndentStyle::Tabs), "");
}

#[test]
fn reindent_to_unknown_is_unchanged() {
    let src = "a\n  b\n";
    assert_eq!(reindent(src, IndentStyle::Unknown), src);
}

// ---------------------------------------------------------------------------
// indent() codegen helper
// ---------------------------------------------------------------------------

#[test]
fn indent_builds_levels() {
    assert_eq!(indent(0, IndentStyle::Spaces(4)), "");
    assert_eq!(indent(1, IndentStyle::Spaces(4)), "    ");
    assert_eq!(indent(2, IndentStyle::Spaces(4)), "        ");
    assert_eq!(indent(3, IndentStyle::Tabs), "\t\t\t");
    assert_eq!(indent(2, IndentStyle::Unknown), "");
}

// ---------------------------------------------------------------------------
// IndentStyle helpers
// ---------------------------------------------------------------------------

#[test]
fn unit_string() {
    assert_eq!(IndentStyle::Spaces(2).unit(), "  ");
    assert_eq!(IndentStyle::Tabs.unit(), "\t");
    assert_eq!(IndentStyle::Unknown.unit(), "");
}

#[test]
fn display() {
    assert_eq!(IndentStyle::Spaces(4).to_string(), "4 spaces");
    assert_eq!(IndentStyle::Spaces(1).to_string(), "1 space");
    assert_eq!(IndentStyle::Tabs.to_string(), "tab");
    assert_eq!(IndentStyle::Unknown.to_string(), "unknown");
}

// ---------------------------------------------------------------------------
// Regression tests from the adversarial pre-publish review
// ---------------------------------------------------------------------------

#[test]
fn detect_robust_against_continuation_lines() {
    let code = "def process(data):\n    result = []\n    for item in data:\n        value = transform(item,\n          options)\n        result.append(value)\n    return result\n";
    assert_eq!(detect(code), IndentStyle::Spaces(4));
}

#[test]
fn detect_ignores_jsdoc_star_alignment() {
    let code = "/**\n * doc\n * more\n */\nfunction g() {\n    return 1;\n}\n";
    assert_eq!(detect(code), IndentStyle::Spaces(4));
}

#[test]
fn detect_ignores_whitespace_only_lines() {
    // a blank line left with trailing spaces must not skew 2-space detection
    let code = "fn f() {\n  a;\n   \n  b;\n}\n";
    assert_eq!(detect(code), IndentStyle::Spaces(2));
}

#[test]
fn detect_resolves_ambiguous_tie_to_first_seen_step() {
    // +4 then +2: the first-seen step (4) wins, like detect-indent.
    assert_eq!(detect("a\n    b\n      c\n"), IndentStyle::Spaces(4));
}

#[test]
fn detect_tabs_win_on_tie_when_seen_first() {
    assert_eq!(detect("\ta\n  b\n"), IndentStyle::Tabs);
}

#[test]
fn reindent_blank_lines_are_emptied_not_indented() {
    let src = "a\n  b\n  \n  c\n";
    assert_eq!(reindent(src, IndentStyle::Tabs), "a\n\tb\n\n\tc\n");
}

#[test]
fn reindent_to_zero_width_is_unchanged() {
    let src = "a\n  b\n";
    assert_eq!(reindent(src, IndentStyle::Spaces(0)), src);
}
