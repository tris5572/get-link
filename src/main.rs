use clap::Parser;

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

    let response = reqwest::blocking::get(args.url)?;
    let body = response.text()?;

    println!("\nResponse Body:\n");
    println!("{}", body);

    Ok(())
}
