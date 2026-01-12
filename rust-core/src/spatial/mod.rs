//! Spatial indexing module
//!
//! Provides spatial query interface for future R-Tree implementation.
//! Currently uses simple list traversal.

pub mod simple_index;

use crate::core::math::TransformMatrix;
use crate::core::scene::ObjectId;

/// Bounding box for spatial queries
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl BoundingBox {
    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        BoundingBox { min_x, min_y, max_x, max_y }
    }

    pub fn from_rect(x: f64, y: f64, width: f64, height: f64) -> Self {
        BoundingBox {
            min_x: x,
            min_y: y,
            max_x: x + width,
            max_y: y + height,
        }
    }

    pub fn from_ellipse(cx: f64, cy: f64, rx: f64, ry: f64) -> Self {
        BoundingBox {
            min_x: cx - rx,
            min_y: cy - ry,
            max_x: cx + rx,
            max_y: cy + ry,
        }
    }

    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    pub fn width(&self) -> f64 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> f64 {
        self.max_y - self.min_y
    }

    pub fn center(&self) -> (f64, f64) {
        ((self.min_x + self.max_x) / 2.0, (self.min_y + self.max_y) / 2.0)
    }

    /// Transform bounding box corners and compute new AABB
    pub fn transform(&self, matrix: &TransformMatrix) -> BoundingBox {
        let corners = [
            matrix.transform_point(self.min_x, self.min_y),
            matrix.transform_point(self.max_x, self.min_y),
            matrix.transform_point(self.min_x, self.max_y),
            matrix.transform_point(self.max_x, self.max_y),
        ];

        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for (x, y) in corners {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        BoundingBox { min_x, min_y, max_x, max_y }
    }
}

/// Spatial entry for indexing
#[derive(Debug, Clone)]
pub struct SpatialEntry {
    pub id: ObjectId,
    pub bounds: BoundingBox,
    pub world_transform: TransformMatrix,
}

/// Trait for spatial queries - can be swapped for R-Tree later
pub trait SpatialQuery {
    /// Query objects at a point (returns IDs in reverse Z-order for hit testing)
    fn query_point(&self, x: f64, y: f64) -> Vec<ObjectId>;
    
    /// Query objects within a rectangle
    fn query_rect(&self, bounds: &BoundingBox) -> Vec<ObjectId>;
    
    /// Insert an entry
    fn insert(&mut self, entry: SpatialEntry);
    
    /// Remove an entry by ID
    fn remove(&mut self, id: &ObjectId);
    
    /// Clear all entries
    fn clear(&mut self);
    
    /// Rebuild index (for batch updates)
    fn rebuild(&mut self, entries: Vec<SpatialEntry>);
}
