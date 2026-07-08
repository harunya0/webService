# 開発思想

- 分かりやすさを最優先する。
- 1つのクラスは1つの責務を持つ。
- コメントには「なぜ」を書く。
- 「何をしているか」は命名で表現する。
- 必要になってから抽象化する。
- 依存関係は一方向にする。
- 未来の自分が5分で理解できる構成を目指す。
- 
## API
- GET /tasks
```json
[
  {
    "id": 1,
    "title": "Rust勉強",
    "completed": false
  }
]
```
- POST /api/...

## 今後実装予定
- ログイン
- タスク検索
- タスク並び替え
- 締切日
- タグ

## 設計

# ユーザーライフサイクル

1. トップページへアクセス
2. ユーザー登録（初回のみ）
3. ログイン
4. JWTを受け取る
5. タスク一覧を取得
6. タスクを作成
7. タスクを編集
8. タスクを削除
9. ログアウト

# 状態変移
未ログイン
    │
    ├─ログイン成功
    ▼
ログイン済み
    │
    ├─一覧取得
    ├─作成
    ├─編集
    ├─削除
    │
    └─ログアウト
          ▼
      未ログイン

## DB

# DBの流れ
ログイン
    ↓
users テーブル検索

タスク一覧
    ↓
tasks テーブル検索

タスク作成
    ↓
tasks INSERT

タスク編集
    ↓
tasks UPDATE

タスク削除
    ↓
tasks DELETE

# DBテーブル
users
------
id
username
password_hash
created_at

tasks
------
id
user_id
title
completed
created_at
updated_at

# ディレクトリ構成
backend/
├── src/
│   ├── main.rs
│   ├── routes/
│   ├── handlers/
│   ├── models/
│   ├── db/
│   └── middleware/

frontend/
├── index.html
├── css/
├── js/
└── assets/

# API
POST /register
        ↓
POST /login
        ↓
GET /tasks
        ↓
POST /tasks
        ↓
PUT /tasks/:id
        ↓
DELETE /tasks/:id
        ↓
POST /logout（任意）