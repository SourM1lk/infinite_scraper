use crate::config::ScraperConfig;
use crate::find::Scraper;
use chrono::prelude::*;
use reqwest::Url;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct Crawler {
    pub config: super::config::ScraperConfig,
    pub scraper: Scraper,
    visited_urls: HashSet<String>,
    max_connections: Arc<Semaphore>,
}

impl Crawler {
    pub fn new(config: ScraperConfig, max_connections: usize) -> Self {
        Crawler {
            config: config.clone(),
            scraper: Scraper::new(config),
            visited_urls: HashSet::new(),
            max_connections: Arc::new(Semaphore::new(max_connections)),
        }
    }

    pub async fn run(&mut self, selectors: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let mut queue: Vec<String> = vec![self.config.base_url.clone()];

        // Get the current timestamp and format it
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();

        while let Some(url) = queue.pop() {
            if self.visited_urls.insert(url.clone()) {
                let _permit = self.max_connections.acquire().await;

                println!("Visiting: {}", url);
                let response = reqwest::get(&url).await?;
                let html = response.text().await?;
                let links = self.extract_links(&html);

                if self.config.full_download {
                    self.save_html(&url, &html)?;
                }

                for link in links {
                    if !self.visited_urls.contains(&link) {
                        queue.push(link.clone());
                        self.save_crawl_results_to_file(&[link], &timestamp).await?;
                    }
                }

                let scraper = Scraper::new(self.config.clone());
                scraper.scrape_data(selectors).await?;

                drop(_permit);
            }
        }

        Ok(())
    }

    fn extract_links(&self, html: &str) -> Vec<String> {
        let document = Html::parse_document(html);
        let a_selector = Selector::parse("a").unwrap();

        let base_url = Url::parse(&self.config.base_url).unwrap();
        let base_domain = base_url.domain().unwrap();

        println!("Staying within domain: {}", base_domain);

        document
            .select(&a_selector)
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
            .map(|link| {
                Url::parse(&self.config.base_url)
                    .unwrap()
                    .join(link)
                    .unwrap()
            })
            .filter(|url| url.domain().map_or(false, |domain| domain == base_domain))
            .map(|url| url.to_string())
            .collect()
    }

    fn save_html(&self, url: &str, html: &str) -> std::io::Result<()> {
        // Create the downloads folder if it doesn't exist
        let folder_path = Path::new(&self.config.download_folder);
        if !folder_path.exists() {
            fs::create_dir_all(folder_path)?;
        }

        let parsed_url = Url::parse(url).unwrap();
        let file_path = parsed_url.path();
        let file_extension = Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("html");

        // Change the format of the file name to include the folder and file extension
        let file_name = format!(
            "{}/{}_{}.{}",
            self.config.download_folder,
            url.replace(":", "_").replace("/", "_"),
            "file",
            file_extension
        );
        let mut file = File::create(file_name)?;
        file.write_all(html.as_bytes())?;

        Ok(()) // Return an empty Ok result
    }

    pub async fn save_crawl_results_to_file(
        &self,
        results: &[String],
        timestamp: &str,
    ) -> std::io::Result<()> {
        // Create the "Results" directory if it doesn't exist
        fs::create_dir_all("Results")?;

        // Create the file path with the timestamp and the folder
        let file_path = format!("Results/{}_crawl_results.txt", timestamp);

        // Open the file with the new file path
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path)?;

        // Write crawl results to the file
        for result in results {
            writeln!(file, "{}", result)?;
        }

        Ok(())
    }
}
