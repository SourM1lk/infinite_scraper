use chrono::prelude::*;
use cssparser::ParserInput;
use itertools::Itertools;
use scraper::{Html, Selector};
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct SelectorExtractor;

impl SelectorExtractor {
    pub fn new() -> Self {
        SelectorExtractor
    }

    pub fn extract_css_selectors(&self, html: &str, show_duplicates: bool) -> Vec<String> {
        let parsed_html = Html::parse_document(html);
        let style_selector = Selector::parse("style").unwrap();
        let mut selectors = Vec::new();

        for style_element in parsed_html.select(&style_selector) {
            let style_text = style_element.inner_html();

            let mut input = ParserInput::new(&style_text);
            let mut parser = cssparser::Parser::new(&mut input);

            while let Ok(token) = parser.next() {
                match token {
                    cssparser::Token::Ident(ref ident) => {
                        let selector = ident.as_ref().to_string();
                        selectors.push(selector);
                    }
                    _ => (),
                }
            }
        }

        let selectors: Vec<String> = if show_duplicates {
            selectors
        } else {
            selectors.into_iter().unique().collect()
        };
        println!("Selectors extracted: {}", selectors.len());
        self.save_selectors_to_file(&selectors).unwrap();

        selectors
    }

    pub fn save_selectors_to_file(&self, selectors: &[String]) -> std::io::Result<()> {
        // Create the "Results" directory if it doesn't exist
        fs::create_dir_all("Results")?;

        // Get the current timestamp and format it
        let timestamp = Local::now().format("%Y%m%d%H%M%S");

        // Create the file path with the timestamp and the folder
        let file_path = format!("Results/{}_selectors.txt", timestamp);

        // Open the file with the new file path
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path)?;

        // Write selector results to the file
        for selector in selectors {
            writeln!(file, "{}", selector)?;
        }

        Ok(())
    }
}
