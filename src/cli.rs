use clap::{CommandFactory, Parser, Subcommand, error::ErrorKind};
use clap_complete::Shell;

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
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// The literal code pattern to search for
    pub query: Option<String>,

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

impl Cli {
    pub fn search_query(&self) -> &str {
        self.query.as_deref().unwrap_or_else(|| {
            Self::command()
                .error(
                    ErrorKind::MissingRequiredArgument,
                    "the following required argument was not provided: <QUERY>",
                )
                .exit()
        })
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate shell completions
    Completion {
        /// Shell to generate for
        shell: Shell,
    },
}
