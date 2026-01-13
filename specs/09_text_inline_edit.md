# Spec: 09_text_inline_edit (In-Place Text Editing)

## 目的
サイドバーではなく、キャンバス上のテキスト図形を直接ダブルクリックして編集できるようにする。
いわゆる「インライン編集」を実現する。

## 実装戦略: The Overlay Approach
SVGの `<text>` タグは直接編集できないため、HTMLの `<textarea>` をオーバーレイ表示する手法をとる。

### 1. React Frontend
- [ ] **Interaction:**
    - テキスト図形を「ダブルクリック」した時、編集モードに入る。
- [ ] **Overlay Component:**
    - 編集モード中、対象のテキスト図形と **全く同じ位置、フォントサイズ、フォント、色** を持つ `<textarea>` を、Canvasの上に絶対配置(`position: absolute`)で表示する。
    - 元のSVGテキストは、編集中は非表示 (`opacity: 0`) にする（二重に見えないように）。
- [ ] **Behavior:**
    - `<textarea>` にフォーカスを当て、カーソルを表示する。
    - 背景は透明 (`transparent`) にし、まるでキャンバスに直接書いているように見せる。
    - 日本語入力（IME）が正常に動作すること。
- [ ] **Commit:**
    - フォーカスが外れる (`onBlur`) か、特定のキー操作（`Ctrl+Enter` 推奨、`Enter`は改行に使われる可能性があるため）で編集を確定。
    - 変更された内容を Rust の `update_text_content` コマンドで送信する。
    - `<textarea>` を消去し、更新されたSVGテキストを再表示する。

## 注意点
- **Zoom/Panの同期:** ズームしている場合、`<textarea>` の `font-size` や `transform` もそれに合わせて計算する必要がある（まずはズーム100%での動作を優先）。
