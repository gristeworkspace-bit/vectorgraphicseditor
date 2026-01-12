# Spec: 02_b_transform (Phase 2 後半: 変形アクション)

## 目的
選択したオブジェクトを実際に「移動」「リサイズ」「回転」できるようにする。
**「行列（Matrix）の更新」** が核心となる。

## 実装ステップ

### 1. Rust Core: Transform Logic
- [ ] **Matrix Operations:**
    - `translate(dx, dy)`: 現在の行列に平行移動を適用。
    - `scale(sx, sy, pivot_x, pivot_y)`: 指定した中心点（ピボット）で拡大縮小。
    - `rotate(angle, pivot_x, pivot_y)`: 指定した中心点で回転。
    - **重要:** すべての変形は「現在の行列 × 新しい変形行列」の形で行うこと。

### 2. Rust Core: Commands
- [ ] **State Machine:**
    - `DragState` を導入 (None, Moving, Resizing, Rotating)。
    - ドラッグ開始時(Down)に「元の行列」を保存し、ドラッグ中(Move)は「元の行列 + 差分」で計算する（累積誤差を防ぐため）。

### 3. UI (React): Interaction
- [ ] **Move (移動):**
    - 選択枠の中をドラッグしたら `move_object` コマンドを発行。
- [ ] **Resize (リサイズ):**
    - 四隅のハンドル (Handle) をドラッグしたら `resize_object` コマンドを発行。
    - 対角線のハンドルを動かすロジックを実装。

## 技術的制約
- **リアルタイム性:** ドラッグ中は毎フレーム (60fps) 行列更新と再描画を行う。
- **データフロー:** Reactで座標計算せず、必ず `delta` (移動量) をRustに渡して、Rustが行列を計算し直すこと。

## 完了条件
- 45度回転している四角形を、さらにドラッグして移動したり、ハンドルで大きくしたりできること。
