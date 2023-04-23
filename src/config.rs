use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = "Mellen Collie and the Infinite Scraper",
    about = "A Rust-based web scraper and crawler. Name idea by: Hoax",
    author = "SourMilk"
)]
pub struct CliOptions {
    #[structopt(
        long = "base_url",
        short = "u",
        help = "Base URL to start scraping or crawling from. (e.g., 'https://example.com')"
    )]
    pub base_url: String,

    #[structopt(
        long = "start_path",
        short = "s",
        default_value = "/",
        help = "Starting path for the scraper or crawler. Defaults to '/'."
    )]
    pub start_path: String,

    #[structopt(
        long = "crawl",
        help = "Enable crawling mode, following links within the website."
    )]
    pub crawl: bool,

    #[structopt(
        long = "scrape",
        help = "Enable scraping mode, extracting data from the page using CSS selectors or regex."
    )]
    pub scrape: bool,

    #[structopt(
        long = "list_selectors",
        help = "List all unique CSS selectors found on the page."
    )]
    pub list_selectors: bool,

    #[structopt(
        long,
        help = "Provide a list of CSS selectors to use for scraping data, separated by commas (e.g., '.title, .price')."
    )]
    pub use_selectors: Option<String>,

    #[structopt(
        long,
        help = "Include duplicate CSS selectors in the list of selectors found on the page."
    )]
    pub include_duplicates: bool,

    #[structopt(
        long = "full-download",
        help = "Download the entire page, including all assets such as images and stylesheets."
    )]
    pub full_download: bool,

    #[structopt(
        long = "download-folder",
        parse(try_from_str),
        help = "Specify the folder where all downloaded pages and assets will be saved."
    )]
    pub output_folder: Option<String>,

    #[structopt(
        long = "use-regex",
        help = "Provide a regex pattern to extract data from the page."
    )]
    pub use_regex: Option<String>,

    #[structopt(
        long = "interval",
        short = "i",
        help = "Repeat the scraper or crawler command after every specified interval in HH:MM:SS format (e.g., '01:30:00' for 1 hour 30 minutes)."
    )]
    pub interval: Option<String>,

    #[structopt(
        long = "max-connections",
        short = "C",
        default_value = "10",
        help = "Set the maximum number of concurrent connections for the scraper or crawler."
    )]
    pub max_connections: usize,

    #[structopt(
        long = "proxies",
        short = "p",
        help = "Use random proxies listed in the 'proxies.txt' file for each connection."
    )]
    pub use_proxies: bool,
}

#[derive(Clone)]
pub struct ScraperConfig {
    pub base_url: String,
    pub start_path: String,
    pub full_download: bool,
    pub download_folder: String,
    pub use_regex: bool,
    pub max_connections: usize,
}

impl ScraperConfig {
    pub fn from_options(options: CliOptions) -> Result<ScraperConfig, &'static str> {
        Ok(ScraperConfig {
            base_url: options.base_url,
            start_path: options.start_path,
            max_connections: options.max_connections,
            full_download: options.full_download,
            download_folder: options
                .output_folder
                .clone()
                .unwrap_or_else(|| "downloads".to_string()),
            use_regex: options.use_regex.is_some(),
        })
    }
}
