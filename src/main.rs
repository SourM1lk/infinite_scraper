mod config;
mod crawler;
mod find;
mod selector;

use config::{CliOptions, ScraperConfig};
use crawler::Crawler;
use find::Scraper;
use selector::SelectorExtractor;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI options
    let options = CliOptions::from_args();
    // Create a ScraperConfig from CLI options
    let config = ScraperConfig::from_options(options.clone())?;

    // Crawl the website if the --crawl flag is set
    if options.crawl {
        let mut crawler = Crawler::new(config.clone());
        //when user provides --crawl flag and --use-regex flag
        if options.use_regex.is_some() {
            let selectors: Vec<String> = options
            .use_regex
            .as_ref()
            .map(|s| vec![s.trim().to_owned()])
            .unwrap_or_else(Vec::new);
            crawler.run(&selectors).await?;
        }
        else {
            // When user provides --crawl flag and --use-selectors flag or --crawl flag only
            let selectors: Vec<String> = options
            .use_selectors
            .as_ref()
            .map(|s| s.split(',').map(|part| part.trim().to_owned()).collect())
            .unwrap_or_else(Vec::new);
        crawler.run(&selectors).await?;
        }
    }

    //when user provides --list-selectors flag
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
    //when user provides --scrape flag and --use-selectors flag
    if options.scrape && options.use_selectors.is_some() {
        let scraper = Scraper::new(config);
        let selectors: Vec<String> = options
            .use_selectors
            .as_ref()
            .map(|s| s.split(',').map(|part| part.trim().to_owned()).collect())
            .unwrap_or_else(Vec::new);
        println!("Scraping data using provided CSS selectors...");
        scraper.scrape_data(&selectors).await?;
    }
    //when user provides --scrape flag and --regex flag
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