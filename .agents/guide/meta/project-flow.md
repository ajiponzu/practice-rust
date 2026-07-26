# プロジェクトの進め方

## フロー概要

```mermaid
flowchart TD
    A[作業開始] --> B[作業種別と目標の選択]
    B --> C[課題・完了条件・採点基準]
    C --> D[人間が作業・提出]
    D --> E[AI がヒント・レビュー・採点]
    E --> F{次の練習}
    F -->|あり| A
    F -->|なし| G[完了]
```

## 詳細フロー

各フェーズの手順は対応する Skill に記述されている．

| フェーズ | Skill |
|---|---|
| 文法・コード読解 | `rust-snippet-generator` |
| 短い実践課題の出題・進行 | `rust-practice-challenge` |
| Markdown・図による説明 | `markdown-sample-generator` |
| 設計のレビュー・採点 | `design-reviewer` |
| テスト設計のレビュー・採点 | `test-design-reviewer` |
| 実装のレビュー・採点 | `implementation-reviewer` |
| テストコードのレビュー・採点 | `test-code-reviewer` |
| 全工程の総合評価 | `summary-reviewer` |

## 進捗管理

- 必要に応じて，学習メモや Git の履歴で管理する．
