# プロジェクトの進め方

## フロー概要

```mermaid
flowchart TD
    A[作業開始] --> B[作業種別と目標の選択]
    B --> C[課題・完了条件・採点基準]
    C --> D[要求分析・初回レビュー]
    D --> R[未決事項の合意・要求確定]
    R --> E[設計・レビュー]
    E --> F[テストケース設計・レビュー]
    F --> G[設計・テストケース整合レビュー]
    G --> H[テストコード実装・レビュー]
    H --> I[実装・レビュー]
    I --> J[リファクタリング・レビュー]
    J --> K[総合評価]
    K --> L{次の練習}
    L -->|あり| A
    L -->|なし| M[完了]
```

## 詳細フロー

各フェーズの手順は対応する Skill に記述されている．

| フェーズ | Skill |
|---|---|
| 文法・コード読解 | `rust-snippet-generator` |
| 短い実践課題の出題・進行 | `rust-practice-challenge` |
| Markdown・図による説明 | `markdown-sample-generator` |
| 設計のレビュー・採点 | `design-reviewer` |
| 要求分析のレビュー・採点 | `requirements-reviewer` |
| 未決事項の合意・要求確定 | `requirements-reviewer` |
| テスト設計のレビュー・採点 | `test-design-reviewer` |
| 設計・テストケース整合のレビュー・採点 | `spec-reviewer` |
| 実装のレビュー・採点 | `implementation-reviewer` |
| テストコードのレビュー・採点 | `test-code-reviewer` |
| リファクタリングのレビュー・採点 | `refactor-reviewer` |
| 全工程の総合評価 | `summary-reviewer` |

## 進捗管理

- 必要に応じて，学習メモや Git の履歴で管理する．
