# 仕様書・設計メモ

簡単な運用ログの集計を行うcliツールを開発する

## 目的と背景

ログ内容を素早く把握するため，ログレベル別の件数を集計する機能を持ったcliツールを開発する

## 要求と受け入れ条件

- 下記形式のログ（複数行）を読込み，ログレベルごとに集計する

```
INFO server started
WARN cache is almost full
INFO request completed
ERROR database unavailable
WARN retry scheduled
```

- 各行のログは先頭語にログレベルが付与される．ログメッセージはログレベルと空白の後から始まる
- 空行は無視する, 読み飛ばす
- ログレベルは下記の三種類であり，それ以外は未知のログレベルすなわちエラーとして扱う(`ERROR`とは意味が違う，Rustのエラー)
  - `INFO`
  - `WARN`
  - `ERROR`
- 未知のログレベルが混入していた場合は，ログのインデックスとログレベル名をエラー出力する
- ログレベルのみ記載されていた場合は，許容されたログレベルなら集計に含め，未許容ならエラーとする
  - ログとして体をなさないため，別の不正エラーとすべきだが，このcliツールの責務から外れるためこのような方針とする
- 集計結果は下記の形式で表示する

```
INFO: 10
WARN: 5
ERROR: 3
```

- 実装後に`cargo fmt`, `cargo clippy`を実行し，コード品質を保証する

## 制約・前提

- 外部クレートを使用しない
- ファイル・モジュール分割を適切に行う
  - 特に集計処理はmain.rsに直接記載しない
- 未知レベル時のエラーはResult型のErrorを利用する
- ログレベルは，HashMapを使用するのではなく，ユーザ定義の構造体を設計して保持する

## モジュール構成

```
log-counter/
├── src/
│   ├── main.rs
│   └── log.rs
├── tests/
│   └── logs/
│      ├── input1.txt
│      └── input2.txt
├── Cargo.toml
└── Cargo.lock
```

## データと状態

### LevelCounter

- ログレベル三つに対応したカウンタを三つを持つ
  - `info_count`, `warn_count`, `error_count`
- 出力メソッド`display`を持つ

| 関数名 | 引数 | 戻り値 | 説明 |
| `display` | | String | ログレベルカウンタの内容を一括表示する |

```
INFO: {info_count}
WARN: {warn_count}
ERROR: {error_count}
```

### UnknownLevel

- 未知のログレベルのログインデックスとログレベルを保持する
  - `index`, `level`
  - `index`はログファイルを想定しているため，1始まりとする
- 出力メソッド`display`

| 関数名 | 引数 | 戻り値 | 説明 |
| `display` | | String | 未知のログレベルの内容を表示する |

```
log #{index}: {level}
```

### エラー処理

- 未知のログレベルのログを検出した場合はUnknownLevelの配列に保持し，関数終了後に`UnknownLevelError`を作成して返す

#### UnknownLevelError

- `Vec<UnknownLevel>`型の`logs`を持つ
- `fmt::Display`トレイトの`fmt`関数について，`logs`の各要素の`display`結果を改行区切りで統合するよう実装する

```
Unknown log levels
***
{logs[xxx].display()}
...
***
Allowed log levels:
  INFO, WARN, ERROR
```

- `std::error::Error`を`UnknownLevelError`にデフォルト実装する

## API・利用例

| api名     | 引数          | 戻り値                                    |
| --------- | ------------- | ----------------------------------------- |
| aggregate | Vec\<String\> | Result\<LevelCounter, UnknownLevelError\> |

```rust
mod log;

fn main() {
    let logs = vec![
        "INFO server started".to_string(),
        "WARN not updated".to_string(),
        "ERROR memory leak".to_string(),
        "INFO server running...".to_string(),
    ];
    let result = log::aggregate(logs);
    match result {
        Ok(counter) => {
            println!("{}", counter.display());
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
```

## テスト可能性

- 単体テストを行う. 多様な入力パターンを効率的に検証するために，testsディレクトリに，ログファイルを複数用意し，ファイル読み込み関数を用いた検証コードも実装する
