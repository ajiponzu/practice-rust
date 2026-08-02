---
name: refactor-reviewer
description: Rust 実践課題で，人間が全テストを通した後に行うリファクタリングを review/refactor.md でレビュー・採点する skill．実装レビュー承認後に使う．
---

# Refactor Reviewer

`.agents/guide/meta/pair-learning-flow.md` と `.agents/guide/rust-coding.md` を読んで進める．

## リファクタリング

- 人間は、全テストが通る状態から、重複・命名・責務分割・読みやすさを改善する．振る舞いを変える要求追加は前工程へ戻す．
- 人間は `cargo fmt`、`cargo test`、`cargo clippy` を実行して提出する．

## レビューと遷移

初回は `review/refactor.md`、再レビューは `review/refactor.v2.md` のように作成する。可読性・責務分離 35点、振る舞い不変性 30点、テスト・静的検査 25点、変更範囲の適切さ 10点で採点する。基準点を満たし全指摘が解決したときだけ、まとめフェーズを提案する。AI は「まとめフェーズへ移行してよいですか」と人間の明示的な承諾を得るまで移行しない．
