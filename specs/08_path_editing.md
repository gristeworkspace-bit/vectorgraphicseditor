# Spec: 08_path_editing (Direct Selection / Node Tool)

## 目的
作成済みのパス（ベジェ曲線）の形状を、後から頂点単位で編集できるようにする。
Illustratorの「ダイレクト選択ツール（白い矢印）」に相当する機能を実装する。

## 実装ステップ

### 1. Rust Core: Path Mutation Logic
- [ ] **Command:** `get_path_points(id) -> Vec<PathPoint>`
    - 指定した図形の頂点情報（座標、ハンドル位置、コマンドタイプ）を取得する。
- [ ] **Command:** `update_path_point(id, index, position, handle_in, handle_out)`
    - 指定したインデックスの頂点座標やハンドルを更新し、形状を再計算する。
    - 隣接するセグメントへの影響（スムーズな接続など）も考慮が必要だが、まずは「座標の直接更新」を実装する。

### 2. Frontend: Interaction (Edit Mode)
- [ ] **Tool:** ツールバーに「Direct Select (White Arrow)」を追加。
- [ ] **Overlay UI:**
    - このツールでパスをクリックした際、通常の「バウンディングボックス（青い枠）」ではなく、**「頂点（■）」と「ハンドル（●ー○）」** をCanvas上に描画する。
    - ハンドルは、選択された頂点に関連するものだけ表示すると見やすい。
- [ ] **Drag Logic:**
    - 頂点（■）をドラッグ -> `update_path_point` で位置（と付随するハンドル）を移動。
    - ハンドル（○）をドラッグ -> `update_path_point` で曲率を変更。

## 完了条件
- 一度描いたハート型や波線を、後から「形を変える」ことができること。
- 頂点を引っ張って形を崩したり、ハンドルを操作してカーブを滑らかに直せること。
