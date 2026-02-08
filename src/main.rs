mod cli;
mod mcp;
mod output;
mod parser;

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let (text, client) = mcp::search(&cli).await?;
    let results = parser::parse(&text);

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&results)?);
    } else {
        output::Printer::new().print_results(&results, &cli.query, cli.match_case);
    }

    client.cancel().await
}
