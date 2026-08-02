# Rust 練習プロジェクト

このリポジトリは，Rust を手を動かして学ぶための学習用ワークスペースです．

## 基本方針

- AI はアドバイザ・レビュアー・採点者に徹し，原則として完成コードや完成文書を先回りして作らない．
- 学習者が自分で考え，設計・実装・検証・説明する余地を残す．
- Rust の文法練習を依頼された場合は `rust-snippet-generator` を使う．
- 短い実践課題を依頼された場合は `rust-practice-challenge` を使い，テスト駆動開発（TDD）で人間主導に進める．
- 実践課題ごとに，課題カードの承認後に新しい Cargo プロジェクトを作成する．AI は順に `docs/requirements.md`，`docs/design.md`，`docs/testcase.md` をテンプレートから作成し，人間が記入する．
- 設計レビューでは，AI がプロジェクト直下の `review/design.md` に採点と回答・差し戻し表を記載する．
- テストケース設計のレビューでは，AI が `review/testcase.md` に採点と回答・差し戻し表を記載する．
- テストコード実装は人間が行う。レビュー依頼後，AI は `review/testcode.md` に採点と回答・差し戻し表を記載する．
- 実装は設計結果と承認済みテストコードを基に人間が行う。レビュー依頼後，AI は `review/implement.md` に採点と回答・差し戻し表を記載する．
- 実装レビューの後に，人間がリファクタリングを行い、`refactor-reviewer` が `review/refactor.md` へレビューを記載する．
- リファクタリングレビューの承認後，AI は各課題プロジェクト直下の `summary.md` に，初回提出時の個別点，修正後の完成度，レビュー対応，良かった点，ネック，次回への助言を記載する．
- AI は設計フェーズへの移行時に図の種類を，テストケース設計フェーズへの移行時に `src/` 内の単体テスト，`tests/` の統合テスト，必要に応じた全体・システムテストの必要性を指定する．
- AI は採点と必須条件を確認して次工程を提案するが，人間の明示的な承諾なしに工程を移行しない．
- Markdown の記法練習を依頼された場合は `markdown-sample-generator` を使う．
- ペアプロの進め方は `.agents/guide/meta/pair-learning-flow.md` を参照する．
- 実装やレビューでは `.agents/guide/rust-coding.md` を参照する．
- GitHub Issue とブランチ運用は，必要になった場合だけ導入する．

## 中心 Skill

通常の Rust 学習では，`rust-practice-challenge` を中心のオーケストレーターとして使う．
この Skill が短い実践課題を出題し，次の順序で人間主導の学習を進める．

1. 人間による要求分析 → `requirements-reviewer` による要求分析レビュー
2. 人間による設計 → `design-reviewer` による設計レビュー
3. 人間によるテストケース設計 → `test-design-reviewer` によるテスト設計レビュー
4. 人間による設計・テストケース見直し → `spec-reviewer` による整合レビュー
5. 人間によるテストコード実装 → `test-code-reviewer` によるテストコードレビュー
6. 人間による実装 → `implementation-reviewer` による実装レビュー
7. 人間によるリファクタリング → `refactor-reviewer` によるリファクタリングレビュー
8. `summary-reviewer` による全工程の総合評価 → 人間の承認で完了

各 Skill は単独でも利用できるが，通常の実践課題では `rust-practice-challenge` の段階として利用する．

## よく使うコマンド

```bash
cargo fmt
cargo check
cargo test
cargo clippy
```
