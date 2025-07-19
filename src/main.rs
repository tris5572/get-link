use clap::Parser;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::process;
use url::Url;

/// A CLI tool to extract links from a given URL.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The URL to extract links from.
    #[arg(short, long)]
    url: String,
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let base_url = Url::parse(&args.url)?;

    eprintln!("Fetching links from: {}", base_url);

    let response = reqwest::blocking::get(base_url.clone())?;
    let body = response.text()?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a[href]")
        .map_err(|e| format!("Failed to parse selector: {}", e))?;

    let mut unique_urls = HashSet::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(mut joined_url) = base_url.join(href) {
                // URLのフラグメント部分（#...）を削除
                joined_url.set_fragment(None);
                unique_urls.insert(joined_url);
            }
            // 不正なURLは単に無視する
        }
    }

    for url in unique_urls {
        println!("{}", url);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
