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
    url: String,

    /// Sort the output URLs alphabetically.
    #[arg(long)]
    sort: bool,

    /// Reverse the order of the output.
    #[arg(short, long)]
    reverse: bool,

    /// Extract only internal links.
    #[arg(short, long)]
    internal: bool,

    /// Extract only external links.
    #[arg(short, long)]
    external: bool,
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.internal && args.external {
        return Err("Cannot use --internal and --external flags simultaneously".into());
    }

    let base_url = Url::parse(&args.url)?;

    eprintln!("Fetching links from: {}", base_url);

    let response = reqwest::blocking::get(base_url.clone())?;
    let body = response.text()?;

    let links = extract_and_process_links(&body, &base_url, &args)?;

    for url in links {
        println!("{}", url);
    }

    Ok(())
}

fn extract_and_process_links(
    html_content: &str,
    base_url: &Url,
    args: &Args,
) -> Result<Vec<Url>, Box<dyn std::error::Error>> {
    let document = Html::parse_document(html_content);
    let selector = Selector::parse("a[href]")
        .map_err(|e| format!("Failed to parse selector: {}", e))?;

    let mut unique_urls_in_order = Vec::new();
    let mut seen_urls = HashSet::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(original_url) = base_url.join(href) {
                let mut url_without_fragment = original_url.clone();
                url_without_fragment.set_fragment(None);

                if args.internal {
                    if url_without_fragment.origin() != base_url.origin() {
                        continue;
                    }
                }

                if args.external {
                    if url_without_fragment.origin() == base_url.origin() {
                        continue;
                    }
                }

                if seen_urls.insert(url_without_fragment.clone()) {
                    unique_urls_in_order.push(url_without_fragment);
                }
            }
        }
    }

    // --sort オプションが指定されていたら、ソートする
    if args.sort {
        unique_urls_in_order.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    }

    // --reverse オプションが指定されていたら、順序を逆にする
    if args.reverse {
        unique_urls_in_order.reverse();
    }

    Ok(unique_urls_in_order)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}