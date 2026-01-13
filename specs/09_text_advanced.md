# Spec: 09_text_advanced (Typography: Letter Spacing)

## 目的
テキストオブジェクトの「文字間隔（Tracking / Letter Spacing）」を調整可能にする。
まずはテキスト全体の均一な間隔調整を実装する。

## 実装ステップ

### 1. Rust Core: Text Attributes
- [ ] **Struct Update:** `ShapeType::Text` 構造体にフィールドを追加。
    - `letter_spacing`: f64 (デフォルト 0.0)
    - ※既存のセーブデータ読み込み互換性 (`#[serde(default)]`) を維持すること。
- [ ] **SVG Export:**
    - `<text>` 要素に `letter-spacing` 属性を出力する。
    - 例: `<text x=".." y=".." letter-spacing="5">Content</text>`
- [ ] **Bounding Box Calculation:**
    - テキストの幅計算ロジックを更新し、`letter_spacing` を加味する。
    - 計算式目安: `(文字ごとの幅の合計) + ((文字数 - 1) * letter_spacing)`

### 2. Frontend: Property Panel
- [ ] **UI:** テキスト選択時のプロパティパネルに以下を追加。
    - ラベル: "Letter Spacing" (または "Tracking")
    - コントロール: 数値入力 (`input type="number"`) または スライダー
    - 単位: ピクセル (px)
- [ ] **Interaction:**
    - 値を変更するとリアルタイムでキャンバス上の文字間隔が広がる/狭まること。
    - バウンディングボックス（青枠）もそれに合わせてサイズが変わること。

## 将来の機能メモ (Future Roadmap)
- **個別文字調整 (Kerning / Baseline Shift):**
    - ユーザー要望: アウトライン化せずに、文字ごとの高さ(dy)や位置(dx)を調整したい。
    - 実装方針案: SVGの `<tspan>` 要素を活用し、文字ごとのスタイル情報を保持するデータ構造へ拡張する（今回は対象外）。
