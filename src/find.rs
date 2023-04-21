use crate::config::ScraperConfig;
use scraper::{Html, Selector};

pub struct Scraper {
    config: ScraperConfig,
}

impl Scraper {
    pub fn new(config: ScraperConfig) -> Self {
        Scraper { config }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.config.base_url, self.config.start_path);
        let response = reqwest::get(&url).await?;
        let html = Html::parse_document(&response.text().await?);

        // Scrape the data you want from the `html` variable.
        // For example, extract specific information using scraper::Selector.

        println!("Scraped data from: {}", url);

        Ok(())
    }
    
    pub async fn scrape_data(&self, selectors: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let url = &self.config.base_url;
        println!("Fetching page: {}", url);
        let response = reqwest::get(url).await?;
        println!("Page fetched successfully.");
        let html = response.text().await?;
        let parsed_html = Html::parse_document(&html);

        for selector in selectors {
            let scraper_selector = match Selector::parse(selector) {
                Ok(selector) => selector,
                Err(err) => {
                    eprintln!("Invalid selector");
                    continue;
                }
            };

            println!("\nScraping data for selector: {}", selector);
            for element in parsed_html.select(&scraper_selector) {
                println!("{}", element.inner_html());
            }
        }

        Ok(())
    }
}
