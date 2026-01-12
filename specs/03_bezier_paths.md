# Spec: 03_bezier_paths (The Path Engine)

## 目的
ベジェ曲線（Cubic Bézier）を含む任意のパスデータ構造の実装と、それを描画・編集するための「ペンツール」の作成。

## 実装ステップ

### 1. Rust Core: Path Data Structure
- [ ] **Path Command:** SVG仕様に準拠した列挙型を定義する。
    ```rust
    pub enum PathCommand {
        MoveTo { x: f64, y: f64 },
        LineTo { x: f64, y: f64 },
        CubicBezierTo { cp1_x: f64, cp1_y: f64, cp2_x: f64, cp2_y: f64, to_x: f64, to_y: f64 },
        ClosePath,
    }
    ```
- [ ] **VectorObject拡張:** `ShapeType` に `Path { commands: Vec<PathCommand> }` を追加。

### 2. Rust Core: Rendering Logic
- [ ] **Render Loop:** `PathCommand` をイテレートし、Web Canvas APIの対応するメソッド (`moveTo`, `lineTo`, `bezierCurveTo`) を呼び出すロジックを実装。

### 3. Interaction: The Pen Tool (State Machine)
- [ ] **Tool Mode:** `EditorState` に `active_tool` (Select | Pen) を追加。
- [ ] **Pen States:**
    - `Idle`: パス開始待ち。
    - `Drawing`: パス作成中。
    - `Dragging`: ハンドルを引き出して曲線を制御中。
- [ ] **UX Logic:**
    - **Click:** アンカーポイントを追加（直線）。
    - **Drag:** 直前のアンカーから制御点（Handle）を引き出す（曲線）。
    - **Close:** 始点をクリックするとパスを閉じて完了する。

## 完了条件
- ペンツールでキャンバスをクリック・ドラッグして、滑らかな曲線（ハート形など）が描けること。
- 描いた曲線が、Phase 2で作った機能で「移動・回転」できること（既存機能との統合）。
