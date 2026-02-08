use clap::Parser;

/// Search real-world code examples from over a million public GitHub repositories.
#[derive(Parser, Debug)]
#[command(
    version,
    after_help = "\
TIPS:
  Search for actual code patterns, not keywords or questions.
  Good: 'useState(', 'import React from', 'async function'
  Bad:  'react tutorial', 'best practices', 'how to use'

  Use --use-regexp with (?s) prefix to match across multiple lines.
"
)]
pub struct Cli {
    /// The literal code pattern to search for
    pub query: String,

    /// Case sensitive search
    #[arg(long)]
    pub match_case: bool,

    /// Match whole words only
    #[arg(long)]
    pub match_whole_words: bool,

    /// Interpret query as a regular expression
    #[arg(long)]
    pub use_regexp: bool,

    /// Filter by repository (e.g., 'facebook/react')
    #[arg(long)]
    pub repo: Option<String>,

    /// Filter by file path (e.g., 'src/components/Button.tsx')
    #[arg(long)]
    pub path: Option<String>,

    /// Filter by programming language (repeatable)
    #[arg(long)]
    pub language: Vec<String>,

    /// Output raw JSON response
    #[arg(long)]
    pub json: bool,
}
