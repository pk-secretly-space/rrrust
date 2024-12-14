# rrrust

## Requirement
- MacOS
- Rust

## Usage
1. brew install rustup-init
2. rustup-init<br />
メッセージが出てくるので、Enter<br />
"Rust is installed now. Great!"が表示されたらOK.<br />
"rustup --version"でバージョン情報表示を確認できたら完了。
3. 'source "$HOME/.cargo/env"'、またはターミナルソフト再起動でcargoコマンド有効化
4. cargo new hello_cargo
5. cd hello_cargo
6. cargo run
7. "Hello, world!"

VScodeを使う場合
1. rustup component add rls rust-src rust-analysis
2. VScodeの拡張機能"rust-analyzer"をインストール

## Description
個人的にRustを学ぶところ

## Reference
Rustのインストール関連<br />
https://qiita.com/maoutokagura/items/c2fd85132bcec399c3a1

Rust チュートリアル<br/>
https://doc.rust-jp.rs/book-ja/title-page.html

## Auther
[Twitter(新X) @kazewatariperu](https://x.com/kazewatariperu)

## License
Copyright (c) 2024 Peru Kazewatari<br />
The source code is licensed MIT.<br />
The website content is licensed CC BY 4.0,see LICENSE.
