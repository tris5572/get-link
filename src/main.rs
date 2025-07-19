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

    let mut unique_urls_in_order = Vec::new();
    let mut seen_urls = HashSet::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(original_url) = base_url.join(href) {
                let mut url_without_fragment = original_url.clone();
                url_without_fragment.set_fragment(None);

                // HashSetへの追加が成功した場合（= まだ見ていないURLの場合）
                if seen_urls.insert(url_without_fragment) {
                    // 順序を保持するVecに元のURLを追加
                    unique_urls_in_order.push(original_url);
                }
            }
        }
    }

    for url in unique_urls_in_order {
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