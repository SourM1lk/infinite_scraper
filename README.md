# Mellon Collie and the Infinite Scraper
A simple yet powerful web scraper and crawler built with Rust.

## Features
- Web crawling and following links within a website.
- Web scraping using CSS selectors or regex patterns.
- Listing unique CSS selectors found on a page.
- Optional inclusion of duplicate CSS selectors.
- Downloading an entire page, including all assets.
- Saving downloaded pages and assets to a specified folder.
- Interval-based repeating of scraper or crawler commands.
- Configurable maximum number of concurrent connections.
- Optional use of proxies for each connection.

## Installation

### Prerequisites

Ensure you have the following installed on your system:

- Rust: Install Rust and its package manager, Cargo, by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

### Clone and Build

1. Clone the repository:
```
https://github.com/SourM1lk/infinite_scraper.git
```
2. Change to the project directory:
```
 cd infinite_scraper
```

3. Build the project:
```
cargo build
```
You should now have a compiled binary in the `target/debug` directory. You can use the binary to run the scraper and crawler:


## Command-Line Options
```
USAGE:
    infinite_scraper [FLAGS] [OPTIONS] --base_url <base-url>

FLAGS:
        --crawl                 Enable crawling mode, following links within the website.
    -F, --full-download         Download the entire page, including all assets such as images and stylesheets.
    -h, --help                  Prints help information
    -D, --include_duplicates    Include duplicate CSS selectors in the list of selectors found on the page.
    -L, --list_selectors        List all unique CSS selectors found on the page.
        --scrape                Enable scraping mode, extracting data from the page using CSS selectors or regex.
    -P, --proxies               Use random proxies listed in the 'proxies.txt' file for each connection.
    -V, --version               Prints version information

OPTIONS:
    -u, --base_url <base-url>                  Base URL to start scraping or crawling from. (e.g.,
                                               'https://example.com')
    -I, --interval <interval>                  Repeat the scraper or crawler command after every specified interval in
                                               HH:MM:SS format (e.g., '01:30:00' for 1 hour 30 minutes).
    -C, --max-connections <max-connections>    Set the maximum number of concurrent connections for the scraper or
                                               crawler. [default: 10]
        --download-folder <output-folder>      Specify the folder where all downloaded pages and assets will be saved.
    -s, --start_path <start-path>              Starting path for the scraper or crawler. Defaults to '/'. [default: /]
    -R, --use-regex <use-regex>                Provide a regex pattern to extract data from the page.
    -S, --use_selectors <use-selectors>        Provide a list of CSS selectors to use for scraping data, separated by
                                               commas (e.g., '.title, .price').
```

## Examples
```
# Crawl the site
$ ./target/debug/infinite_scraper --base_url https://example.com --crawl

# Scrape data using CSS selectors
$ ./target/debug/infinite_scraper --base_url https://example.com --scrape --use_selectors ".title, .description"

# List CSS selectors for the page
$ ./target/debug/infinite_scraper --base_url https://example.com --list_selectors

# Include duplicate CSS selectors in the list
$ ./target/debug/infinite_scraper --base_url https://example.com --list_selectors --include_duplicates

# Scrape data using regex patterns
$ ./target/debug/infinite_scraper --base_url https://example.com --scrape --use-regex "Title: (.*?)\\n"

# Run the scraper every 1 hour, 30 minutes, and 0 seconds
$ ./target/debug/infinite_scraper --base_url https://example.com --scrape --use_selectors ".title, .description" --interval 01:30:00

# Crawl using proxies
$ ./target/debug/infinite_scraper --base_url https://example.com --crawl --proxies

# Limit the maximum number of concurrent connections to 5
$ ./target/debug/your_binary_name --base_url https://example.com --crawl --max-connections 5
```