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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_and_process_links_basic() {
        let html = r#"
            <a href="/path/to/page1.html">Page 1</a>
            <a href="https://example.com/page2.html">Page 2</a>
            <a href="/path/to/page1.html">Page 1 Duplicate</a>
        "#;
        let base_url = Url::parse("https://example.org/").unwrap();
        let args = Args {
            url: "".to_string(),
            sort: false,
            reverse: false,
            internal: false,
            external: false,
        };

        let links = extract_and_process_links(html, &base_url, &args).unwrap();
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].as_str(), "https://example.org/path/to/page1.html");
        assert_eq!(links[1].as_str(), "https://example.com/page2.html");
    }

    #[test]
    fn test_extract_and_process_links_sort() {
        let html = r#"
            <a href="/b.html">B</a>
            <a href="/a.html">A</a>
        "#;
        let base_url = Url::parse("https://example.org/").unwrap();
        let args = Args {
            url: "".to_string(),
            sort: true,
            reverse: false,
            internal: false,
            external: false,
        };

        let links = extract_and_process_links(html, &base_url, &args).unwrap();
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].as_str(), "https://example.org/a.html");
        assert_eq!(links[1].as_str(), "https://example.org/b.html");
    }

    #[test]
    fn test_extract_and_process_links_reverse() {
        let html = r#"
            <a href="/a.html">A</a>
            <a href="/b.html">B</a>
        "#;
        let base_url = Url::parse("https://example.org/").unwrap();
        let args = Args {
            url: "".to_string(),
            sort: false,
            reverse: true,
            internal: false,
            external: false,
        };

        let links = extract_and_process_links(html, &base_url, &args).unwrap();
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].as_str(), "https://example.org/b.html");
        assert_eq!(links[1].as_str(), "https://example.org/a.html");
    }

    #[test]
    fn test_extract_and_process_links_internal() {
        let html = r#"
            <a href="/internal.html">Internal</a>
            <a href="https://external.com/external.html">External</a>
        "#;
        let base_url = Url::parse("https://example.org/").unwrap();
        let args = Args {
            url: "".to_string(),
            sort: false,
            reverse: false,
            internal: true,
            external: false,
        };

        let links = extract_and_process_links(html, &base_url, &args).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].as_str(), "https://example.org/internal.html");
    }

    #[test]
    fn test_extract_and_process_links_external() {
        let html = r#"
            <a href="/internal.html">Internal</a>
            <a href="https://external.com/external.html">External</a>
        "#;
        let base_url = Url::parse("https://example.org/").unwrap();
        let args = Args {
            url: "".to_string(),
            sort: false,
            reverse: false,
            internal: false,
            external: true,
        };

        let links = extract_and_process_links(html, &base_url, &args).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].as_str(), "https://external.com/external.html");
    }

    #[test]
    fn test_extract_and_process_links_fragment() {
        let html = r#"
            <a href="/page.html#section1">Section 1</a>
            <a href="/page.html#section2">Section 2</a>
            <a href="/page.html">No Fragment</a>
        "#;
        let base_url = Url::parse("https://example.org/").unwrap();
        let args = Args {
            url: "".to_string(),
            sort: false,
            reverse: false,
            internal: false,
            external: false,
        };

        let links = extract_and_process_links(html, &base_url, &args).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].as_str(), "https://example.org/page.html");
    }
}