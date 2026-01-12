# Spec: 05_persistence (Save & Load)

## 目的
現在のキャンバスの状態（Scene）をJSONファイルとしてダウンロード保存し、またファイルをアップロードして状態を復元できるようにする。

## 実装ステップ

### 1. Rust Core: Serialization
- [x] **Serialize:** `Scene` 構造体（およびShape, Style等）が `Serialize`, `Deserialize` を実装していることを確認。
- [x] **Commands:**
    - `export_scene_to_json() -> String`: 現在のシーン全体をJSON文字列として返す。
    - `import_scene_from_json(json: String)`: JSON文字列を受け取り、`Scene` を上書きする。

### 2. Frontend: File I/O
- [x] **Header UI:** 画面上部（ヘッダー）に以下のボタンを追加。
    - 💾 **Save (.json):** クリックすると `drawing.json` としてダウンロードされる。
    - 📂 **Load:** ファイル選択ダイアログが開き、JSONファイルを選ぶとキャンバスに反映される。

### 3. (Optional) SVG Export
- [ ] **Export SVG:** 将来的にはSVG出力も目指すが、まずは独自形式（JSON）の保存・読み込みを最優先とする。

## 完了条件
- [x] 描いた絵を「Save」ボタンでPCに保存できること。
- [x] ブラウザをリロードして真っ白にした後、「Load」で保存したファイルを読み込み、元の状態（色や重なり順含む）が完全に復元されること。

## ✅ 完了日: 2026-01-12
