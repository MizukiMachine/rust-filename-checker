# Asset File Name Checker in Rust

このリポジトリは、ゲーム開発におけるアセットファイルの命名規則をチェックするRust製のツールを提供します。
Zenn記事「超入門!Rustで作るアセット管理ツール～アセット命名規則チェッカー～」のサンプルコードです。


## 関連記事

- [超入門!Rustで作るアセット管理ツール～アセット命名規則チェッカー～](https://zenn.dev/nezumizuki/articles/3fe59fbf7923b1)

## 機能

- 指定されたディレクトリ内のファイルを再帰的にチェック
- 正規表現によるファイル名パターンの検証
- 許可された拡張子のチェック
- エラー内容の詳細な報告

## 必要要件

- Rust（最新の安定版を推奨）
- Cargo（Rustのパッケージマネージャー）

## セットアップ

```bash
# リポジトリのクローン
git clone https://github.com/MizukiMachine/rust-filename-checker.git
cd asset-file-checker

# 依存パッケージのインストール & ビルド
cargo build
```

## 使い方

1. チェック対象のディレクトリを準備
```bash
mkdir checktarget
# チェックしたいファイルを checktarget ディレクトリに配置
```

2. プログラムを実行
```bash
cargo run
```

### デフォルトの命名規則

- ファイル名は小文字で始まる
- 小文字のアルファベット、数字、アンダースコアのみ使用可能
- 許可される拡張子: png, jpg

例：
- ✅ `player_sprite.png`
- ✅ `background_01.jpg`
- ❌ `Player.png`
- ❌ `background.gif`

## カスタマイズ

命名規則を変更するには、`src/main.rs`の以下の部分を編集します：

```rust
let checker = AssetChecker::new(
    r"^[a-z][a-z0-9_]*$",  // 正規表現パターンを変更
    vec!["png".to_string(), "jpg".to_string()], // 許可する拡張子を変更
);
```

## プロジェクト構造

```
asset-file-checker/
├── src/
│   └── main.rs          # メインのソースコード
├── Cargo.toml           # 依存関係の設定
├── Cargo.lock           # 依存関係のバージョン固定
└── checktarget/         # チェック対象ディレクトリ
```

## 今後の展望

- [ ] 設定ファイルによるルール定義
- [ ] より詳細なレポート出力
- [ ] 複数のルールセットのサポート
- [ ] CIパイプラインへの統合サンプル
- [ ] バッチ処理モードの追加

## ライセンス

MIT

## 貢献について

Issue、Pull Requestは大歓迎です。

特に以下の領域での貢献を歓迎します
- 新しい命名規則パターンの追加
- パフォーマンスの改善
- エラーメッセージの改善
- ドキュメントの充実

