mod config;
mod crawler;

use config::{CliOptions, ScraperConfig};
use crawler::Crawler;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = CliOptions::from_args();
    let config = ScraperConfig::from_options(options)?;

    let crawler = Crawler { config };
    crawler.run().await?;

    Ok(())
}
