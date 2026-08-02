---
name: requirements-reviewer
description: Rust 実践課題で，人間が記入した docs/requirements.md の要求分析を review/requirements.md でレビュー・採点する skill．課題カードを具体的な仕様へ分解した後に使う．
---

# Requirements Reviewer

`.agents/guide/meta/pair-learning-flow.md` を読んで進める．AI は人間の提出前に要求を代筆しない．

## 要求分析フェーズ

1. `docs/requirements.md` がなければ，AI は次のテンプレートを作成する．
2. 人間は利用者・入力・出力・正常系・異常系・制約・未決事項を記入して提出する．
3. AI は `review/requirements.md` を作成し、再レビューは `requirements.v2.md` のように新規作成する．

```md
# 要求分析

## 目的と利用者

## 入力と出力

## 機能要求

## 非機能要求・制約

## 受け入れ条件

## 未決事項
```

## レビュー

採点表は「目的・利用者の明確さ 20点」「要求・受け入れ条件の完全性 35点」「入力・出力・失敗時の明確さ 25点」「制約・未決事項の整理 20点」の100点満点とする．指摘表には既存の標準列を使う．

## 遷移

- 全指摘が `承認` または理由付き `却下` で基準点を満たしたときだけ、設計フェーズを提案する．
- AI は「設計フェーズへ移行してよいですか」と人間の明示的な承諾を得るまで移行しない．
