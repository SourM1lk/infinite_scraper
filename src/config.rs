use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Web Scraper",
    about = "A Rust-based web scraper for a specific domain/address.",
)]
pub struct CliOptions {
    #[structopt(long = "base_url", short = "u")]
    pub base_url: String,

    #[structopt(long = "start_path", short = "s", default_value = "/")]
    pub start_path: String,
}

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
