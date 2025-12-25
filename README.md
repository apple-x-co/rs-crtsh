# rs-crtsh

🔍 crt.sh API のための高速で使いやすい Rust 製コマンドラインツール

## 概要

`rs-crtsh` は、[crt.sh](https://crt.sh/) の証明書透明性ログを検索するためのコマンドラインツールです。  
指定したドメインに関連する SSL/TLS 証明書の情報を、見やすいテーブル形式で表示します。

## 特徴

- ✨ **見やすいテーブル表示**: 証明書情報を整理されたテーブル形式で表示
- 🚀 **高速**: Rust で実装された高性能な HTTP クライアント
- 🔄 **リトライ機能**: ネットワークエラー時の自動リトライに対応
- ⚙️ **柔軟な設定**: コマンドラインオプションと設定ファイルの両方に対応
- 📊 **詳細な情報**: タイミング情報やリクエスト/レスポンスの詳細を表示可能

## インストール

### ソースからビルド

```bash
# リポジトリをクローン
git clone https://github.com/apple-x-co/rs-crtsh.git
cd rs-crtsh

# リリースビルド
cargo build --release

# バイナリの場所
./target/release/rs-crtsh
```

### バイナリを PATH に追加（オプション）

```bash
# macOS/Linux
sudo cp target/release/rs-crtsh /usr/local/bin/

# または、シンボリックリンクを作成
sudo ln -s $(pwd)/target/release/rs-crtsh /usr/local/bin/rs-crtsh
```

## 使い方

### 基本的な使い方

```bash
# ドメインの証明書を検索
rs-crtsh --hostname example.com
```

### オプション一覧

| オプション                         | 短縮形  | 説明                                                                                                                                                                          | デフォルト |
|-------------------------------|------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------|
| `--hostname <HOSTNAME>`       | -    | 検索するホスト名/ドメイン                                                                                                                                                               | -     |
| `--config <CONFIG>`           | `-c` | 設定ファイルのパス                                                                                                                                                                   | -     |
| `--format <FORMAT>`           | `-f` | 出力形式（table, csv または raw）                                                                                                                                                    | table |
| `--column_name <COLUMN_NAME>` | -    | 表示するカラム名（複数指定可能）<br>指定可能な値: `id`, `common_name`, `entry_timestamp`, `issuer_ca_id`, `issuer_name`, `name_value`, `not_before`, `not_after`, `result_count`, `serial_number` | 全て表示  |
| `--preset <PRESET>`           | -    | 使用するプリセット名                                                                                                                                                                  | -     |
| `--retry <RETRY>`             | -    | リトライ回数                                                                                                                                                                      | 0     |
| `--retry-delay <RETRY_DELAY>` | -    | リトライ間隔（秒）                                                                                                                                                                   | 1.0   |
| `--timeout <TIMEOUT>`         | `-t` | タイムアウト時間（秒）                                                                                                                                                                 | 30    |
| `--timing`                    | -    | タイミング情報を表示                                                                                                                                                                  | false |
| `--verbose`                   | `-v` | 詳細な情報を表示                                                                                                                                                                    | false |
| `--help`                      | `-h` | ヘルプメッセージを表示                                                                                                                                                                 | -     |
| `--version`                   | `-V` | バージョン情報を表示                                                                                                                                                                  | -     |

## 実行例

### 基本的な検索

```bash
rs-crtsh --hostname example.com
```

**出力例:**

```
+-------------+------------------------------------------+-------------------------+--------------+--------------------------------------------------------------------------------------------------------------------+------------------------------------------+---------------------+---------------------+-------+------------------------------------------+
|  crt.sh ID  |           Matching Identities            |        Logged At        | Issuer CA ID |                                                    Issuer Name                                                     |                Name Value                |     Not Before      |      Not After      | Count |              Serial Number               |
+-------------+------------------------------------------+-------------------------+--------------+--------------------------------------------------------------------------------------------------------------------+------------------------------------------+---------------------+---------------------+-------+------------------------------------------+
| 23164227397 | example.com                              | 2025-12-16T21:50:43.225 |       204407 | C=GB, O=Sectigo Limited, CN=Sectigo Public Server Authentication CA DV E36                                         | *.example.com                            | 2026-03-16T20:59:52 | 2025-12-16T00:00:00 |     3 | 4b87ab08fde761c73d3c9f7a6a141bd3         |
|             |                                          |                         |              |                                                                                                                    | example.com                              |                     |                     |       |                                          |
+-------------+------------------------------------------+-------------------------+--------------+--------------------------------------------------------------------------------------------------------------------+------------------------------------------+---------------------+---------------------+-------+------------------------------------------+
| 23164227256 | example.com                              | 2025-12-16T21:50:41.574 |       204407 | C=GB, O=Sectigo Limited, CN=Sectigo Public Server Authentication CA DV E36                                         | *.example.com                            | 2026-03-16T20:59:52 | 2025-12-16T00:00:00 |     3 | 4b87ab08fde761c73d3c9f7a6a141bd3         |
|             |                                          |                         |              |                                                                                                                    | example.com                              |                     |                     |       |                                          |
+-------------+------------------------------------------+-------------------------+--------------+--------------------------------------------------------------------------------------------------------------------+------------------------------------------+---------------------+---------------------+-------+------------------------------------------+
| 23163376071 | example.com                              | 2025-12-16T20:59:37.431 |       204406 | C=GB, O=Sectigo Limited, CN=Sectigo Public Server Authentication CA DV R36                                         | *.example.com                            | 2026-03-16T16:12:36 | 2025-12-16T00:00:00 |     3 | 7492bfdffaa42846b8a14370d3d8b3f5         |
|             |                                          |                         |              |                                                                                                                    | example.com                              |                     |                     |       |                                          |
+-------------+------------------------------------------+-------------------------+--------------+--------------------------------------------------------------------------------------------------------------------+------------------------------------------+---------------------+---------------------+-------+------------------------------------------+```
```

### 詳細情報とタイミング表示

```bash
rs-crtsh --hostname example.com --verbose --timing
```

**出力例:**

```
> GET https://crt.sh/?q=example.com&output=json
> user-agent: rs-crtsh/1.0

< HTTP/2.0 200 OK
< server: nginx
< date: Tue, 23 Dec 2025 03:16:26 GMT
< content-type: application/json
< access-control-allow-origin: *
< strict-transport-security: max-age=15768000; includeSubDomains; preload
< expect-ct: preload

--- Timing Information ---
Response received: 1.519082042s
Body read time: 57.583µs
Total time: 1.519187834s
Response size: 17952 bytes (17.53 KB)
Throughput: 11.54 KB/s

+-------------+------------------------------------------+-------------------------+--------------+--------------------------------------------------------------------------------------------------------------------+------------------------------------------+---------------------+---------------------+-------+------------------------------------------+
|  crt.sh ID  |           Matching Identities            |        Logged At        | Issuer CA ID |                                                    Issuer Name                                                     |                Name Value                |     Not Before      |      Not After      | Count |              Serial Number               |
+-------------+------------------------------------------+-------------------------+--------------+--------------------------------------------------------------------------------------------------------------------+------------------------------------------+---------------------+---------------------+-------+------------------------------------------+
| 23164227397 | example.com                              | 2025-12-16T21:50:43.225 |       204407 | C=GB, O=Sectigo Limited, CN=Sectigo Public Server Authentication CA DV E36                                         | *.example.com                            | 2026-03-16T20:59:52 | 2025-12-16T00:00:00 |     3 | 4b87ab08fde761c73d3c9f7a6a141bd3         |
|             |                                          |                         |              |                                                                                                                    | example.com                              |                     |                     |       |                                          |
+-------------+------------------------------------------+-------------------------+--------------+--------------------------------------------------------------------------------------------------------------------+------------------------------------------+---------------------+---------------------+-------+------------------------------------------+
| 23164227256 | example.com                              | 2025-12-16T21:50:41.574 |       204407 | C=GB, O=Sectigo Limited, CN=Sectigo Public Server Authentication CA DV E36                                         | *.example.com                            | 2026-03-16T20:59:52 | 2025-12-16T00:00:00 |     3 | 4b87ab08fde761c73d3c9f7a6a141bd3         |
|             |                                          |                         |              |                                                                                                                    | example.com                              |                     |                     |       |                                          |
+-------------+------------------------------------------+-------------------------+--------------+--------------------------------------------------------------------------------------------------------------------+------------------------------------------+---------------------+---------------------+-------+------------------------------------------+
| 23163376071 | example.com                              | 2025-12-16T20:59:37.431 |       204406 | C=GB, O=Sectigo Limited, CN=Sectigo Public Server Authentication CA DV R36                                         | *.example.com                            | 2026-03-16T16:12:36 | 2025-12-16T00:00:00 |     3 | 7492bfdffaa42846b8a14370d3d8b3f5         |
|             |                                          |                         |              |                                                                                                                    | example.com                              |                     |                     |       |                                          |
+-------------+------------------------------------------+-------------------------+--------------+--------------------------------------------------------------------------------------------------------------------+------------------------------------------+---------------------+---------------------+-------+------------------------------------------+```
```

### リトライ設定を使用

```bash
# ネットワークエラー時に3回リトライ、各リトライの間隔は2秒
rs-crtsh --hostname example.com --retry 3 --retry-delay 2
```

### タイムアウト時間を変更

```bash
# タイムアウトを60秒に設定
rs-crtsh --hostname example.com --timeout 60
```

### 生の JSON データを出力

```bash
# テーブル形式ではなく、crt.sh API から返された生の JSON データを出力
rs-crtsh --hostname example.com --format raw
```

**出力例:**

```json
[
  {
    "issuer_ca_id": 204407,
    "issuer_name": "C=GB, O=Sectigo Limited, CN=Sectigo Public Server Authentication CA DV E36",
    "common_name": "example.com",
    "name_value": "*.example.com\nexample.com",
    "id": 23164227397,
    "entry_timestamp": "2025-12-16T21:50:43.225",
    "not_before": "2025-12-16T00:00:00",
    "not_after": "2026-03-16T20:59:52",
    "serial_number": "4b87ab08fde761c73d3c9f7a6a141bd3"
  }
]
```

### CSV 形式で出力

```bash
# CSV 形式で証明書データを出力
rs-crtsh --hostname example.com --format csv
```

**出力例:**

```csv
id,issuer_ca_id,issuer_name,common_name,name_value,entry_timestamp,not_before,not_after,serial_number
23164227397,204407,"C=GB, O=Sectigo Limited, CN=Sectigo Public Server Authentication CA DV E36",example.com,"*.example.com
example.com",2025-12-16T21:50:43.225,2025-12-16T00:00:00,2026-03-16T20:59:52,4b87ab08fde761c73d3c9f7a6a141bd3
23164227256,204407,"C=GB, O=Sectigo Limited, CN=Sectigo Public Server Authentication CA DV E36",example.com,"*.example.com
example.com",2025-12-16T21:50:41.574,2025-12-16T00:00:00,2026-03-16T20:59:52,4b87ab08fde761c73d3c9f7a6a141bd3
23163376071,204406,"C=GB, O=Sectigo Limited, CN=Sectigo Public Server Authentication CA DV R36",example.com,"*.example.com
example.com",2025-12-16T20:59:37.431,2025-12-16T00:00:00,2026-03-16T16:12:36,7492bfdffaa42846b8a14370d3d8b3f5
```

### 特定のカラムのみを表示

```bash
# 証明書の ID、ドメイン名、有効期限のみを表示
rs-crtsh --hostname example.com --column_name id --column_name name_value --column_name not_before --column_name not_after
```

**出力例:**

```
+-------------+------------------------------------------+---------------------+---------------------+
|  crt.sh ID  |                Name Value                |     Not Before      |      Not After      |
+-------------+------------------------------------------+---------------------+---------------------+
| 23164227397 | *.example.com                            | 2025-12-16T00:00:00 | 2026-03-16T20:59:52 |
|             | example.com                              |                     |                     |
+-------------+------------------------------------------+---------------------+---------------------+
| 23164227256 | *.example.com                            | 2025-12-16T00:00:00 | 2026-03-16T20:59:52 |
|             | example.com                              |                     |                     |
+-------------+------------------------------------------+---------------------+---------------------+
```

## 設定ファイルの使用

設定ファイルを使用すると、よく使う設定をプリセットとして保存できます。

### 設定ファイルの例（config.toml）

```toml
[presets.default]
timeout = 60
timing = true
verbose = false
retry = 3
retry_delay = 2.0
format = "table"

[presets.quick]
timeout = 10
timing = false
verbose = false
retry = 0
format = "table"

[presets.debug]
timeout = 120
timing = true
verbose = true
retry = 5
retry_delay = 3.0
format = "table"

[presets.raw]
timeout = 30
timing = false
verbose = false
retry = 0
format = "raw"

[presets.csv]
timeout = 30
timing = false
verbose = false
retry = 0
format = "csv"
```

### 設定ファイルを使用して実行

```bash
# デフォルトプリセットを使用
rs-crtsh --config config.toml --preset default --hostname example.com

# quick プリセットを使用
rs-crtsh --config config.toml --preset quick --hostname example.com

# debug プリセットを使用
rs-crtsh --config config.toml --preset debug --hostname example.com
```

## 証明書情報について

このツールは、以下の証明書情報を表示します:

| 表示名                 | カラム名              | 説明                                             |
|---------------------|-------------------|------------------------------------------------|
| crt.sh ID           | `id`              | crt.sh データベース内の証明書 ID                          |
| Matching Identities | `common_name`     | 証明書の Common Name（CN）                           |
| Logged At           | `entry_timestamp` | 証明書が CT ログに記録された日時                             |
| Issuer CA ID        | `issuer_ca_id`    | 発行者の認証局 ID                                     |
| Issuer Name         | `issuer_name`     | 証明書を発行した認証局の名前                                 |
| Name Value          | `name_value`      | 証明書に含まれる Subject Alternative Name（SAN）やその他の識別名 |
| Not Before          | `not_before`      | 証明書の有効期間開始日                                    |
| Not After           | `not_after`       | 証明書の有効期間終了日                                    |
| Count               | `result_count`    | マッチした証明書の数                                     |
| Serial Number       | `serial_number`   | 証明書のシリアル番号                                     |

`--column_name` オプションで「カラム名」を指定することで、表示する情報を選択できます。

## 開発

### 開発ビルド

```bash
cargo build
```

### テスト実行

```bash
cargo test
```

### フォーマット

```bash
cargo fmt
```

### Lint

```bash
cargo clippy
```

## ライセンス

このプロジェクトは MIT ライセンスの下で公開されています。

## 貢献

バグ報告や機能リクエストは、GitHub の Issues でお気軽にお知らせください。
プルリクエストも歓迎します！

## 参考リンク

- [crt.sh](https://crt.sh/) - Certificate Transparency Log Search
- [Certificate Transparency](https://certificate.transparency.dev/) - 証明書透明性について
