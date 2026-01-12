# DATA_SCHEMA.md

## データ構造の正解 (Single Source of Truth)

### 1. 座標変換行列 (Rust)
```rust
// 3x3 Matrix for 2D Affine Transformations
pub struct TransformMatrix {
    pub a: f64, pub b: f64, pub c: f64, pub d: f64,
    pub tx: f64, pub ty: f64,
}
2. シーングラフノード (Composite Pattern)

Rust
pub enum SceneNode {
    Group { id: String, children: Vec<SceneNode>, transform: TransformMatrix },
    Leaf { id: String, shape: VectorObject }
}
