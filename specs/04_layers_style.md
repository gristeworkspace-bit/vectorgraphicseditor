# Spec: 04_layers_style (Color & Structure)

## 目的
図形の「見た目（スタイル）」を編集可能にし、重なり順（レイヤー）を管理できるようにする。

## 実装ステップ

### 1. Rust Core: Style System
- [x] **Style Struct:** `StyleAttributes` を拡張し、プロパティを編集可能にする。
    ```rust
    #[derive(Clone, Serialize, Deserialize)]
    pub struct StyleAttributes {
        pub fill_color: String,   // Hex "#FF0000" or "none"
        pub stroke_color: String, // Hex "#000000" or "none"
        pub stroke_width: f64,
    }
    ```
- [x] **Commands:** `update_style(id, style)` コマンドを実装する。

### 2. Frontend: Properties Panel
- [x] **UI:** 画面右側（または左側の空きスペース）に「プロパティパネル」を作成。
    - 何も選択していない時は「No Selection」と表示。
    - 選択中は、そのオブジェクトの色情報を表示。
- [x] **Color Picker:**
    - HTML標準の `<input type="color">` を使用。
    - 変更イベントをリアルタイムにRustへ送信し、Canvasを再描画する。

### 3. Rust Core: Layer Order (Z-Index)
- [x] **Reorder Commands:**
    - `bring_to_front(id)`: 配列の末尾へ移動（最前面）。
    - `send_to_back(id)`: 配列の先頭へ移動（最背面）。

## 完了条件
- [x] ハート形を「赤色」や「ピンク色」に変更できること。
- [x] 枠線の太さを変えられること。
- [x] 複数の図形があるとき、重なり順を変更できること。

## ✅ 完了日: 2026-01-12
