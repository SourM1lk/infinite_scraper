mod config;
mod crawler;
mod find;
mod selector;

use config::{CliOptions, ScraperConfig};
use find::Scraper;
use selector::SelectorExtractor;
use crawler::Crawler;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = CliOptions::from_args();
    let config = ScraperConfig::from_options(options.clone())?;

    if options.crawl {
        let crawler = Crawler::new(config.clone());
        crawler.run().await?;
    }

    if options.scrape {
        let scraper = Scraper::new(config.clone());
        scraper.run().await?;
    }

    if options.selectors {
        let url = &config.base_url;
        println!("Fetching page: {}", url);
        let response = reqwest::get(url).await?;
        println!("Page fetched successfully.");
        let html = response.text().await?;
        println!("Extracting CSS Selectors from the page...");
        let selector_extractor = SelectorExtractor::new();
        let selectors = selector_extractor.extract_css_selectors(&html, options.include_duplicates);
        println!("CSS Selectors found in the page:");
        for selector in selectors {
            println!("{}", selector);
        }
    }

    if !options.css_selectors.is_empty() {
        let scraper = Scraper::new(config);
        let selectors: Vec<String> = options.css_selectors.split(',').map(|s| s.trim().to_owned()).collect();
        println!("Scraping data using provided CSS selectors...");
        scraper.scrape_data(&selectors).await?;
    }

    Ok(())
}
