use reqwest::Url;
use scraper::{Html, Selector};
use crate::config::ScraperConfig;
use std::collections::HashSet;
use crate::find::Scraper;

pub struct Crawler {
    pub config: super::config::ScraperConfig,
    pub scraper: Scraper,
    visited_urls: HashSet<String>,
}

impl Crawler {
    pub fn new(config: ScraperConfig) -> Self {
        Crawler { 
            config: config.clone(),
            scraper: Scraper::new(config),
            visited_urls: HashSet::new(),        
        }
    }

    pub async fn run(&mut self, selectors: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let mut queue: Vec<String> = vec![self.config.base_url.clone()];
        
        while let Some(url) = queue.pop() {
            if self.visited_urls.insert(url.clone()) {
                println!("Visiting: {}", url);
                let response = reqwest::get(&url).await?;
                let html = response.text().await?;
                let links = self.extract_links(&html);
                
                for link in links {
                    if !self.visited_urls.contains(&link) {
                        queue.push(link);
                    }
                }
                
                // Scrape data using provided CSS selectors
                let scraper = Scraper::new(self.config.clone());
                scraper.scrape_data(selectors).await?;
            }
        }
        
        Ok(())
    }
    

    fn extract_links(&self, html: &str) -> Vec<String> {
        let document = Html::parse_document(html);
        let a_selector = Selector::parse("a").unwrap();
        let base_url = &self.config.base_url;
        
        document.select(&a_selector)
            .filter_map(|element| element.value().attr("href"))
            .filter(|link| !link.starts_with("javascript:"))
            .filter(|link| !link.starts_with("mailto:"))
            .filter(|link| !link.starts_with("#"))
            .filter(|link| !link.starts_with("tel:"))
            .filter(|link| !link.starts_with("data:"))
            .filter(|link| !link.starts_with("ftp:"))
            .filter(|link| !link.starts_with("file:"))
            .filter(|link| !link.starts_with("sms:"))
            .filter(|link| !link.starts_with("skype:"))
            .filter(|link| !link.starts_with("whatsapp:"))
            .filter(|link| !link.starts_with("viber:"))
            .filter(|link| !link.starts_with("intent:"))
            .filter(|link| !link.starts_with("geo:"))
            .filter(|link| !link.starts_with("magnet:"))
            .filter(|link| !link.starts_with("bitcoin:"))
            .filter(|link| !link.starts_with("spotify:"))
            .filter(|link| !link.starts_with("steam:"))
            .map(|link| Url::parse(link).unwrap_or_else(|_| Url::parse(&format!("{}/{}", base_url, link)).unwrap()))
            .map(|url| url.into_string())
            .collect()
    }
    
}
