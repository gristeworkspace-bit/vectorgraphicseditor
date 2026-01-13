//! Hit testing module
//!
//! Provides precise hit testing for vector objects using inverse transforms.

use crate::core::math::TransformMatrix;
use crate::core::scene::{PathCommand, VectorObject};

/// Check if a point is inside a rectangle (in local coordinates)
pub fn point_in_rect(x: f64, y: f64, rect_x: f64, rect_y: f64, width: f64, height: f64) -> bool {
    x >= rect_x && x <= rect_x + width && y >= rect_y && y <= rect_y + height
}

/// Check if a point is inside an ellipse (in local coordinates)
pub fn point_in_ellipse(x: f64, y: f64, cx: f64, cy: f64, rx: f64, ry: f64) -> bool {
    if rx <= 0.0 || ry <= 0.0 {
        return false;
    }
    let dx = (x - cx) / rx;
    let dy = (y - cy) / ry;
    dx * dx + dy * dy <= 1.0
}

/// Check if a point is inside a path's bounding box (in local coordinates)
/// Uses a simple bounding box approach - calculates min/max from all points in path
pub fn point_in_path_bounds(x: f64, y: f64, commands: &[PathCommand]) -> bool {
    if commands.is_empty() {
        return false;
    }
    
    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;
    
    // Helper to update bounds
    let mut update_bounds = |px: f64, py: f64| {
        min_x = min_x.min(px);
        min_y = min_y.min(py);
        max_x = max_x.max(px);
        max_y = max_y.max(py);
    };
    
    for cmd in commands {
        match cmd {
            PathCommand::MoveTo { x, y } => update_bounds(*x, *y),
            PathCommand::LineTo { x, y } => update_bounds(*x, *y),
            PathCommand::CurveTo { x1, y1, x2, y2, x, y } => {
                // Include all control points and endpoint for conservative bounds
                update_bounds(*x1, *y1);
                update_bounds(*x2, *y2);
                update_bounds(*x, *y);
            }
            PathCommand::ClosePath => {}
        }
    }
    
    // Check if point is within bounds
    x >= min_x && x <= max_x && y >= min_y && y <= max_y
}

/// Test if a world point hits a vector object with the given transform
pub fn hit_test_object(
    world_x: f64,
    world_y: f64,
    object: &VectorObject,
    world_transform: &TransformMatrix,
) -> bool {
    // Get inverse transform to convert world coordinates to local coordinates
    let inverse = match world_transform.inverse() {
        Some(inv) => inv,
        None => return false, // Degenerate transform, can't hit test
    };

    // Transform world point to local coordinates
    let (local_x, local_y) = inverse.transform_point(world_x, world_y);

    // Test against the shape in local coordinates
    match object {
        VectorObject::Rectangle { x, y, width, height } => {
            point_in_rect(local_x, local_y, *x, *y, *width, *height)
        }
        VectorObject::Ellipse { cx, cy, rx, ry } => {
            point_in_ellipse(local_x, local_y, *cx, *cy, *rx, *ry)
        }
        VectorObject::Path { commands, .. } => {
            point_in_path_bounds(local_x, local_y, commands)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_point_in_rect() {
        assert!(point_in_rect(50.0, 50.0, 0.0, 0.0, 100.0, 100.0));
        assert!(point_in_rect(0.0, 0.0, 0.0, 0.0, 100.0, 100.0));
        assert!(point_in_rect(100.0, 100.0, 0.0, 0.0, 100.0, 100.0));
        assert!(!point_in_rect(101.0, 50.0, 0.0, 0.0, 100.0, 100.0));
        assert!(!point_in_rect(-1.0, 50.0, 0.0, 0.0, 100.0, 100.0));
    }

    #[test]
    fn test_point_in_ellipse() {
        assert!(point_in_ellipse(50.0, 50.0, 50.0, 50.0, 30.0, 20.0)); // center
        assert!(point_in_ellipse(80.0, 50.0, 50.0, 50.0, 30.0, 20.0)); // right edge
        assert!(!point_in_ellipse(81.0, 50.0, 50.0, 50.0, 30.0, 20.0)); // outside
    }

    #[test]
    fn test_hit_test_rotated_rect() {
        // Rectangle at origin, 100x50, rotated 45 degrees around origin
        let rect = VectorObject::Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 50.0,
        };
        let transform = TransformMatrix::rotate(PI / 4.0); // 45 degrees

        // Point at local (50, 25) should hit
        // In world coords after 45 deg rotation: approximately (17.7, 53.0)
        let (wx, wy) = transform.transform_point(50.0, 25.0);
        assert!(hit_test_object(wx, wy, &rect, &transform));

        // Point far away should not hit
        assert!(!hit_test_object(1000.0, 1000.0, &rect, &transform));
    }
}
