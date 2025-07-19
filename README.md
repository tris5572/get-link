
# Get-Link

Get-Linkは、指定されたURLに含まれるリンク先を抽出するcliツールです。

Gemini Code Assistを使用して作ってみたテストです。

## 使い方

```sh
get-link <url> <options>
```

url: 対象となるURL

options

オプション | 名称 | 動作
:--: | :--: | :--:
`-dedupe` | 重複排除 | 重複したリンク先を削除する
`-sort` | ソート | 結果をソートして返す

