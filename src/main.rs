use clap::Parser;

/// 指定されたURLからリンクを抽出するCLIツール
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// リンクを抽出する対象のURL
    #[arg(short, long)]
    url: String,
}

fn main() {
    let args = Args::parse();

    println!("URL: {}", args.url);
}