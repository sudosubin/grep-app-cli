use anyhow::{Context, Result};
use rmcp::{
    ServiceExt,
    model::{CallToolRequestParams, ClientCapabilities, ClientInfo, Implementation},
    service::{RunningService, ServiceRole},
    transport::StreamableHttpClientTransport,
};
use serde_json::Value;

use crate::cli::Cli;

const MCP_ENDPOINT: &str = "https://mcp.grep.app";

/// Connected MCP client handle. Must be cancelled when done.
pub struct Client(Box<dyn CancelHandle>);

impl Client {
    pub async fn cancel(self) -> Result<()> {
        self.0.cancel().await
    }
}

trait CancelHandle: Send {
    fn cancel(self: Box<Self>) -> std::pin::Pin<Box<dyn Future<Output = Result<()>> + Send>>;
}

use std::future::Future;

impl<T: ServiceRole + 'static, S: rmcp::Service<T> + 'static> CancelHandle
    for Option<RunningService<T, S>>
{
    fn cancel(mut self: Box<Self>) -> std::pin::Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        Box::pin(async move {
            if let Some(svc) = self.take() {
                svc.cancel().await?;
            }
            Ok(())
        })
    }
}

pub async fn search(cli: &Cli) -> Result<(String, Client)> {
    let transport = StreamableHttpClientTransport::from_uri(MCP_ENDPOINT);
    let client_info = ClientInfo {
        meta: None,
        protocol_version: Default::default(),
        capabilities: ClientCapabilities::default(),
        client_info: Implementation {
            name: "grep-app-cli".into(),
            title: None,
            version: env!("CARGO_PKG_VERSION").into(),
            website_url: None,
            icons: None,
        },
    };

    let service = client_info
        .serve(transport)
        .await
        .context("Failed to connect to mcp.grep.app")?;

    let result = service
        .call_tool(CallToolRequestParams {
            meta: None,
            name: "searchGitHub".into(),
            arguments: Some(build_arguments(cli)),
            task: None,
        })
        .await
        .context("Failed to call searchGitHub tool")?;

    let text = result
        .content
        .iter()
        .filter_map(|c| match c.raw {
            rmcp::model::RawContent::Text(ref t) => Some(t.text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok((text, Client(Box::new(Some(service)))))
}

fn build_arguments(cli: &Cli) -> serde_json::Map<String, Value> {
    let mut args = serde_json::Map::new();
    args.insert("query".into(), Value::String(cli.query.clone()));
    args.insert("matchCase".into(), Value::Bool(cli.match_case));
    args.insert("matchWholeWords".into(), Value::Bool(cli.match_whole_words));
    args.insert("useRegexp".into(), Value::Bool(cli.use_regexp));

    if let Some(ref repo) = cli.repo {
        args.insert("repo".into(), Value::String(repo.clone()));
    }
    if let Some(ref path) = cli.path {
        args.insert("path".into(), Value::String(path.clone()));
    }
    if !cli.language.is_empty() {
        let langs = cli
            .language
            .iter()
            .map(|l| Value::String(l.clone()))
            .collect();
        args.insert("language".into(), Value::Array(langs));
    }
    args
}
