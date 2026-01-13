# cc-dashboard

Claude Code 用のカスタムステータスライン。リアルタイムで使用状況を表示。

## 表示内容

```
 Opus 4.5  v2.1.6 |  ⎇ main  (+42,-10)
 💰 $1.23  (2h45m) |  🧠 88%  |  ⏱ 54%/47%
```

### Line 1
| 項目 | 説明 |
|------|------|
| **モデル名** | 現在使用中のモデル（Opus 4.5 など） |
| **バージョン** | Claude Code のバージョン |
| **ブランチ** | jj/git のブックマーク・ブランチ名 |
| **変更量** | 追加・削除行数 |

### Line 2
| 項目 | 説明 |
|------|------|
| **💰 今日のコスト** | JSONL から計算した本日の使用料金 |
| **(残り時間)** | 5時間ブロックのリセットまでの時間 |
| **🧠 コンテキスト** | コンテキストウィンドウの残り割合 |
| **⏱ 使用率** | 5時間/7日の使用率（API から取得） |

## 機能

- 🎨 **ANSI カラー** - 各項目にバックグラウンドカラー表示
- 📊 **コスト計算** - JSONL から今日の使用料金を自動計算
- ⏰ **使用制限表示** - API から 5時間/7日の使用率を取得（60秒キャッシュ）
- 🧠 **コンテキスト** - 残り割合を色分け（緑 >50%, 黄 20-50%, 赤 <20%）
- 🔄 **jj 対応** - Jujutsu のブックマーク・変更量を表示

## インストール

```bash
# 1. スクリプトをコピー
cp cc-dashboard.sh ~/.claude/scripts/

# 2. 実行権限を付与
chmod +x ~/.claude/scripts/cc-dashboard.sh

# 3. Claude Code 設定を更新
# ~/.claude/settings.json に追加:
```

```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/scripts/cc-dashboard.sh"
  }
}
```

## 設定

### 価格設定（デフォルト: Opus 4.5）

スクリプト内で変更可能：

| トークン種別 | 価格 / 1M tokens |
|-------------|------------------|
| Input | $15 |
| Output | $75 |
| Cache Write | $18.75 |
| Cache Read | $1.50 |

### キャッシュ設定

```bash
CACHE_FILE="/tmp/cc-dashboard-usage-cache.json"
CACHE_TTL=60  # seconds
```

## 依存関係

- `jq` - JSON パーサー
- `curl` - API リクエスト
- `security` (macOS) - Keychain アクセス
- `jj` または `git` - バージョン管理

## 使用制限 API について

このスクリプトは Anthropic の undocumented API を使用して使用制限情報を取得します：

```
GET https://api.anthropic.com/api/oauth/usage
Authorization: Bearer <oauth_token>
anthropic-beta: oauth-2025-04-20
```

認証情報は macOS Keychain の `Claude Code-credentials` から取得します。

## TODO

- [ ] モデル別価格の自動切り替え
- [ ] 設定ファイル対応（JSON/YAML）
- [ ] カラーテーマ
- [ ] Linux 対応（Keychain 代替）

## ライセンス

MIT
