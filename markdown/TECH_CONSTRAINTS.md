
TECH_CONSTRAINTS.md (禁止事項)
DOM操作禁止: Canvas以外のDOMを直接操作しない。

Any禁止: TypeScriptの any は使用禁止。

Rust Panic禁止: unwrap() は使わず Result で返す。

テスト必須: 幾何学計算は必ず単体テストを書く。 EOF

==========================================
4. specs/01_mvp_setup.md (Updated for Web)
==========================================
cat << 'EOF' > specs/01_mvp_setup.md

Spec: 01_mvp_setup (Web/Wasm Edition)
目的
React + Rust (Wasm) の開発環境を構築し、パイプラインを開通させる。 Tauriは使用しない。

実装ステップ
1. Rust Core Setup

ルートに rust-core ディレクトリを作成。

cargo new --lib . で初期化。

Cargo.toml に wasm-bindgen を追加。

crate-type = ["cdylib"] を設定。

2. Frontend Setup

ルートに Vite + React + TypeScript をセットアップ（既存ファイルがあれば上書き）。

vite-plugin-wasm と vite-plugin-top-level-await を導入。

3. Bridge Implementation

Rust側: greet ではなく draw_rect(x, y, w, h) のような描画関数を実装。

React側: useEffect でWasmをロードし、Canvasに描画関数を呼び出す。

完了条件
npm run dev でブラウザが立ち上がり、CanvasにRustから指示された図形が表示される。 EOF

==========================================
5. specs/02_interaction.md (Spatial Index)
==========================================
cat << 'EOF' > specs/02_interaction.md

Spec: 02_interaction
目的
選択ツールと移動機能の実装。 重要: 空間インデックス導入の準備

実装ステップ
Rust: hit_test に逆行列を使用。

Optimization: 将来のために rust-core/src/spatial/ モジュールを作成（まだ単純実装で良いが、構造は作る）。

UI: 選択時のバウンディングボックス表示。 EOF

echo "✅ 仕様書（Webアプリ構成版）を復元しました。"


### 手順 2: Claude Code への返信

ファイルが復元できたら、先ほど作成した文章をClaude Codeに送信してください。これで辻褄が合います。

**以下のテキストをコピーして Claude Code に送信:**

```text
質問への回答と、最初のアクション指示です。

1. 実装状況
エラーが解決不能になったため、既存のコードは全て破棄しました。
完全に「ゼロベース」からの再構築をお願いします。現在のディレクトリは空（設定ファイルのみ）だと考えてください。

2. ターゲットプラットフォームとディレクトリ構造【重要】
「純粋なWebアプリ (React + Rust Wasm)」として作成します。デスクトップアプリ（Tauri）ではありません。
ですので、Rust側のルートディレクトリ名は `rust-core/` としてください。(`src-tauri/` は禁止です)

3. 優先順位
ロードマップの Phase 1 から順を追って実装します。

4. 既存のコードベース
ありません。これからあなたが作成します。
