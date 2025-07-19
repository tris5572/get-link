use clap::Parser;
use scraper::{Html, Selector};
use url::Url;

/// A CLI tool to extract links from a given URL.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The URL to extract links from.
    #[arg(short, long)]
    url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let base_url = Url::parse(&args.url)?;

    println!("Fetching links from: {}", base_url);

    let response = reqwest::blocking::get(base_url.clone())?;
    let body = response.text()?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a[href]").unwrap();

    println!("\nFound links:\n");

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            match base_url.join(href) {
                Ok(joined_url) => println!("{}", joined_url),
                Err(e) => eprintln!("Could not join URL: '{}'. Error: {}", href, e),
            }
        }
    }

    Ok(())
}
