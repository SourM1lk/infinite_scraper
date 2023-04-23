mod config;
mod crawler;
mod find;
mod selector;

use config::{CliOptions, ScraperConfig};
use crawler::Crawler;
use find::Scraper;
use selector::SelectorExtractor;
use structopt::StructOpt;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI options
    let options = CliOptions::from_args();
    // Create a ScraperConfig from CLI options
    let config = ScraperConfig::from_options(options.clone())?;

    async fn run_scraper(options: &CliOptions, config: &ScraperConfig) -> Result<(), Box<dyn std::error::Error>> {
        if options.crawl {
            let mut crawler = Crawler::new(config.clone());
            if options.use_regex.is_some() {
                let selectors: Vec<String> = options
                    .use_regex
                    .as_ref()
                    .map(|s| vec![s.trim().to_owned()])
                    .unwrap_or_else(Vec::new);
                crawler.run(&selectors).await?;
            } else {
                let selectors: Vec<String> = options
                    .use_selectors
                    .as_ref()
                    .map(|s| s.split(',').map(|part| part.trim().to_owned()).collect())
                    .unwrap_or_else(Vec::new);
                crawler.run(&selectors).await?;
            }
        }

        if options.list_selectors {
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

        if options.scrape && options.use_selectors.is_some() {
            let scraper = Scraper::new(config.clone());
            let selectors: Vec<String> = options
                .use_selectors
                .as_ref()
                .map(|s| s.split(',').map(|part| part.trim().to_owned()).collect())
                .unwrap_or_else(Vec::new);
            println!("Scraping data using provided CSS selectors...");
            scraper.scrape_data(&selectors).await?;
        }

        if options.scrape && options.use_regex.is_some() {
            let config = ScraperConfig::from_options(options.clone())?;
            let scraper = Scraper::new(config.clone());
            let regex_patterns: Vec<String> = options
                .use_regex
                .as_ref()
                .map(|s| vec![s.trim().to_owned()])
                .unwrap_or_else(Vec::new);
            println!("Scraping data using provided regex patterns...");
            println!("Regex patterns: {:?}", regex_patterns);
            scraper.scrape_data_with_regex(&regex_patterns).await?;
        }

        Ok(())
    }

    if let Some(interval_str) = &options.interval {
        let interval_parts: Vec<&str> = interval_str.split(':').collect();
        if interval_parts.len() != 3 {
            eprintln!("Invalid interval format. Use HH:MM:SS.");
            std::process::exit(1);
        }

        let hours: u64 = interval_parts[0].parse().unwrap();
        let minutes: u64 = interval_parts[1].parse().unwrap();
        let seconds: u64 = interval_parts[2].parse().unwrap();
        let interval_duration = Duration::from_secs(hours * 3600 + minutes * 60 + seconds);

        loop {
            println!("Running scraper at {}", chrono::Utc::now());
            if let Err(e) = run_scraper(&options, &config).await {
                eprintln!("Error during scraper run: {}", e);
            }

            println!("Waiting for {} seconds before running scraper again...", interval_duration.as_secs());
            sleep(interval_duration).await;
        }
    } else {
        run_scraper(&options, &config).await?;
    }

    Ok(())
}

