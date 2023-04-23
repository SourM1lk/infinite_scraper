mod config;
mod crawler;
mod find;
mod selector;

use config::{CliOptions, ScraperConfig};
use crawler::Crawler;
use find::Scraper;
use selector::SelectorExtractor;
use std::time::Duration;
use structopt::StructOpt;
use tokio::time::sleep;
use colored::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    display_welcome_message();
    // Parse CLI options
    let options = CliOptions::from_args();
    // Create a ScraperConfig from CLI options
    let config = ScraperConfig::from_options(options.clone())?;

    // Run the scraper
    async fn run_scraper(
        options: &CliOptions,
        config: &ScraperConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if options.crawl {
            // Create a Crawler instance using regex patterns
            let mut crawler =
                Crawler::new(config.clone(), config.max_connections, options.use_proxies);
            if options.use_regex.is_some() {
                let selectors: Vec<String> = options
                    .use_regex
                    .as_ref()
                    .map(|s| vec![s.trim().to_owned()])
                    .unwrap_or_else(Vec::new);
                crawler.run(&selectors).await?;
            } else {
                // Create a Crawler instance using CSS selectors
                let selectors: Vec<String> = options
                    .use_selectors
                    .as_ref()
                    .map(|s| s.split(',').map(|part| part.trim().to_owned()).collect())
                    .unwrap_or_else(Vec::new);
                crawler.run(&selectors).await?;
            }
        }

        // List ALL CSS selectors from a page
        if options.list_selectors {
            let url = &config.base_url;
            println!("Fetching page: {}", url);
            let response = reqwest::get(url).await?;
            println!("Page fetched successfully.");
            let html = response.text().await?;
            println!("Extracting CSS Selectors from the page...");
            let selector_extractor = SelectorExtractor::new();
            let selectors =
                selector_extractor.extract_css_selectors(&html, options.include_duplicates);
            println!("CSS Selectors found in the page:");
            for selector in selectors {
                println!("{}", selector);
            }
        }

        // Scraping data using CSS selectors
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

        // Scraping data using regex patterns
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

    // Run the scraper once or run it in an interval
    if let Some(interval_str) = &options.interval {
        let interval_parts: Vec<&str> = interval_str.split(':').collect();
        if interval_parts.len() != 3 {
            eprintln!("Invalid interval format. Use HH:MM:SS.");
            std::process::exit(1);
        }
        // HH:MM:SS
        let hours: u64 = interval_parts[0].parse().unwrap();
        let minutes: u64 = interval_parts[1].parse().unwrap();
        let seconds: u64 = interval_parts[2].parse().unwrap();
        let interval_duration = Duration::from_secs(hours * 3600 + minutes * 60 + seconds);

        loop {
            println!("Running scraper at {}", chrono::Utc::now());
            if let Err(e) = run_scraper(&options, &config).await {
                eprintln!("Error during scraper run: {}", e);
            }

            println!(
                "Waiting for {} seconds before running scraper again...",
                interval_duration.as_secs()
            );
            sleep(interval_duration).await;
        }
    } else {
        run_scraper(&options, &config).await?;
    }

    Ok(())
}

fn display_welcome_message() {
    let logo = r#"

    __  __      _ _               ____      _ _ _         ___                               
    |  \/  | ___| | | ___ _ __    / ___|___ | | (_) ___   ( _ )                              
    | |\/| |/ _ \ | |/ _ \ '_ \  | |   / _ \| | | |/ _ \  / _ \/\                            
    | |  | |  __/ | |  __/ | | | | |__| (_) | | | |  __/ | (_>  <                            
    |_|  |_|\___|_|_|\___|_| |_|  \____\___/|_|_|_|\___|  \___/\/                            
     _   _            ___        __ _       _ _         ____                                 
    | |_| |__   ___  |_ _|_ __  / _(_)_ __ (_) |_ ___  / ___|  ___ _ __ __ _ _ __   ___ _ __ 
    | __| '_ \ / _ \  | || '_ \| |_| | '_ \| | __/ _ \ \___ \ / __| '__/ _` | '_ \ / _ \ '__|
    | |_| | | |  __/  | || | | |  _| | | | | | ||  __/  ___) | (__| | | (_| | |_) |  __/ |   
     \__|_| |_|\___| |___|_| |_|_| |_|_| |_|_|\__\___| |____/ \___|_|  \__,_| .__/ \___|_|   
                                                                            |_|                 
                                                 "#;

    let mut colors = vec![
        "red",
        "green",
        "yellow",
        "blue",
        "magenta",
        "cyan",
        "white",
    ];
    let mut rng = thread_rng();
    colors.shuffle(&mut rng);
    let mut index = 0;
    for line in logo.lines() {
        println!("{}", line.color(colors[index % colors.len()]).bold());
        index += 1;
    }
}