# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 開発コマンド

### テストと品質保証
```bash
# コードフォーマット（CI必須）
cargo fmt --all

# リンター実行（CI では警告をエラーとして扱う）
cargo clippy --all-targets --all-features -- -D warnings

# 全テスト実行
cargo test --all

# リリースビルド
cargo build --release
```

### アプリケーションテスト
```bash
# 現在の設定を表示
cargo run -- config

# 日報作成（安全のため --dry-run を使用）
cargo run -- create-new-daily-report --date 2023-12-25 --dry-run

# ファイルを日付ディレクトリに格納
cargo run -- pack <移動元ディレクトリ> <移動先ディレクトリ> --dry-run

# 日付ディレクトリからファイルを取り出し
cargo run -- extract <移動元ディレクトリ> <移動先ディレクトリ> --dry-run
```

## アーキテクチャ概要

### コア設計パターン
アプリケーションは**モジュラーコマンドパターン**を採用：
- `src/lib.rs` が初期化を担当し、CUIシステムに処理を委譲
- `src/cui/` がclap deriveマクロを使用してコマンドライン処理を管理
- `src/commands/` に独立したコマンド実装を配置
- 全コマンドが安全な動作テスト用の `--dry-run` をサポート

### 主要コンポーネント

**設定システム（`src/commands/config.rs`）**
- `confy` クレートを使用したクロスプラットフォーム永続化設定
- strftime日付フォーマットを正規表現パターンに変換してファイルマッチング
- デフォルトパターン：ファイルプレフィックス `%Y%m%d_`、ディレクトリ構造 `%Y-%m\%Y-%m-%d`

**日付処理（`src/commands/mod.rs`）**
- `date_from_str()`: 様々な文字列フォーマットからの柔軟な日付解析
- `date_from_dir_path()`: ネストしたディレクトリパスからの日付抽出
- `has_date_prefix()`: ファイル名の既存日付プレフィックス検出
- 正規表現を使用したパターンベースのファイル・ディレクトリ操作

**コマンド構造：**
- `pack`: ファイルを `YYYY-MM/YYYY-MM-DD/` ディレクトリ構造に `YYYYMMDD_` プレフィックス付きで移動
- `extract`: pack操作の逆処理、ファイルをフラット構造に戻す
- `create`: テンプレートから日付置換による日報生成
- `config`: 現在の設定情報表示

### 依存関係とその用途
- **clap v4**: サブコマンド用deriveマクロによるCLI引数解析
- **chrono**: ファイル整理用の日付フォーマットと解析
- **confy**: serde シリアライゼーションによる設定ファイル管理
- **regex**: 日付フォーマット文字列から生成されるパターンマッチング
- **walkdir**: ファイル操作用の再帰的ディレクトリ走査
- **tokio**: 非同期ランタイム（実際の操作は主に同期的）

### ファイル整理ロジック
ツールは2つの主要概念で動作：
1. **日付プレフィックス付きファイル**: `YYYYMMDD_` プレフィックス付きファイル
2. **日付整理ディレクトリ**: `2023-12/2023-12-25/` のようなネスト構造

設定パターンがファイル名プレフィックスとディレクトリ構造の両方を制御し、strftimeフォーマット文字列から正規表現パターンを自動生成。

### CI/CDパイプライン
- **フォーマット**: `cargo fmt --all -- --check`（強制）
- **リンター**: 警告をエラーとして扱う `cargo clippy`
- **テスト**: 全ターゲットでの `cargo test --all`
- **クロスプラットフォームビルド**: Windows MSVC と Linux musl ターゲット
- **自動リリース**: バイナリパッケージングとGitHubリリースドラフト

### 開発時の注意事項
- 全ファイル操作がドライランモードをサポート - 常に `--dry-run` で先にテスト
- 設定は `confy` で管理され、正規表現パターンが自動生成される
- エラーハンドリングは全体を通じて `anyhow::Result` を一貫使用
- `commands/mod.rs` に日付操作用の包括的ユーティリティ関数を含む