use anyhow::{Context, Result};
use clap::Parser;
use url::Url;

use crate::cli::Cli;

mod cli;
mod config;
mod fetcher;

use crate::config::Config;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    //1. Parse CLI
    let cli = Cli::parse();
    //2. Validation URL
    let start_url =
        Url::parse(&cli.url).with_context(|| format!("Некорректный UTL : {}", cli.url))?;
    //3. Cheks scheme is http | https
    if !matches!(start_url.scheme(), "http" | "https") {
        anyhow::bail!(
            "Поддерживается только http/https, получено: {}",
            start_url.scheme()
        );
    }

    //4. Assembling the configuration
    let config = Config {
        start_url,
        max_depth: cli.depth,
        max_pages: cli.max_pages,
        timeout_secs: cli.timeout,
        concurrency: cli.concurrency,
    };

    tracing::info!("Конфигурация : {:?}", config);

    //TO DO . Here need write krayler call
    println!(
        "Краулер готов к работе. Стартовый URL: {}",
        config.start_url
    );

    let client = fetcher::build_client(config.timeout_secs)?;
    match fetcher::fetch(&client, &config.start_url).await {
        Ok(result) => {
            tracing::info!(
                "status={} requested={} final={} html_len={}",
                result.status,
                result.requested_url,
                result.final_url,
                result.html.as_ref().map(|s| s.len()).unwrap_or(0)
            );
            if let Some(html) = &result.html {
                let preview : String = html.chars().take(200).collect();
                println!("---- first 200 sumbols HTML ----");
                println!("{}", preview);
            }
        }
        Err(e) => {
            tracing::error!("fetch down: {e:#}")
        }
    }
    Ok(())
}
