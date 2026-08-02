# ディレクトリ

## ディレクトリ構成

```
practice-rust/           # 課題プロジェクトを置くワークスペース
├── .agents/
│   ├── skills/          # Codex が利用するプロジェクト固有 skill
│   └── guide/           # Codex 向けプロジェクトガイド
│       ├── meta/        # プロジェクト全体の進め方・責任分界
│       ├── commit-message.md
│       ├── mermaid-style.md
│       ├── rust-coding.md
│       └── text-style.md
├── <crate-name>/        # 課題ごとに `cargo new` で作成
│   ├── src/
│   ├── docs/
│   │   ├── requirements.md # 人間が書く要求分析
│   │   ├── design.md    # 人間が書く仕様書・設計メモ
│   │   └── testcase.md   # 人間が書くテストケース
│   ├── tests/           # AI が指定した結合・全体テスト
│   └── review/
│       ├── design.md    # AI が記載する設計レビュー
│       ├── requirements.md # AI が記載する要求分析レビュー
│       ├── implement.md # AI が記載する実装レビュー
│       ├── testcase.md  # AI が記載するテストケース設計レビュー
│       ├── spec.md      # AI が記載する設計・テストケース整合レビュー
│       ├── testcode.md  # AI が記載するテストコードレビュー
│       └── refactor.md  # AI が記載するリファクタリングレビュー
│   └── summary.md       # AI が記載する全工程の総括
└── sessions/            # セッション記録（必要な場合，git管理しない）
```
