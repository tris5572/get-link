
# Get-Link

Get-Linkは、指定されたURLに含まれるリンク先を抽出するcliツールです。

Gemini Code Assistを使用して作ってみたテストです。

## インストール

### 前提条件

Rustがインストールされている必要があります。Rustのインストール方法については、[Rust公式ウェブサイト](https://www.rust-lang.org/ja/tools/install)を参照してください。

### `cargo install` を使用したインストール

GitHubリポジトリから直接インストールするには、以下のコマンドを実行します。

```sh
cargo install --git https://github.com/tris5572/get-link.git
```

これにより、`get-link`コマンドが `$HOME/.cargo/bin` (または環境変数`CARGO_HOME`で指定されたパス) にインストールされ、パスが通っていればどこからでも実行できるようになります。

### ソースからのビルドと実行 (開発者向け)

1.  **リポジトリをクローンします:**

    ```sh
    git clone https://github.com/tris5572/get-link.git
    cd get-link
    ```

2.  **プロジェクトをビルドします:**

    ```sh
    cargo build --release
    ```

    これにより、`target/release/get-link`に実行可能ファイルが生成されます。

3.  **アプリケーションを実行します:**

    ```sh
    ./target/release/get-link <url> [options]
    ```

    または、`cargo run`を使用することもできます（この場合、自動的にビルドされます）。

    ```sh
    cargo run -- <url> [options]
    ```

## 使い方

```sh
get-link <url> <options>
```

`url` : 対象となるURL

基本的には登場順で、重複を排除したリンク先のリストを返す。

`options`

オプション | 名称 | 動作
-- | -- | --
`--sort` | ソート | 結果を文字列でソートして返す
`--reverse` `-r` | 逆順 | 結果を逆順で返す
`--internal` `-i` | 内部リンク | 内部リンクのみを返す
`--external` `-e` | 外部リンク | 外部リンクのみを返す

