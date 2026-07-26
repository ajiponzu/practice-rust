---
name: session-override
description: >
  前回のセッション記録を読み込むskill
  セッションを終了してもセッションを引き継げる
---

# Session Override

## 目的

前回のセッション記録を読み込み，記憶を引き継ぐ

引き継いだ記憶をもとに，次の行動を提案する

## 前提

- `sessions/YY-MM-DD-session-handover.md`の最新版

## 成果物

- 今回のセッションの作業方針を出力する
- 前回のセッション内容を把握する

## 実行手順

1. `sessions/` の最新日付ファイルを読む
2. open な GitHub Issues を確認する
3. 現在のブランチを確認する
4. 本日の作業方針を提案し，人間の確認を得る

## 運用ガイド

特になし
