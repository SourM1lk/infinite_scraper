use reqwest::Client;
use scraper::{Html, Selector};
use std::env;
use std::error::Error;

async fn fetch_links(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    let links = extract_links(&body);
    Ok(links)
}

fn extract_links(body: &str) -> Vec<String> {
    let document = Html::parse_document(body);
    let selector = Selector::parse("a").unwrap();

    document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .map(|link| link.to_string())
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("BASE_URL").expect("BASE_URL is not set");
    let start_path = env::var("START_PATH").expect("START_PATH is not set");
    let url = format!("{}{}", base_url, start_path);

    let links = fetch_links(&url).await?;

    for link in links {
        println!("{}", link);
    }

    Ok(())
}
