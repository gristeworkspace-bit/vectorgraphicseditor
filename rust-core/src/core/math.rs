//! Math module - Matrix operations for 2D affine transformations
//!
//! Matrix format (row-major):
//! | a  b  tx |
//! | c  d  ty |
//! | 0  0  1  |

use serde::{Deserialize, Serialize};

/// 2D Affine Transformation Matrix
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct TransformMatrix {
    pub a: f64,  // scale x
    pub b: f64,  // skew y
    pub c: f64,  // skew x
    pub d: f64,  // scale y
    pub tx: f64, // translate x
    pub ty: f64, // translate y
}

impl TransformMatrix {
    /// Create an identity matrix
    pub fn identity() -> Self {
        TransformMatrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }

    /// Create a translation matrix
    pub fn translate(tx: f64, ty: f64) -> Self {
        TransformMatrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx,
            ty,
        }
    }

    /// Create a scale matrix
    pub fn scale(sx: f64, sy: f64) -> Self {
        TransformMatrix {
            a: sx,
            b: 0.0,
            c: 0.0,
            d: sy,
            tx: 0.0,
            ty: 0.0,
        }
    }

    /// Create a rotation matrix (angle in radians)
    pub fn rotate(angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        TransformMatrix {
            a: cos,
            b: sin,
            c: -sin,
            d: cos,
            tx: 0.0,
            ty: 0.0,
        }
    }

    /// Multiply two matrices: self * other
    pub fn multiply(&self, other: &TransformMatrix) -> TransformMatrix {
        TransformMatrix {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
            tx: self.a * other.tx + self.b * other.ty + self.tx,
            ty: self.c * other.tx + self.d * other.ty + self.ty,
        }
    }

    /// Calculate the inverse matrix
    /// Returns None if the matrix is not invertible (determinant is zero)
    pub fn inverse(&self) -> Option<TransformMatrix> {
        let det = self.a * self.d - self.b * self.c;
        if det.abs() < 1e-10 {
            return None;
        }
        let inv_det = 1.0 / det;
        Some(TransformMatrix {
            a: self.d * inv_det,
            b: -self.b * inv_det,
            c: -self.c * inv_det,
            d: self.a * inv_det,
            tx: (self.b * self.ty - self.d * self.tx) * inv_det,
            ty: (self.c * self.tx - self.a * self.ty) * inv_det,
        })
    }

    /// Transform a point (x, y) using this matrix
    pub fn transform_point(&self, x: f64, y: f64) -> (f64, f64) {
        (
            self.a * x + self.b * y + self.tx,
            self.c * x + self.d * y + self.ty,
        )
    }

    /// Get the determinant of the matrix
    pub fn determinant(&self) -> f64 {
        self.a * self.d - self.b * self.c
    }

    /// Scale around a pivot point
    /// Formula: Translate(pivot) × Scale × Translate(-pivot)
    pub fn scale_around(sx: f64, sy: f64, pivot_x: f64, pivot_y: f64) -> Self {
        // Move pivot to origin, scale, move back
        let to_origin = TransformMatrix::translate(-pivot_x, -pivot_y);
        let scale = TransformMatrix::scale(sx, sy);
        let from_origin = TransformMatrix::translate(pivot_x, pivot_y);
        from_origin.multiply(&scale.multiply(&to_origin))
    }

    /// Rotate around a pivot point (angle in radians)
    /// Formula: Translate(pivot) × Rotate × Translate(-pivot)
    pub fn rotate_around(angle: f64, pivot_x: f64, pivot_y: f64) -> Self {
        // Move pivot to origin, rotate, move back
        let to_origin = TransformMatrix::translate(-pivot_x, -pivot_y);
        let rotation = TransformMatrix::rotate(angle);
        let from_origin = TransformMatrix::translate(pivot_x, pivot_y);
        from_origin.multiply(&rotation.multiply(&to_origin))
    }

    /// Get translation components
    pub fn translation(&self) -> (f64, f64) {
        (self.tx, self.ty)
    }
}

impl Default for TransformMatrix {
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let m = TransformMatrix::identity();
        let (x, y) = m.transform_point(10.0, 20.0);
        assert!((x - 10.0).abs() < 1e-10);
        assert!((y - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_translate() {
        let m = TransformMatrix::translate(5.0, 10.0);
        let (x, y) = m.transform_point(0.0, 0.0);
        assert!((x - 5.0).abs() < 1e-10);
        assert!((y - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_scale() {
        let m = TransformMatrix::scale(2.0, 3.0);
        let (x, y) = m.transform_point(10.0, 10.0);
        assert!((x - 20.0).abs() < 1e-10);
        assert!((y - 30.0).abs() < 1e-10);
    }

    #[test]
    fn test_inverse() {
        let m = TransformMatrix::translate(5.0, 10.0)
            .multiply(&TransformMatrix::scale(2.0, 2.0));
        let inv = m.inverse().unwrap();
        let result = m.multiply(&inv);
        let identity = TransformMatrix::identity();
        assert!((result.a - identity.a).abs() < 1e-10);
        assert!((result.d - identity.d).abs() < 1e-10);
        assert!((result.tx - identity.tx).abs() < 1e-10);
        assert!((result.ty - identity.ty).abs() < 1e-10);
    }

    #[test]
    fn test_rotation() {
        use std::f64::consts::PI;
        let m = TransformMatrix::rotate(PI / 2.0); // 90 degrees
        let (x, y) = m.transform_point(1.0, 0.0);
        assert!(x.abs() < 1e-10); // should be ~0
        assert!((y + 1.0).abs() < 1e-10); // clockwise rotation: (1,0) -> (0,-1)
    }

    #[test]
    fn test_scale_around_pivot() {
        // Scale 2x around point (100, 100)
        // Point (150, 100) should become (200, 100) - 50 units away becomes 100 units
        let m = TransformMatrix::scale_around(2.0, 2.0, 100.0, 100.0);
        let (x, y) = m.transform_point(150.0, 100.0);
        assert!((x - 200.0).abs() < 1e-10);
        assert!((y - 100.0).abs() < 1e-10);
        
        // Pivot point itself should not move
        let (px, py) = m.transform_point(100.0, 100.0);
        assert!((px - 100.0).abs() < 1e-10);
        assert!((py - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_rotate_around_pivot() {
        use std::f64::consts::PI;
        // Rotate 90 degrees around point (100, 100)
        // Point (150, 100) should become (100, 50) - rotated CCW
        let m = TransformMatrix::rotate_around(PI / 2.0, 100.0, 100.0);
        let (x, y) = m.transform_point(150.0, 100.0);
        assert!((x - 100.0).abs() < 1e-10);
        // Note: our rotation is clockwise, so (150,100) relative to (100,100) = (50,0) rotates to (0,-50)
        // In world coords: (100, 100) + (0, -50) = (100, 50)
        assert!((y - 50.0).abs() < 1e-10);
        
        // Pivot point itself should not move
        let (px, py) = m.transform_point(100.0, 100.0);
        assert!((px - 100.0).abs() < 1e-10);
        assert!((py - 100.0).abs() < 1e-10);
    }
}
