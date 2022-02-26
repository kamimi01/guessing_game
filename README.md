# Rustで数当てゲームを作る

- 1~10の数字を使った数当てゲーム
  - 元のチュートリアルは1〜100だったけど、それだと一向に当たらないので、1〜10にした

- 成果物

![](./Image/app.gif)

## 環境構築

### Rustの導入

1. homebrewまたはcurlコマンドを使用して、rustupをインストール

   `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

  - 以下が表示されればOK
```shell
Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, run:
source $HOME/.cargo/env
```

2. パスを通す

  - zshを使用している場合は、以下を`zshrc`に記載する
  - 記載したら、`source ~/.zshrc`を実行する（ターミナルの再起動でもOK）

```shell
source $HOME/.cargo/env
```

3. インストールされているかどうか確認する

```shell
# rustupのバージョン確認
rustup --version
# cargoのバージョン確認
cargo --version
```

### VSCodeでの環境構築

- 拡張機能のインストール
  - [Rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
    - Rustの補完が効くようになる
  - [better toml](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
    - 設定ファイル`Cargo.toml`を書きやすくするため

## その他便利ツール

- Rustのcomponentのインストール
  - rustfmt：自動フォーマット用
  - rustfix：コンパイラの警告を自動で修正する
  - clippy：静的解析ツール（rustfixとの違いはまだよくわからない）

- 今回はVSCodeの機能でcomponentをインストールしますか→Yesでインストールした

- 参考
  - [付録D - 便利な開発ツール](https://doc.rust-jp.rs/book-ja/appendix-04-useful-development-tools.html)

### よく使うコマンド

- `cargo check`：コンパイルできるかどうかの確認
- `cargo run`：コンパイルと実行どっちもやってくれる
  - `cargo build`：コンパイルだけする場合は、このコマンド
- `cargo fmt`：フォーマットのエラーを教えてくれる
- `cargo fix`：フォーマットのエラーを直してくれる
- `cargo clippy --fix`：フォーマットのエラーを直してくれる

## 参考

- [数当てゲームをプログラムする](https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html)
- [rustup is an installer for
the systems programming language Rust](https://rustup.rs/)
- [Rustの環境構築 with VSCode](https://qiita.com/udayan28/items/e59afd39a7ab16911c25#vscode%E3%81%AE%E7%92%B0%E5%A2%83%E6%A7%8B%E7%AF%89)
- [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
- [Rust はじめに](https://www.rust-lang.org/ja/learn/get-started)
- [macOSでRustのローカル開発環境を整えるための手順2022](https://qiita.com/notakaos/items/9f3ee8a3f3a0caf39f7b)
- [Visual Studio CodeでRust開発環境を整える](https://qiita.com/84zume/items/377033ab6b6aee2a68d7)