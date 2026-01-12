# ROADMAP_ULTIMATE.md

## 👑 プロジェクト究極目標
「機能を追加しても壊れない」堅牢なアーキテクチャを持つ、商用レベルのブラウザ版Vector Graphics Editor。

## 🛡️ アーキテクチャ原則 (The Extensibility Core)

このプロジェクトは、単なる機能の積み上げではなく、**「Core (カーネル)」と「Plugins (拡張)」の分離**を徹底する。

1.  **Micro-kernel Pattern:**
    - Rustのコアエンジンは「図形の管理」と「レンダリング」のみに集中する。
    - ツール（ペン、選択）、操作（移動、変形）、エフェクトはすべて「Command」として外付けで実装する。
2.  **Event-Driven:**
    - コンポーネント間の直接参照を禁止し、イベントバスを経由して通信する。
3.  **Spatial Partitioning:**
    - オブジェクトが1万個になっても重くならないよう、最初から空間分割アルゴリズムを導入する。

---

## 🗺️ 開発フェーズ (The Timeline)

### Phase 1: The Foundation (核となるエンジン)
**目標:** 堅牢なデータ構造とレンダリングパイプラインの確立
- [ ] **Core Architecture:** `SceneGraph` (Composite Pattern) の実装。
- [ ] **Math Engine:** 行列演算 (`AffineTransform`) と `inverse` の実装。
- [ ] **Render Pipeline:** Rust -> Wasm -> Canvas の高速描画ループ。
- [ ] **Input System:** マウス/タッチイベントを正規化し、座標変換してRustへ渡す仕組み。

### Phase 2: The Editor Essentials (編集機能の基本)
**目標:** 「選択」「移動」「変形」の完全な実装
- [ ] **Spatial Indexing (R-Tree / Quadtree):**
    - **【重要】** オブジェクト数が増えてもヒットテストがO(log n)で終わるようにする。これがないと将来必ず詰む。
- [ ] **Selection Engine:**
    - 複数選択、グループ選択、Cmd+Clickによる深い階層の選択。
- [ ] **Transform System:**
    - 選択群の中心(Center of Gravity)を基準とした回転・拡大縮小。
    - `Smart Guide` (スナップ機能) のための吸着ロジック基礎。

### Phase 3: The Path Engine (ベジェ曲線の完全制御)
**目標:** プロ仕様のパス編集
- [ ] **Path Data Structure:** SVG準拠のコマンド (`M`, `L`, `C`, `Z`)。
- [ ] **Pen Tool (State Machine):**
    - クリック(Corner)、ドラッグ(Smooth)、Alt+ドラッグ(Cusp) のステート管理。
- [ ] **Bezier Math:**
    - 曲線の分割(Subdivision)、長さ計算、曲率に応じた適応的レンダリング。

### Phase 4: Advanced Geometry (ブール演算とシェイプ)
**目標:** 「図形の合成」などの高度な計算
- [ ] **Boolean Operations (Rust):**
    - クライナー法または `Weiler-Atherton` アルゴリズムの実装。
    - Union (合体), Subtract (型抜き), Intersect (交差), Xor (中マド)。
- [ ] **Stroke Expansion:**
    - 「パスのアウトライン化」機能（線を太さのある面に変換する計算）。
    - 線の結合形状 (Miter, Round, Bevel) の計算処理。

### Phase 5: Text Typography Engine (最難関)
**目標:** テキストの表示と編集（Webフォントではなく自前レンダリング）
- [ ] **Font Parsing:** `ttf-parser` (Rust) を使用してフォントファイルを解析。
- [ ] **Glyph to Path:** 文字の形状をベジェ曲線として抽出。
- [ ] **Text Layout:**
    - カーニング、行送り、整列の実装。
    - **Text on Path:** パスに沿って文字を配置する機能。

### Phase 6: Performance Optimization (大規模データ対応)
**目標:** 10,000オブジェクトでも60fps維持
- [ ] **Render Caching:**
    - 静止しているオブジェクトをオフスクリーンCanvasにキャッシュ。
    - 変更があった部分（Dirty Rect）のみを再描画。
- [ ] **Web Worker / Multi-threading:**
    - 重い幾何学計算（ブール演算など）をメインスレッドから分離。

### Phase 7: Plugin & Scripting API (無限の拡張性)
**目標:** サードパーティ（または自分）が本体を触らずに機能追加できる仕組み
- [ ] **Command Registry:**
    - すべての機能を `CommandID` で呼び出せるようにする。
- [ ] **Scripting Interface:**
    - JSからRustの内部コマンドを呼び出せるAPIを公開。
    - 例: `editor.execute('create_star', { points: 5, radius: 100 })`

---

## 🏗️ 拡張性に耐えうるディレクトリ構造 (Rust Core)

「後から機能を追加する」ときは、既存ファイルを修正するのではなく、新しいモジュールを追加するだけで済むようにする。

```text
src-tauri/src/
├── core/
│   ├── scene/          # シーングラフ、ノード定義
│   ├── math/           # 行列、ベクター計算
│   └── spatial/        # R-Tree (空間インデックス)
├── geometry/           # 純粋な幾何学アルゴリズム
│   ├── bezier/         # ベジェ曲線計算
│   ├── boolean/        # ブール演算
│   └── stroke/         # アウトライン化処理
├── tools/              # ツールごとのロジック (Plugin的構造)
│   ├── pen.rs
│   ├── select.rs
│   └── shape.rs
├── commands/           # Undo/Redo可能な操作単位
├── renderer/           # 描画処理
└── export/             # SVG/PDF/Image出力
