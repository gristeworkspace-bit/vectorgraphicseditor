# Spec: 03_b_pen_tool (The Pen Tool Interaction)

## 目的
Adobe Illustratorのような「ペンツール」を実装し、クリックで直線、ドラッグで曲線を自由に描けるようにする。

## 実装ステップ

### 1. Rust Core: Tool State Machine
- [ ] **Pen Tool State:** `EditorState` にペンツール専用の状態変数を追加する。
    ```rust
    enum PenState {
        Idle,
        Drawing { 
            current_path: Vec<PathCommand>, 
            active_handle: Option<(f64, f64)> // ドラッグ中の制御点
        }
    }
    ```
- [ ] **Preview Logic:** 「現在マウスがある場所」へ伸びるラバーバンド（プレビュー線）の計算ロジック。

### 2. Interaction Logic (The "Behaviors")
- [ ] **Click (Mouse Down & Up quickly):**
    - アンカーポイントを追加し、**直線 (LineTo)** を引く。
- [ ] **Drag (Mouse Down & Move):**
    - 直前のアンカーポイントから制御ハンドルを引き出し、**曲線 (CubicBezierTo)** に変換する。
    - マウスを動かすたびに、曲線の形状をリアルタイム更新する。
- [ ] **Close Path:**
    - 始点（最初の点）をクリックした場合、パスを閉じて (`ClosePath`)、編集を終了する。

### 3. Frontend: Visualization
- [ ] **Preview Rendering:**
    - 確定したパス（黒線）だけでなく、**「今引いている最中の線（青いガイド線）」** と **「制御ハンドル（ヒゲ）」** を描画する。

## 完了条件
- キャンバス上でカチカチとクリックして多角形が描けること。
- グイッとドラッグして、滑らかな曲線（波線など）が描けること。
- 始点をクリックして図形を閉じられること。
