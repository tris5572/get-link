
# Get-Link

Get-Linkは、指定されたURLに含まれるリンク先を抽出するcliツールです。

Gemini Code Assistを使用して作ってみたテストです。

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

