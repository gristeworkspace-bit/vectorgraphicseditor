//! Simple spatial index using list traversal
//!
//! This is a placeholder implementation. Will be replaced with R-Tree for performance.

use super::{BoundingBox, ObjectId, SpatialEntry, SpatialQuery};

/// Simple list-based spatial index
#[derive(Debug, Default)]
pub struct SimpleIndex {
    entries: Vec<SpatialEntry>,
}

impl SimpleIndex {
    pub fn new() -> Self {
        SimpleIndex { entries: Vec::new() }
    }
}

impl SpatialQuery for SimpleIndex {
    fn query_point(&self, x: f64, y: f64) -> Vec<ObjectId> {
        // Return in reverse order (top-most first for Z-order)
        self.entries
            .iter()
            .rev()
            .filter(|entry| entry.bounds.contains_point(x, y))
            .map(|entry| entry.id.clone())
            .collect()
    }

    fn query_rect(&self, bounds: &BoundingBox) -> Vec<ObjectId> {
        self.entries
            .iter()
            .filter(|entry| {
                // AABB intersection test
                entry.bounds.min_x <= bounds.max_x
                    && entry.bounds.max_x >= bounds.min_x
                    && entry.bounds.min_y <= bounds.max_y
                    && entry.bounds.max_y >= bounds.min_y
            })
            .map(|entry| entry.id.clone())
            .collect()
    }

    fn insert(&mut self, entry: SpatialEntry) {
        self.entries.push(entry);
    }

    fn remove(&mut self, id: &ObjectId) {
        self.entries.retain(|entry| &entry.id != id);
    }

    fn clear(&mut self) {
        self.entries.clear();
    }

    fn rebuild(&mut self, entries: Vec<SpatialEntry>) {
        self.entries = entries;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::math::TransformMatrix;

    #[test]
    fn test_query_point() {
        let mut index = SimpleIndex::new();
        
        index.insert(SpatialEntry {
            id: "obj_1".to_string(),
            bounds: BoundingBox::from_rect(0.0, 0.0, 100.0, 100.0),
            world_transform: TransformMatrix::identity(),
        });
        
        index.insert(SpatialEntry {
            id: "obj_2".to_string(),
            bounds: BoundingBox::from_rect(50.0, 50.0, 100.0, 100.0),
            world_transform: TransformMatrix::identity(),
        });

        // Point in both objects
        let hits = index.query_point(75.0, 75.0);
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0], "obj_2"); // Top-most first

        // Point only in first object
        let hits = index.query_point(25.0, 25.0);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0], "obj_1");

        // Point outside
        let hits = index.query_point(200.0, 200.0);
        assert!(hits.is_empty());
    }
}
