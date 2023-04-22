use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = "Web Scraper",
    about = "A Rust-based web scraper for a specific domain/address.",
)]
pub struct CliOptions {
    #[structopt(long = "base_url", short = "u")]
    pub base_url: String,

    #[structopt(long = "start_path", short = "s", default_value = "/")]
    pub start_path: String,

    #[structopt(long = "crawl")]
    pub crawl: bool,

    #[structopt(long = "scrape")]
    pub scrape: bool,

    #[structopt(long = "list_selectors")]
    pub list_selectors: bool,

    #[structopt(long, help = "CSS selectors to use for scraping data, separated by commas")]
    pub scrape_selectors: Option<String>,
   
    #[structopt(long)]
    pub include_duplicates: bool,
}

#[derive(Clone)]
pub struct ScraperConfig {
    pub base_url: String,
    pub start_path: String,
}

impl ScraperConfig {
    pub fn from_options(options: CliOptions) -> Result<ScraperConfig, &'static str> {
        Ok(ScraperConfig {
            base_url: options.base_url,
            start_path: options.start_path,
        })
    }
}
