# Spec: 06_undo_redo (History System)

## 目的
ユーザーの操作履歴を記録し、「元に戻す (Undo)」と「やり直し (Redo)」を行えるようにする。

## 実装ステップ

### 1. Rust Core: History Stack
- [x] **Struct:** `EditorState` (または `Scene`) に、履歴管理用のフィールドを追加。
    ```rust
    pub struct History {
        undo_stack: Vec<Scene>, // 過去のシーンのスナップショット
        redo_stack: Vec<Scene>, // 取り消したシーン（やり直し用）
        max_history: usize,     // 履歴の上限（例: 50回）
    }
    ```
- [x] **Snapshot Logic:**
    - 図形を追加・変更・削除する「破壊的操作」の直前に、現在のシーンを `undo_stack` にプッシュする処理を実装。
    - ※選択変更だけの操作は履歴に残さないのが一般的。

### 2. Rust Core: Commands
- [x] **Undo:**
    - `undo_stack` から最新の状態をポップし、現在の `Scene` を `redo_stack` に退避させてから、シーンを上書きする。
- [x] **Redo:**
    - `redo_stack` から状態をポップし、現在の `Scene` を `undo_stack` に退避させてから、シーンを上書きする。

### 3. Frontend: UI & Shortcuts
- [x] **UI:** ツールバーに「↩️ Undo」「↪️ Redo」ボタンを追加。
    - 履歴がない場合はボタンを無効化（グレーアウト）する。
- [x] **Hotkeys:**
    - `Ctrl + Z` (Mac: `Cmd + Z`) -> Undo
    - `Ctrl + Y` (or `Cmd + Shift + Z`) -> Redo
    - キーボードイベントをリッスンし、対応するRustコマンドを呼び出す。

## 完了条件
- [x] 図形を動かした後、「Undo」ボタンで元の位置に戻ること。
- [x] 「Undo」した操作を「Redo」で再度実行できること。
- [x] キーボードショートカットでサクサク操作できること。

## ✅ 完了日: 2026-01-12
