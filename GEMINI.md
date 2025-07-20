# Project Overview: get-link

This project, `get-link`, is a Rust command-line interface (CLI) tool designed to extract links from a given URL.

## Purpose
`get-link` は、指定されたURLに含まれるリンク先を抽出し、そのリストを返すCLIツールです。重複するリンクは排除され、基本的には登場順でリストが返されます。

## Technologies Used
- **Rust**: The primary programming language.
- **clap**: For parsing command-line arguments.
- **reqwest**: For making HTTP requests to fetch web page content.
- **scraper**: For parsing HTML and extracting elements.
- **url**: For URL parsing and manipulation.

## Key Files
- `Cargo.toml`: Defines project metadata, dependencies, and build configurations.
- `src/main.rs`: Contains the core logic of the `get-link` application, including argument parsing, web fetching, HTML parsing, link extraction, and output formatting.
- `README.md`: Provides a general overview of the project, its purpose, and basic usage instructions.

## How to Build and Run

To build the project:
```bash
cargo build
```

To run the project:
```bash
cargo run -- <url> [options]
```

**Example Usage:**
```sh
cargo run -- https://example.com --sort
```

### Command-line Options:

| Option    | Description                               |
|-----------|-------------------------------------------|
| `<url>`   | The target URL from which to extract links. |
| `--sort`  | Sorts the extracted links alphabetically. |
| `--reverse`, `-r` | Reverses the order of the output links. |
