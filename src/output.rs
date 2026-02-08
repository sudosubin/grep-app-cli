use std::io::Cursor;
use std::path::Path;

use crossterm::style::{
    Attribute, Color, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, Theme, ThemeSet},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

use crate::parser::SearchResult;

const THEME_DARK: &[u8] = include_bytes!("../assets/github-dark.tmTheme");
const THEME_LIGHT: &[u8] = include_bytes!("../assets/github-light.tmTheme");

// ── Color palette ───────────────────────────────────────────────────

const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb { r, g, b }
}

struct Palette {
    heading: Color,
    repo: Color,
    match_bg: Color,
    info: Color,
    separator: Color,
}

impl Palette {
    fn dark() -> Self {
        Self {
            heading: rgb(63, 185, 80),
            repo: rgb(88, 166, 255),
            match_bg: rgb(120, 70, 10),
            info: rgb(139, 148, 158),
            separator: rgb(110, 118, 129),
        }
    }

    fn light() -> Self {
        Self {
            heading: rgb(26, 127, 55),
            repo: rgb(9, 105, 218),
            match_bg: rgb(255, 220, 160),
            info: rgb(101, 109, 118),
            separator: rgb(208, 215, 222),
        }
    }
}

// ── Printer ─────────────────────────────────────────────────────────

pub struct Printer {
    syntax_set: SyntaxSet,
    theme: Theme,
    palette: Palette,
}

impl Printer {
    pub fn new() -> Self {
        let is_light = matches!(terminal_light::luma(), Ok(l) if l > 0.6);
        let theme_data = if is_light { THEME_LIGHT } else { THEME_DARK };
        let theme = ThemeSet::load_from_reader(&mut Cursor::new(theme_data))
            .expect("bundled theme must be valid");

        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme,
            palette: if is_light {
                Palette::light()
            } else {
                Palette::dark()
            },
        }
    }

    pub fn print_results(&self, results: &[SearchResult], query: &str, case_sensitive: bool) {
        let p = &self.palette;

        if results.is_empty() {
            println!("{}No results found.{RST}", fg(p.info));
            return;
        }

        println!(
            "{}{}Found {} result(s):{RST}",
            fg(p.heading),
            BOLD,
            results.len()
        );
        println!();

        for (i, result) in results.iter().enumerate() {
            self.print_result(result, query, case_sensitive);
            if i < results.len() - 1 {
                println!("{}{}{RST}", fg(p.separator), "─".repeat(60));
                println!();
            }
        }
    }

    fn print_result(&self, result: &SearchResult, query: &str, case_sensitive: bool) {
        let p = &self.palette;

        // Header: repo + license
        if result.license != "Unknown" {
            println!(
                "{}{BOLD}● {}{RST} {}{}{RST}",
                fg(p.repo),
                result.repository,
                fg(p.info),
                result.license
            );
        } else {
            println!("{}{BOLD}● {}{RST}", fg(p.repo), result.repository);
        }

        // URL
        println!("  {}{UL}{}{RST}", fg(p.info), result.url);
        println!();

        // File path (aligned with source: 2 + 5 + 1 = 8 chars indent)
        println!("        File: {BOLD}{}{RST}", result.path);

        // Snippets
        let syntax = self.syntax_for_path(&result.path);
        let mut prev_end: Option<u64> = None;
        let last_idx = result.snippets.len().saturating_sub(1);

        for (idx, snippet) in result.snippets.iter().enumerate() {
            if let Some(end) = prev_end
                && snippet.start_line > end + 1
            {
                println!("  {DIM}{:>5}{RST}", "⋮");
            }

            let mut hl = HighlightLines::new(syntax, &self.theme);
            let code = snippet.lines.join("\n") + "\n";

            for (offset, line_text) in LinesWithEndings::from(&code).enumerate() {
                let line_num = snippet.start_line + offset as u64;
                let ranges = hl
                    .highlight_line(line_text, &self.syntax_set)
                    .unwrap_or_default();
                let has_match = if case_sensitive {
                    line_text.contains(query)
                } else {
                    line_text.to_lowercase().contains(&query.to_lowercase())
                };
                let colored = if has_match {
                    self.highlight_matches(&ranges, line_text, query, case_sensitive)
                } else {
                    styled_line(&ranges)
                };
                print!("  {DIM}{line_num:>5}{RST} {colored}");
            }

            prev_end = Some(snippet.start_line + snippet.lines.len().saturating_sub(1) as u64);
            if idx == last_idx {
                println!();
            }
        }
    }

    fn syntax_for_path<'a>(&'a self, path: &str) -> &'a syntect::parsing::SyntaxReference {
        let ext = Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let mapped = match ext {
            "ts" | "tsx" | "jsx" | "mjs" | "cjs" | "mts" | "cts" => "js",
            "kt" | "kts" => "java",
            "swift" => "c",
            "dart" => "java",
            "toml" => "yaml",
            "dockerfile" | "Dockerfile" => "sh",
            "vue" | "svelte" => "html",
            other => other,
        };

        self.syntax_set
            .find_syntax_by_extension(mapped)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text())
    }

    fn highlight_matches(
        &self,
        ranges: &[(Style, &str)],
        line: &str,
        query: &str,
        case_sensitive: bool,
    ) -> String {
        let matches = match_byte_ranges(line, query, case_sensitive);
        if matches.is_empty() {
            return styled_line(ranges);
        }

        let bg = SetBackgroundColor(self.palette.match_bg).to_string();
        let mut out = String::new();
        let mut pos: usize = 0;

        for &(style, text) in ranges {
            let span = pos..pos + text.len();
            let sfg = style_fg(style);

            if !matches
                .iter()
                .any(|(ms, me)| *ms < span.end && *me > span.start)
            {
                out.push_str(&sfg);
                out.push_str(text);
            } else {
                let mut i = 0;
                while i < text.len() {
                    let abs = pos + i;
                    let in_match = matches.iter().any(|(ms, me)| abs >= *ms && abs < *me);
                    let run_end = next_boundary(&matches, abs, in_match, pos, text.len());
                    let chunk = &text[i..run_end];

                    if in_match {
                        out.push_str(&bg);
                        out.push_str(&BOLD.to_string());
                        out.push_str(&sfg);
                        out.push_str(chunk);
                        out.push_str(&RST.to_string());
                    } else {
                        out.push_str(&sfg);
                        out.push_str(chunk);
                    }
                    i = run_end;
                }
            }
            pos += text.len();
        }
        out.push_str(&RST.to_string());
        out
    }
}

