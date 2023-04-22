use crate::config::ScraperConfig;
use scraper::{Html, Selector};
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::{OpenOptions};
use std::io::prelude::*;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct ScrapedData {
    content: String,
}

pub fn save_scraped_data_as_json(data: &ScrapedData, file_name: &str) -> std::io::Result<()> {
    let json_data = serde_json::to_string(&data).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_name)?;
    writeln!(file, "{}", json_data)?;
    Ok(())
}

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
                let content = element.text().collect::<Vec<_>>().join(" ");
                let trimmed_content = content.trim().to_string();
                let scraped_data = ScrapedData {
                    content: trimmed_content.clone(),
                };
                save_scraped_data_as_json(&scraped_data, "output.json").expect("Failed to save data as JSON");

                println!("{}", element.inner_html().trim());
                println!("{}", content);
                println!("{{\"{}\"}}", content);
                println!("{}", content);
            }
        }

        Ok(())
    }
}
