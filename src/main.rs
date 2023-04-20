mod config;
mod crawler;

use config::Config;
use crawler::Crawler;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    println!("Configuration: {:?}", config);

    let mut crawler = Crawler::new(&config.base_url)?;
    let links = crawler.crawl(&config.start_path).await?;

    println!("Links:");
    for link in links {
        println!("{}", link);
    }

    Ok(())
}
