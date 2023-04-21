use reqwest::Url;
use scraper::{Html, Selector};
use crate::config::ScraperConfig;
use std::collections::HashSet;

pub struct Crawler {
    pub config: super::config::ScraperConfig,
}

impl Crawler {
    pub fn new(config: ScraperConfig) -> Self {
        Crawler { config }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut visited_urls: HashSet<String> = HashSet::new();
        let mut urls_to_visit = vec![self.config.start_path.clone()];
    
        while let Some(url) = urls_to_visit.pop() {
            if visited_urls.contains(&url) {
                continue;
            }
    
            let full_url = format!("{}{}", self.config.base_url, url);
    
            match reqwest::get(&full_url).await {
                Ok(response) => {
                    let html = Html::parse_document(&response.text().await?);
                    let new_links = self.extract_links(&html);
                    urls_to_visit.extend(new_links.into_iter().filter(|link| !visited_urls.contains(link)));
    
                    visited_urls.insert(url.clone());
                    println!("New page found: {}", full_url);
                }
                Err(error) => {
                    println!("Failed to fetch URL: {}, Error: {}", full_url, error);
                }
            }
        }
    
        Ok(())
    }
    
    

    fn extract_links(&self, html: &Html) -> HashSet<String> {
        let link_selector = Selector::parse("a[href]").unwrap();
        let base_url = Url::parse(&self.config.base_url).unwrap();
        let mut links = HashSet::new();

        for element in html.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(parsed_url) = base_url.join(href) {
                    if parsed_url.host_str() == base_url.host_str() {
                        links.insert(parsed_url.path().to_owned());
                    }
                }
            }
        }

        links
    }
}
