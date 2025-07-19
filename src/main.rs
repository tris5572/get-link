use clap::Parser;
use scraper::{Html, Selector};

/// 指定されたURLからリンクを抽出するCLIツール
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// リンクを抽出する対象のURL
    #[arg(short, long)]
    url: String,
}

fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    println!("Fetching links from: {}", args.url);

    let response = reqwest::blocking::get(&args.url)?;
    let body = response.text()?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a[href]").unwrap();

    println!("\nFound links:\n");

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            println!("{}", href);
        }
    }

    Ok(())
}
