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

    #[structopt(long = "selectors")]
    pub selectors: bool,

    #[structopt(long = "css_selectors", default_value = "")]
    pub css_selectors: String,

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
