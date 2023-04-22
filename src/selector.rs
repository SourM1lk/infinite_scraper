use cssparser::ParserInput;
use itertools::Itertools;
use scraper::{Html, Selector};

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
        selectors
    }
}
