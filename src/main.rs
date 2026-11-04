use anyhow::{Context, Result};
use clap::Parser;
use url::Url;

use crate::cli::Cli;

mod cli;
mod config;

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
    if !matches!(start_url.scheme(), "http" |"https"){
        anyhow::bail!("Поддерживается только http/https, получено: {}" , start_url.scheme());
    }

    //4. Assembling the configuration
    let config = Config{
        start_url,
        max_depth : cli.depth,
        max_pages : cli.max_pages,
        timeout_secs : cli.timeout,
        concurrency : cli.concurrency,
    };

    tracing::info!("Конфигурация : {:?}", config);

    //TO DO . Here need write krayler call
    println!("Краулер готов к работе. Стартовый URL: {}", config.start_url);

    Ok(())
}
