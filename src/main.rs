use clap::Parser;
use scraper::{Html, Selector};
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

    // 標準エラー出力に、どのURLを取得しているか表示
    eprintln!("Fetching links from: {}", base_url);

    let response = reqwest::blocking::get(base_url.clone())?;
    let body = response.text()?;

    let document = Html::parse_document(&body);
    // unwrap() をやめて、? でエラーを伝播させる
    let selector = Selector::parse("a[href]")
        .map_err(|e| format!("Failed to parse selector: {}", e))?;

    // 最終的なリンクは標準出力へ
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            match base_url.join(href) {
                Ok(joined_url) => println!("{}", joined_url),
                // 個別のリンク変換失敗は標準エラー出力に出すが、処理は続行
                Err(e) => eprintln!("Could not join URL: '{}'. Error: {}", href, e),
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}