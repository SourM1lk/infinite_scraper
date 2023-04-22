use crate::config::ScraperConfig;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::OpenOptions;
use std::io::prelude::*;

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
    pub async fn scrape_data(&self, selectors: &[String],) -> Result<(), Box<dyn std::error::Error>> {
        if self.config.use_regex {
            println!("Scraping data with regex patterns.");
            self.scrape_data_with_regex(selectors).await?;
        } else {
            let url = &self.config.base_url;
            let response = reqwest::get(url).await?;
            println!("Page fetched successfully.");
            let html = response.text().await?;
            let parsed_html = Html::parse_document(&html);

            for selector in selectors {
                let scraper_selector = match Selector::parse(selector) {
                    Ok(selector) => selector,
                    Err(_err) => {
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
                    save_scraped_data_as_json(&scraped_data, "output.json")
                        .expect("Failed to save data as JSON");

                    println!("{}", element.inner_html().trim());
                    println!("{}", content);
                    println!("{{\"{}\"}}", content);
                    println!("{}", content);
                }
            }
        }

        Ok(())
    }

    pub async fn scrape_data_with_regex(
        &self,
        patterns: &[String],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = &self.config.base_url;
        let response = reqwest::get(url).await?;
        println!("Page fetched successfully.");
        let html = response.text().await?;

        for pattern in patterns {
            let regex = match Regex::new(pattern) {
                Ok(regex) => regex,
                Err(_err) => {
                    eprintln!("Invalid regex pattern");
                    continue;
                }
            };

            println!("\nScraping data for regex pattern: {}", pattern);
            for capture in regex.captures_iter(&html) {
                let content = capture.get(0).map_or("", |m| m.as_str()).to_string();
                let scraped_data = ScrapedData {
                    content: content.clone(),
                };
                save_scraped_data_as_json(&scraped_data, "output.json")
                    .expect("Failed to save data as JSON");

                println!("{}", content);
                println!("{{\"{}\"}}", content);
                println!("{}", content);
            }
        }

        Ok(())
    }
}
