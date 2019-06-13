# phosphor

---

[![Build Status](https://travis-ci.org/ekuinox/phosphor.svg?branch=master)](https://travis-ci.org/ekuinox/phosphor)

## これは何

- Rustでブログをホストするやつを作ってみたい
- 気分でやってる
- 設計ができない

## 必要

- SQlite3 3.16.2
- Rust 1.36.0-nightly

## 導入

1. `git clone git@github.com:ekuinox/phophor.git && cd ./phosphor`
2. `export DATABASE_URL=<DATABASE_URL>`
3. `cargo install diesel_cli --features "sqlite3"`
4. `diesel migration run`
5. `cargo run`
