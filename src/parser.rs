use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub repository: String,
    pub path: String,
    pub url: String,
    pub license: String,
    pub snippets: Vec<Snippet>,
}

#[derive(Debug, Serialize)]
pub struct Snippet {
    pub start_line: u64,
    pub lines: Vec<String>,
}

pub fn parse(text: &str) -> Vec<SearchResult> {
    let mut results = Vec::new();
    let mut current: Option<SearchResult> = None;
    let mut current_snippet: Option<Snippet> = None;
    let mut in_snippet = false;

    for line in text.lines() {
        if let Some(repo) = line.strip_prefix("Repository: ") {
            flush_result(&mut results, &mut current, &mut current_snippet);
            current = Some(SearchResult {
                repository: repo.into(),
                path: String::new(),
                url: String::new(),
                license: String::new(),
                snippets: Vec::new(),
            });
            in_snippet = false;
        } else if let Some(v) = line.strip_prefix("Path: ") {
            if let Some(ref mut r) = current {
                r.path = v.into();
            }
        } else if let Some(v) = line.strip_prefix("URL: ") {
            if let Some(ref mut r) = current {
                r.url = v.into();
            }
        } else if let Some(v) = line.strip_prefix("License: ") {
            if let Some(ref mut r) = current {
                r.license = v.into();
            }
        } else if line.starts_with("--- Snippet ") && line.ends_with("---") {
            if let Some(s) = current_snippet.take()
                && let Some(ref mut r) = current
            {
                r.snippets.push(s);
            }
            current_snippet = Some(Snippet {
                start_line: parse_start_line(line),
                lines: Vec::new(),
            });
            in_snippet = true;
        } else if line == "Snippets:" {
            // skip header
        } else if in_snippet && let Some(ref mut s) = current_snippet {
            s.lines.push(line.into());
        }
    }

    flush_result(&mut results, &mut current, &mut current_snippet);

    // Trim trailing empty lines from snippets
    for r in &mut results {
        for s in &mut r.snippets {
            while s.lines.last().is_some_and(|l| l.is_empty()) {
                s.lines.pop();
            }
        }
    }

    results
}

fn flush_result(
    results: &mut Vec<SearchResult>,
    current: &mut Option<SearchResult>,
    snippet: &mut Option<Snippet>,
) {
    if let Some(mut r) = current.take() {
        if let Some(s) = snippet.take() {
            r.snippets.push(s);
        }
        results.push(r);
    }
}

fn parse_start_line(header: &str) -> u64 {
    header
        .split("(Line ")
        .nth(1)
        .and_then(|s| s.strip_suffix(") ---"))
        .and_then(|s| s.parse().ok())
        .unwrap_or(1)
}
