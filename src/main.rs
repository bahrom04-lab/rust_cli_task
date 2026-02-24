mod config;
mod error;
mod test;

use crate::config::Cli;
use crate::error::CustomError;

use clap::Parser;
use serde::Deserialize;

static BINARY_DATA: &str = include_str!("../data/quotes.json");

#[derive(Deserialize, Debug, PartialEq, Clone)]
struct Quote {
    author: String,
    quote: String,
}

impl Default for Quote {
    fn default() -> Self {
        Self {
            author: "Unknown".to_string(),
            quote: "Some wise words.".to_string(),
        }
    }
}

impl Quote {
    fn new(author: String, quote: String) -> Self {
        Self { author, quote }
    }
}

#[derive(Deserialize)]
struct Quotes(Vec<Quote>);

impl Quotes {
    fn restore<T>(path: T) -> Result<Self, CustomError>
    where
        T: AsRef<str>,
    {
        let data: Vec<Quote> =
            serde_json::from_str(path.as_ref()).map_err(CustomError::JsonParse)?;

        Ok(Self(data))
    }

    fn author_quotes<T: AsRef<str>>(author: T, quotes: Vec<Quote>) -> Self {
        let author = author.as_ref().trim_start_matches("--");
        let _quotes = &quotes
            .into_iter()
            .filter(|q| q.author.eq(author))
            .collect::<Vec<Quote>>();

        Self(_quotes.to_owned())
    }
}
fn main() {
    let args: Cli = Cli::parse();
    let quotes = Quotes::restore(BINARY_DATA);

    let all_quotes = quotes.unwrap_or_else(|error| panic!("aaaa {error:?}"));

    let author_quotes = Quotes::author_quotes(&args.author, all_quotes.0);
    println!("{} says these - {:#?}", &args.author, author_quotes.0);
}
