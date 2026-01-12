//! Pen Tool State Machine
//!
//! Manages the state of the pen tool for drawing bezier paths.

use crate::core::scene::PathCommand;

/// Pen tool state
#[derive(Debug, Clone)]
pub enum PenState {
    /// Not currently drawing
    Idle,
    /// Actively drawing a path
    Drawing {
        /// Path commands built so far
        commands: Vec<PathCommand>,
        /// First anchor point (for close path detection)
        start_point: (f64, f64),
        /// Last confirmed anchor point
        last_anchor: (f64, f64),
        /// Where the user clicked to start dragging (the ENDPOINT - fixed!)
        drag_start_anchor: Option<(f64, f64)>,
        /// Current mouse position during drag (for CP2 control point)
        drag_handle: Option<(f64, f64)>,
        /// Is currently dragging (to distinguish click vs drag)
        is_dragging: bool,
    },
}

impl Default for PenState {
    fn default() -> Self {
        PenState::Idle
    }
}

impl PenState {
    pub fn new() -> Self {
        PenState::Idle
    }

    /// Check if we're currently drawing
    pub fn is_drawing(&self) -> bool {
        matches!(self, PenState::Drawing { .. })
    }

    /// Get the start point if drawing
    pub fn get_start_point(&self) -> Option<(f64, f64)> {
        match self {
            PenState::Drawing { start_point, .. } => Some(*start_point),
            PenState::Idle => None,
        }
    }

    /// Get current commands if drawing
    pub fn get_commands(&self) -> Option<&Vec<PathCommand>> {
        match self {
            PenState::Drawing { commands, .. } => Some(commands),
            PenState::Idle => None,
        }
    }

    /// Check if a point is near the start point (for closing path)
    pub fn is_near_start(&self, x: f64, y: f64, threshold: f64) -> bool {
        match self {
            PenState::Drawing { start_point, commands, .. } => {
                // Need at least 2 points to form a closeable path
                if commands.len() < 2 {
                    return false;
                }
                let dx = x - start_point.0;
                let dy = y - start_point.1;
                (dx * dx + dy * dy).sqrt() < threshold
            }
            PenState::Idle => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pen_state_default() {
        let state = PenState::new();
        assert!(!state.is_drawing());
    }

    #[test]
    fn test_is_near_start() {
        let state = PenState::Drawing {
            commands: vec![
                PathCommand::MoveTo { x: 100.0, y: 100.0 },
                PathCommand::LineTo { x: 200.0, y: 100.0 },
            ],
            start_point: (100.0, 100.0),
            last_anchor: (200.0, 100.0),
            drag_start_anchor: None,
            drag_handle: None,
            is_dragging: false,
        };
        
        assert!(state.is_near_start(105.0, 100.0, 10.0));
        assert!(!state.is_near_start(200.0, 200.0, 10.0));
    }
}
