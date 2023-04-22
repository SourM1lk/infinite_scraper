use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = "Web Scraper",
    about = "A Rust-based web scraper and crawler."
)]
pub struct CliOptions {
    #[structopt(long = "base_url", short = "u", help = "Base URL to start scraping or crawling from")]
    pub base_url: String,

    #[structopt(long = "start_path", short = "s", default_value = "/")]
    pub start_path: String,

    #[structopt(long = "crawl", help = "Crawl the site")]
    pub crawl: bool,

    #[structopt(long = "scrape", help = "Scrape data from the page using CSS selectors or regex")]
    pub scrape: bool,

    #[structopt(long = "list_selectors", help = "List CSS selectors for the page")]
    pub list_selectors: bool,

    #[structopt(long, help = "CSS selectors to use for scraping data, separated by commas")]
    pub use_selectors: Option<String>,

    #[structopt(long)]
    pub include_duplicates: bool,

    #[structopt(long = "full-download", help = "Download the page and all its assets")]
    pub full_download: bool,

    #[structopt(long = "output-folder", parse(try_from_str))]
    pub output_folder: Option<String>,

    #[structopt(long = "use-regex", help = "Use regex to extract data")]
    pub use_regex: Option<String>,
}

#[derive(Clone)]
pub struct ScraperConfig {
    pub base_url: String,
    pub start_path: String,
    pub full_download: bool,
    pub output_folder: String,
    pub use_regex: bool,
}

impl ScraperConfig {
    pub fn from_options(options: CliOptions) -> Result<ScraperConfig, &'static str> {
        Ok(ScraperConfig {
            base_url: options.base_url,
            start_path: options.start_path,
            full_download: options.full_download,
            output_folder: options
                .output_folder
                .clone()
                .unwrap_or_else(|| "downloads".to_string()),
            use_regex: options.use_regex.is_some(),
        })
    }
}
