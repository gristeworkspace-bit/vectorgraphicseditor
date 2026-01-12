# Spec: 07_export_polish (Export & Final Polish)

## 目的
作成した作品を、Webや他のツールで使える標準フォーマット（SVG, PNG）で出力し、アプリを完成させる。

## 実装ステップ

### 1. Rust Core: SVG Generation
- [x] **Logic:** `Scene` データを巡回し、正しいSVGタグ文字列を生成する関数を実装。
    - `<svg xmlns="...">`: ヘッダーとviewBox。
    - `<rect>`, `<path>`: 座標変換とスタイル（fill, stroke）をSVG属性に変換。
    - ※JSONの時と違い、今回は「他人がブラウザで見れる標準規格」に変換する点がポイント。
- [x] **Command:** `export_to_svg() -> String` を実装。

### 2. Frontend: Export UI
- [x] **UI:** ヘッダーに「Export」メニュー（またはアイコン）を追加。
    - 📄 **Export SVG:** Rustから生成された文字列を受け取り、`.svg` ファイルとしてダウンロード。
    - 🖼️ **Export PNG:** HTML Canvas API (`canvas.toBlob` or `toDataURL`) を利用して、現在の見た目をそのまま `.png` 画像としてダウンロード。

## 完了条件
- 描いた絵を「Export SVG」すると、Illustratorやブラウザで開けるSVGファイルが手に入ること。
- 「Export PNG」すると、画像として保存できること。
