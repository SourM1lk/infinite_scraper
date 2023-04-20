use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use url::{ParseError, Url};

pub struct Crawler {
    client: Client,
    base_url: Url,
}

impl Crawler {
    pub fn new(base_url: &str) -> Result<Self, ParseError> {
        let base_url = Url::parse(base_url)?;

        Ok(Self {
            client: Client::new(),
            base_url,
        })
    }

    pub async fn crawl(&mut self, path: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let url = self.base_url.join(path)?;

        let response = self.client.get(url.clone()).send().await?;
        let body = response.text().await?;

        let links = self.extract_links(&body);
        Ok(links)
    }

    fn extract_links(&self, body: &str) -> Vec<String> {
        let document = Html::parse_document(body);
        let selector = Selector::parse("a").unwrap();
        let base_url = &self.base_url;

        document
            .select(&selector)
            .filter_map(|element| element.value().attr("href"))
            .filter_map(|link| base_url.join(link).ok())
            .filter(|url| url.domain() == base_url.domain())
            .map(|url| url.to_string())
            .collect()
    }
}
