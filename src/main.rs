mod cli;
mod mcp;
mod output;
mod parser;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use std::io::{ErrorKind as IoErrorKind, Write};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    if let Some(cli::Commands::Completion { shell }) = cli.command {
        let mut command = cli::Cli::command();
        let mut completions = Vec::new();
        clap_complete::generate(
            shell,
            &mut command,
            env!("CARGO_BIN_NAME"),
            &mut completions,
        );
        if let Err(err) = std::io::stdout().write_all(&completions)
            && err.kind() != IoErrorKind::BrokenPipe
        {
            return Err(err.into());
        }
        return Ok(());
    }

    let query = cli.search_query();

    let (text, client) = mcp::search(query, &cli).await?;
    let results = parser::parse(&text);

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&results)?);
    } else {
        output::Printer::new().print_results(&results, query, cli.match_case);
    }

    client.cancel().await
}
