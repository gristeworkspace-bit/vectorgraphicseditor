//! Drag State Module - Manages drag operations for precise transformations
//!
//! This module implements a state machine to handle drag operations.
//! By saving the initial transform at drag start, we compute all changes
//! from the baseline, preventing cumulative floating-point errors.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::core::math::TransformMatrix;

/// Handle index for resize operations (corners and edges)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandleIndex {
    TopLeft = 0,
    TopRight = 1,
    BottomRight = 2,
    BottomLeft = 3,
}

impl HandleIndex {
    /// Get the opposite corner (for calculating pivot during resize)
    pub fn opposite(&self) -> Self {
        match self {
            HandleIndex::TopLeft => HandleIndex::BottomRight,
            HandleIndex::TopRight => HandleIndex::BottomLeft,
            HandleIndex::BottomRight => HandleIndex::TopLeft,
            HandleIndex::BottomLeft => HandleIndex::TopRight,
        }
    }
}

/// Drag operation mode
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DragMode {
    None,
    Moving,
    Resizing(HandleIndex),
    Rotating,
}

impl Default for DragMode {
    fn default() -> Self {
        DragMode::None
    }
}

/// Drag state tracking
#[derive(Debug, Clone, Default)]
pub struct DragState {
    /// Current drag mode
    pub mode: DragMode,
    /// Mouse position at drag start (world coordinates)
    pub start_point: (f64, f64),
    /// Initial transforms of selected objects at drag start
    pub initial_transforms: HashMap<String, TransformMatrix>,
    /// Pivot point for resize/rotate operations (opposite corner or center)
    pub pivot: (f64, f64),
}

impl DragState {
    /// Create a new inactive drag state
    pub fn new() -> Self {
        DragState::default()
    }

    /// Check if a drag operation is active
    pub fn is_active(&self) -> bool {
        self.mode != DragMode::None
    }

    /// Begin a new drag operation
    pub fn begin(
        &mut self,
        mode: DragMode,
        start_x: f64,
        start_y: f64,
        transforms: HashMap<String, TransformMatrix>,
        pivot: (f64, f64),
    ) {
        self.mode = mode;
        self.start_point = (start_x, start_y);
        self.initial_transforms = transforms;
        self.pivot = pivot;
    }

    /// End the current drag operation
    pub fn end(&mut self) {
        self.mode = DragMode::None;
        self.start_point = (0.0, 0.0);
        self.initial_transforms.clear();
        self.pivot = (0.0, 0.0);
    }

    /// Calculate delta from start point
    pub fn delta(&self, current_x: f64, current_y: f64) -> (f64, f64) {
        (current_x - self.start_point.0, current_y - self.start_point.1)
    }

    /// Get the initial transform for an object
    pub fn get_initial_transform(&self, id: &str) -> Option<&TransformMatrix> {
        self.initial_transforms.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drag_state_lifecycle() {
        let mut state = DragState::new();
        assert!(!state.is_active());

        let mut transforms = HashMap::new();
        transforms.insert("obj_1".to_string(), TransformMatrix::identity());
        
        state.begin(DragMode::Moving, 100.0, 100.0, transforms, (0.0, 0.0));
        assert!(state.is_active());
        assert_eq!(state.mode, DragMode::Moving);

        let (dx, dy) = state.delta(150.0, 120.0);
        assert!((dx - 50.0).abs() < 1e-10);
        assert!((dy - 20.0).abs() < 1e-10);

        state.end();
        assert!(!state.is_active());
    }

    #[test]
    fn test_handle_opposite() {
        assert_eq!(HandleIndex::TopLeft.opposite(), HandleIndex::BottomRight);
        assert_eq!(HandleIndex::BottomRight.opposite(), HandleIndex::TopLeft);
    }
}