// ── Rendering helpers ───────────────────────────────────────────────

const BOLD: SetAttribute = SetAttribute(Attribute::Bold);
const DIM: SetAttribute = SetAttribute(Attribute::Dim);
const UL: SetAttribute = SetAttribute(Attribute::Underlined);
const RST: ResetColor = ResetColor;

fn fg(color: Color) -> SetForegroundColor {
    SetForegroundColor(color)
}

fn style_fg(style: Style) -> String {
    let c = style.foreground;
    SetForegroundColor(Color::Rgb {
        r: c.r,
        g: c.g,
        b: c.b,
    })
    .to_string()
}

fn styled_line(ranges: &[(Style, &str)]) -> String {
    let mut out = String::new();
    for &(style, text) in ranges {
        out.push_str(&style_fg(style));
        out.push_str(text);
    }
    out.push_str(&RST.to_string());
    out
}

/// Find the next transition point between match/non-match regions.
fn next_boundary(
    matches: &[(usize, usize)],
    abs: usize,
    in_match: bool,
    span_start: usize,
    text_len: usize,
) -> usize {
    let limit = text_len;
    if in_match {
        matches
            .iter()
            .filter(|(ms, me)| abs >= *ms && abs < *me)
            .map(|(_, me)| me.saturating_sub(span_start).min(limit))
            .min()
    } else {
        matches
            .iter()
            .filter(|(ms, _)| *ms > abs)
            .map(|(ms, _)| ms.saturating_sub(span_start).min(limit))
            .min()
    }
    .unwrap_or(limit)
}

/// Find all byte-offset ranges where `query` matches in `line`.
/// When `case_sensitive` is false, matches case-insensitively.
fn match_byte_ranges(line: &str, query: &str, case_sensitive: bool) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    if query.is_empty() {
        return ranges;
    }

    if case_sensitive {
        let mut start = 0;
        while let Some(pos) = line[start..].find(query) {
            let abs = start + pos;
            ranges.push((abs, abs + query.len()));
            start = abs + query.len();
        }
    } else {
        let lower_line = line.to_lowercase();
        let lower_query = query.to_lowercase();
        let mut start = 0;
        while let Some(pos) = lower_line[start..].find(&lower_query) {
            let abs = start + pos;
            ranges.push((abs, abs + lower_query.len()));
            start = abs + lower_query.len();
        }
    }

    ranges
}
