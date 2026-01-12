# Spec: 02_interaction (Ultimate Edition)

## 目的
オブジェクトの「選択」「移動」「変形」を実装する。
**将来の数万オブジェクト描画に耐えうる「空間インデックス」の準備も行う。**

## 実装ステップ

### 1. Rust Core: Spatial & Math
- [x] **Matrix Math:** `nalgebra` クレートを導入し、`TransformMatrix` に逆行列 (`invert`) メソッドを実装する。
- [x] **Spatial Indexing (準備):** `rust-core/src/spatial/` モジュールを作成。
    - まだR-Treeの完全実装はしなくて良いが、`Query` トレイトを定義し、将来的に差し替え可能な構造にする。
    - 当面は単純なリスト探索で良いが、必ずこのインターフェースを経由させること。

### 2. Rust Core: Hit Testing
- [x] **Inverse Transform:**
    - クリック判定は「マウス座標を逆行列でローカル座標に変換」して行う（必須）。
    - これにより、回転した図形も正しくクリックできるようにする。
- [x] **Selection State:** `EditorState` に `selected_ids: HashSet<String>` を追加。

### 3. Frontend (React): Interaction
- [x] **Event Handling:** Canvasへの PointerEvent (Down, Move, Up) をRustに転送。
- [x] **UI Overlay:** 選択されたオブジェクトの周囲に「青い枠（バウンディングボックス）」を表示する。
    - ハンドル（四隅の■）の実装はPhase 2の後半で行うため、まずは枠だけで良い。

## 完了条件
- [x] 回転させた四角形（データ上で回転定義）をクリックした際、正確に選択できること。
- [x] コンソールに「Selected: [ID]」とログが出る、または画面上で枠が表示されること。

