//! Vector Graphics Editor - Rust Core Engine
//! 
//! This is the core engine for the vector graphics editor.
//! It handles all geometry calculations, scene management, and rendering commands.

use std::collections::HashSet;
use wasm_bindgen::prelude::*;

pub mod core;
pub mod drag_state;
pub mod hit_test;
pub mod pen_state;
pub mod renderer;
pub mod spatial;
pub mod text_engine;

use crate::core::math::TransformMatrix;
use crate::core::scene::{PathCommand, SceneGraph, SceneNode, VectorObject};
use crate::drag_state::{DragMode, DragState, HandleIndex};
use crate::hit_test::hit_test_object;
use crate::pen_state::PenState;
use crate::renderer::SelectionOverlay;
use crate::spatial::BoundingBox;

/// Editor state that holds the entire scene
#[wasm_bindgen]
pub struct Editor {
    scene: SceneGraph,
    selected_ids: HashSet<String>,
    drag_state: DragState,
    pen_state: PenState,
    // History for undo/redo
    undo_stack: Vec<SceneGraph>,
    redo_stack: Vec<SceneGraph>,
    max_history: usize,
}

#[wasm_bindgen]
impl Editor {
    /// Create a new editor instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Set panic hook for better error messages
        console_error_panic_hook::set_once();
        
        Editor {
            scene: SceneGraph::new(),
            selected_ids: HashSet::new(),
            drag_state: DragState::new(),
            pen_state: PenState::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history: 50, // Keep up to 50 undo states
        }
    }

    /// Add a rectangle to the scene
    pub fn add_rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) -> String {
        let id = self.scene.generate_id();
        let rect = VectorObject::Rectangle { x, y, width, height };
        self.scene.add_object(id.clone(), rect, TransformMatrix::identity());
        id
    }

    /// Add an ellipse to the scene
    pub fn add_ellipse(&mut self, cx: f64, cy: f64, rx: f64, ry: f64) -> String {
        let id = self.scene.generate_id();
        let ellipse = VectorObject::Ellipse { cx, cy, rx, ry };
        self.scene.add_object(id.clone(), ellipse, TransformMatrix::identity());
        id
    }

    /// Add a rotated rectangle to the scene (for testing hit detection)
    /// cx, cy: center position, width, height: size, angle_degrees: rotation in degrees
    pub fn add_rotated_rectangle(&mut self, cx: f64, cy: f64, width: f64, height: f64, angle_degrees: f64) -> String {
        let id = self.scene.generate_id();
        // Create rectangle centered at origin
        let rect = VectorObject::Rectangle { 
            x: -width / 2.0, 
            y: -height / 2.0, 
            width, 
            height 
        };
        
        // Create transform: translate to center, then rotate
        let angle_radians = angle_degrees * std::f64::consts::PI / 180.0;
        let rotation = TransformMatrix::rotate(angle_radians);
        let translation = TransformMatrix::translate(cx, cy);
        // Combined transform: first rotate around origin, then translate to position
        let transform = translation.multiply(&rotation);
        
        self.scene.add_object(id.clone(), rect, transform);
        id
    }

    /// Add a path from JSON commands string
    /// Each command: {"type": "MoveTo", "x": 0, "y": 0} etc.
    pub fn add_path(&mut self, commands_json: &str) -> String {
        let id = self.scene.generate_id();
        let commands: Vec<PathCommand> = serde_json::from_str(commands_json).unwrap_or_default();
        let path = VectorObject::Path { commands, is_closed: true };
        self.scene.add_object(id.clone(), path, TransformMatrix::identity());
        id
    }

    /// Add a heart-shaped path at the specified center position (for testing)
    pub fn add_heart_path(&mut self, cx: f64, cy: f64, size: f64) -> String {
        let id = self.scene.generate_id();
        
        // Heart shape using cubic bezier curves
        // Based on: https://codepen.io/jonitrythall/pen/OyxdBE
        let scale = size / 100.0;
        let commands = vec![
            // Start at bottom point of heart
            PathCommand::MoveTo { x: 0.0 * scale, y: 30.0 * scale },
            // Left curve up to top
            PathCommand::CurveTo { 
                x1: -50.0 * scale, y1: -20.0 * scale, 
                x2: -50.0 * scale, y2: -70.0 * scale, 
                x: 0.0 * scale, y: -50.0 * scale 
            },
            // Right curve down to bottom
            PathCommand::CurveTo { 
                x1: 50.0 * scale, y1: -70.0 * scale, 
                x2: 50.0 * scale, y2: -20.0 * scale, 
                x: 0.0 * scale, y: 30.0 * scale 
            },
            PathCommand::ClosePath,
        ];
        
        let path = VectorObject::Path { commands, is_closed: true };
        // Position at center
        let transform = TransformMatrix::translate(cx, cy);
        self.scene.add_object(id.clone(), path, transform);
        id
    }

    pub fn get_render_commands(&self) -> String {
        let commands = renderer::generate_render_commands(&self.scene);
        serde_json::to_string(&commands).unwrap_or_else(|_| "[]".to_string())
    }

    /// Get selection overlay commands as JSON string
    pub fn get_selection_overlay(&self) -> String {
        let overlays = self.generate_selection_overlays();
        serde_json::to_string(&overlays).unwrap_or_else(|_| "[]".to_string())
    }

    /// Get the number of objects in the scene
    pub fn object_count(&self) -> usize {
        self.scene.object_count()
    }

    /// Hit test at a point, returns the ID of the top-most object hit (or empty string)
    pub fn hit_test(&self, x: f64, y: f64) -> String {
        // Iterate leaves in reverse order (top-most first)
        let leaves: Vec<_> = self.scene.iter_leaves();
        for (object, transform, _style) in leaves.into_iter().rev() {
            if hit_test_object(x, y, object, &transform) {
                // Find the ID by matching the object
                if let Some(id) = self.find_id_for_object(object) {
                    return id;
                }
            }
        }
        String::new()
    }

    /// Select object at point (replaces current selection)
    pub fn select_at(&mut self, x: f64, y: f64) -> String {
        self.selected_ids.clear();
        let id = self.hit_test(x, y);
        if !id.is_empty() {
            self.selected_ids.insert(id.clone());
        }
        id
    }

    /// Add to selection at point (Shift+Click behavior)
    pub fn add_to_selection_at(&mut self, x: f64, y: f64) -> String {
        let id = self.hit_test(x, y);
        if !id.is_empty() {
            if self.selected_ids.contains(&id) {
                self.selected_ids.remove(&id);
            } else {
                self.selected_ids.insert(id.clone());
            }
        }
        id
    }

    /// Deselect all objects
    pub fn deselect_all(&mut self) {
        self.selected_ids.clear();
    }

    /// Get selected IDs as JSON array
    pub fn get_selected_ids(&self) -> String {
        let ids: Vec<&String> = self.selected_ids.iter().collect();
        serde_json::to_string(&ids).unwrap_or_else(|_| "[]".to_string())
    }

    /// Check if any object is selected
    pub fn has_selection(&self) -> bool {
        !self.selected_ids.is_empty()
    }

    /// Get style of first selected object as JSON
    /// Returns: { fill: "#color" | null, stroke: "#color" | null, strokeWidth: number }
    pub fn get_selected_style(&self) -> String {
        if let Some(id) = self.selected_ids.iter().next() {
            if let Some(node) = self.scene.get_node_by_id(id) {
                if let SceneNode::Leaf { style, .. } = node {
                    let json = serde_json::json!({
                        "fill": style.fill_color,
                        "stroke": style.stroke_color,
                        "strokeWidth": style.stroke_width,
                    });
                    return serde_json::to_string(&json).unwrap_or_else(|_| "{}".to_string());
                }
            }
        }
        "{}".to_string()
    }

    /// Update style of all selected objects
    pub fn update_style(&mut self, fill: &str, stroke: &str, stroke_width: f64) {
        let fill_color = if fill == "none" || fill.is_empty() { None } else { Some(fill.to_string()) };
        let stroke_color = if stroke == "none" || stroke.is_empty() { None } else { Some(stroke.to_string()) };
        
        for id in &self.selected_ids.clone() {
            if let Some(node) = self.scene.get_node_by_id_mut(id) {
                if let SceneNode::Leaf { style, .. } = node {
                    style.fill_color = fill_color.clone();
                    style.stroke_color = stroke_color.clone();
                    style.stroke_width = stroke_width;
                }
            }
        }
    }

    /// Bring the first selected object to the front (top of z-order)
    pub fn bring_to_front(&mut self) -> bool {
        if let Some(id) = self.selected_ids.iter().next().cloned() {
            return self.scene.bring_to_front(&id);
        }
        false
    }

    /// Send the first selected object to the back (bottom of z-order)
    pub fn send_to_back(&mut self) -> bool {
        if let Some(id) = self.selected_ids.iter().next().cloned() {
            return self.scene.send_to_back(&id);
        }
        false
    }

    // ==============================================
    // Persistence APIs (Save/Load)
    // ==============================================

    /// Export the entire scene to a JSON string
    pub fn export_scene_to_json(&self) -> String {
        serde_json::to_string_pretty(&self.scene).unwrap_or_else(|_| "{}".to_string())
    }

    /// Import a scene from a JSON string, replacing the current scene
    /// Returns true if successful, false if parsing failed
    pub fn import_scene_from_json(&mut self, json: &str) -> bool {
        match serde_json::from_str::<SceneGraph>(json) {
            Ok(scene) => {
                self.scene = scene;
                self.selected_ids.clear();
                self.drag_state.end();
                self.pen_state = PenState::Idle;
                true
            }
            Err(_) => false,
        }
    }

    /// Clear the entire scene
    pub fn clear_scene(&mut self) {
        self.scene = SceneGraph::new();
        self.selected_ids.clear();
        self.drag_state.end();
        self.pen_state = PenState::Idle;
    }

    /// Export the scene to SVG format
    pub fn export_to_svg(&self, width: u32, height: u32) -> String {
        crate::renderer::generate_svg(&self.scene, width, height)
    }

    // ==============================================
    // Undo/Redo APIs
    // ==============================================

    /// Save a snapshot of the current scene for undo
    /// Call this BEFORE making a destructive change
    pub fn save_snapshot(&mut self) {
        // Clone current scene and push to undo stack
        self.undo_stack.push(self.scene.clone());
        
        // Clear redo stack when new action is performed
        self.redo_stack.clear();
        
        // Limit history size
        while self.undo_stack.len() > self.max_history {
            self.undo_stack.remove(0);
        }
    }

    /// Undo the last operation
    /// Returns true if undo was performed, false if nothing to undo
    pub fn undo(&mut self) -> bool {
        if let Some(previous_scene) = self.undo_stack.pop() {
            // Save current state to redo stack
            self.redo_stack.push(self.scene.clone());
            
            // Restore previous state
            self.scene = previous_scene;
            self.selected_ids.clear();
            self.drag_state.end();
            
            true
        } else {
            false
        }
    }

    /// Redo the last undone operation
    /// Returns true if redo was performed, false if nothing to redo
    pub fn redo(&mut self) -> bool {
        if let Some(next_scene) = self.redo_stack.pop() {
            // Save current state to undo stack
            self.undo_stack.push(self.scene.clone());
            
            // Restore next state
            self.scene = next_scene;
            self.selected_ids.clear();
            self.drag_state.end();
            
            true
        } else {
            false
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the size of the undo stack
    pub fn undo_stack_size(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the size of the redo stack
    pub fn redo_stack_size(&self) -> usize {
        self.redo_stack.len()
    }

    /// Move selected objects by delta
    /// Note: For precise movement, use begin_move_drag/update_move_drag/end_drag instead
    pub fn move_selected(&mut self, dx: f64, dy: f64) {
        for id in &self.selected_ids.clone() {
            if let Some(node) = self.scene.get_node_by_id_mut(id) {
                if let SceneNode::Leaf { transform, .. } = node {
                    // Apply translation to existing transform
                    let translation = TransformMatrix::translate(dx, dy);
                    *transform = translation.multiply(transform);
                }
            }
        }
    }

    /// Begin a move drag operation - saves initial transforms
    pub fn begin_move_drag(&mut self, start_x: f64, start_y: f64) {
        let mut initial_transforms = std::collections::HashMap::new();
        for id in &self.selected_ids {
            if let Some(node) = self.scene.get_node_by_id(id) {
                if let SceneNode::Leaf { transform, .. } = node {
                    initial_transforms.insert(id.clone(), *transform);
                }
            }
        }
        self.drag_state.begin(
            DragMode::Moving,
            start_x,
            start_y,
            initial_transforms,
            (0.0, 0.0), // No pivot needed for move
        );
    }

    /// Update move drag - applies delta from baseline (no cumulative error)
    pub fn update_move_drag(&mut self, current_x: f64, current_y: f64) {
        if !self.drag_state.is_active() || self.drag_state.mode != DragMode::Moving {
            return;
        }
        
        let (dx, dy) = self.drag_state.delta(current_x, current_y);
        let translation = TransformMatrix::translate(dx, dy);
        
        for id in &self.selected_ids.clone() {
            if let Some(initial) = self.drag_state.get_initial_transform(id) {
                if let Some(node) = self.scene.get_node_by_id_mut(id) {
                    if let SceneNode::Leaf { transform, .. } = node {
                        // Apply translation to INITIAL transform (not current!)
                        *transform = translation.multiply(initial);
                    }
                }
            }
        }
    }

    /// End drag operation
    pub fn end_drag(&mut self) {
        self.drag_state.end();
    }

    /// Check if a drag operation is in progress
    pub fn is_dragging(&self) -> bool {
        self.drag_state.is_active()
    }

    /// Begin a resize drag operation
    /// handle_index: 0=TopLeft, 1=TopRight, 2=BottomRight, 3=BottomLeft
    pub fn begin_resize_drag(&mut self, start_x: f64, start_y: f64, handle_index: u8) {
        let handle = match handle_index {
            0 => HandleIndex::TopLeft,
            1 => HandleIndex::TopRight,
            2 => HandleIndex::BottomRight,
            3 => HandleIndex::BottomLeft,
            _ => return,
        };

        // Get initial transforms and calculate pivot (opposite corner)
        let mut initial_transforms = std::collections::HashMap::new();
        let mut pivot = (0.0, 0.0);
        
        // Get the first selected object's opposite corner as pivot
        if let Some(id) = self.selected_ids.iter().next() {
            if let Some(overlay) = self.generate_selection_overlays().iter().find(|o| &o.id == id) {
                let opposite_idx = handle.opposite() as usize;
                pivot = overlay.corners[opposite_idx];
                
                // Store initial transforms for all selected objects
                for sel_id in &self.selected_ids {
                    if let Some(node) = self.scene.get_node_by_id(sel_id) {
                        if let SceneNode::Leaf { transform, .. } = node {
                            initial_transforms.insert(sel_id.clone(), *transform);
                        }
                    }
                }
            }
        }

        self.drag_state.begin(
            DragMode::Resizing(handle),
            start_x,
            start_y,
            initial_transforms,
            pivot,
        );
    }

    /// Update resize drag - scales from pivot point
    pub fn update_resize_drag(&mut self, current_x: f64, current_y: f64) {
        let (handle, pivot) = match &self.drag_state.mode {
            DragMode::Resizing(h) => (*h, self.drag_state.pivot),
            _ => return,
        };

        let (start_x, start_y) = self.drag_state.start_point;
        
        // Calculate distance from pivot at start and current positions
        let start_dx = start_x - pivot.0;
        let start_dy = start_y - pivot.1;
        let current_dx = current_x - pivot.0;
        let current_dy = current_y - pivot.1;
        
        // Calculate scale factors with minimum to prevent zero/negative scale
        let start_dist = (start_dx * start_dx + start_dy * start_dy).sqrt().max(1.0);
        let current_dist = (current_dx * current_dx + current_dy * current_dy).sqrt().max(1.0);
        
        // Uniform scale to maintain aspect ratio
        let scale = current_dist / start_dist;
        let scale = scale.max(0.1).min(10.0); // Clamp to reasonable range
        
        // Apply scale around pivot to each selected object
        let scale_matrix = TransformMatrix::scale_around(scale, scale, pivot.0, pivot.1);
        
        for id in &self.selected_ids.clone() {
            if let Some(initial) = self.drag_state.get_initial_transform(id) {
                if let Some(node) = self.scene.get_node_by_id_mut(id) {
                    if let SceneNode::Leaf { transform, .. } = node {
                        // Apply scale to INITIAL transform
                        *transform = scale_matrix.multiply(initial);
                    }
                }
            }
        }
    }

    /// Get handle positions for the first selected object (for hit testing in frontend)
    /// Returns JSON: [[x,y], [x,y], [x,y], [x,y]] or "[]" if no selection
    pub fn get_handle_positions(&self) -> String {
        if let Some(overlay) = self.generate_selection_overlays().first() {
            serde_json::to_string(&overlay.corners).unwrap_or_else(|_| "[]".to_string())
        } else {
            "[]".to_string()
        }
    }

    /// Get the center point of the selection bounding box
    /// Returns JSON: [x, y] or "[]" if no selection
    pub fn get_selection_center(&self) -> String {
        if let Some(overlay) = self.generate_selection_overlays().first() {
            // Calculate center from corners
            let corners = &overlay.corners;
            let cx = (corners[0].0 + corners[1].0 + corners[2].0 + corners[3].0) / 4.0;
            let cy = (corners[0].1 + corners[1].1 + corners[2].1 + corners[3].1) / 4.0;
            serde_json::to_string(&[cx, cy]).unwrap_or_else(|_| "[]".to_string())
        } else {
            "[]".to_string()
        }
    }

    /// Begin a rotation drag operation
    /// Uses the center of the bounding box as pivot
    pub fn begin_rotate_drag(&mut self, start_x: f64, start_y: f64) {
        // Get initial transforms and calculate center as pivot
        let mut initial_transforms = std::collections::HashMap::new();
        let mut center = (0.0, 0.0);
        
        // Calculate center from selection overlay
        if let Some(overlay) = self.generate_selection_overlays().first() {
            let corners = &overlay.corners;
            center = (
                (corners[0].0 + corners[1].0 + corners[2].0 + corners[3].0) / 4.0,
                (corners[0].1 + corners[1].1 + corners[2].1 + corners[3].1) / 4.0,
            );
            
            // Store initial transforms for all selected objects
            for id in &self.selected_ids {
                if let Some(node) = self.scene.get_node_by_id(id) {
                    if let SceneNode::Leaf { transform, .. } = node {
                        initial_transforms.insert(id.clone(), *transform);
                    }
                }
            }
        }

        self.drag_state.begin(
            DragMode::Rotating,
            start_x,
            start_y,
            initial_transforms,
            center, // Pivot is the center
        );
    }

    /// Update rotation drag - rotates around center
    pub fn update_rotate_drag(&mut self, current_x: f64, current_y: f64) {
        if self.drag_state.mode != DragMode::Rotating {
            return;
        }

        let pivot = self.drag_state.pivot;
        let (start_x, start_y) = self.drag_state.start_point;
        
        // Calculate angles from center to start and current points
        let start_angle = (start_y - pivot.1).atan2(start_x - pivot.0);
        let current_angle = (current_y - pivot.1).atan2(current_x - pivot.0);
        // Negate delta to fix rotation direction (screen Y-axis points down)
        let delta_angle = -(current_angle - start_angle);
        
        // Apply rotation around center to each selected object
        let rotation_matrix = TransformMatrix::rotate_around(delta_angle, pivot.0, pivot.1);
        
        for id in &self.selected_ids.clone() {
            if let Some(initial) = self.drag_state.get_initial_transform(id) {
                if let Some(node) = self.scene.get_node_by_id_mut(id) {
                    if let SceneNode::Leaf { transform, .. } = node {
                        // Apply rotation to INITIAL transform
                        *transform = rotation_matrix.multiply(initial);
                    }
                }
            }
        }
    }

    // ==============================================
    // Pen Tool APIs
    // ==============================================

    /// Handle pen tool mouse down
    /// Returns true if near start point (for closing path)
    pub fn pen_down(&mut self, x: f64, y: f64) -> bool {
        const CLOSE_THRESHOLD: f64 = 15.0;
        
        match &self.pen_state {
            PenState::Idle => {
                // Start a new path
                self.pen_state = PenState::Drawing {
                    commands: vec![PathCommand::MoveTo { x, y }],
                    start_point: (x, y),
                    last_anchor: (x, y),
                    drag_start_anchor: None,
                    drag_handle: None,
                    is_dragging: false,
                };
                false
            }
            PenState::Drawing { start_point, commands, .. } => {
                // Check if closing the path
                if commands.len() >= 2 {
                    let dx = x - start_point.0;
                    let dy = y - start_point.1;
                    if (dx * dx + dy * dy).sqrt() < CLOSE_THRESHOLD {
                        return true; // Signal that we should close
                    }
                }
                
                // Mark with FIXED endpoint position (drag_start_anchor)
                if let PenState::Drawing { is_dragging, drag_handle, drag_start_anchor, .. } = &mut self.pen_state {
                    *is_dragging = false;
                    *drag_start_anchor = Some((x, y)); // FIXED endpoint!
                    *drag_handle = Some((x, y)); // Initially same as click position
                }
                false
            }
        }
    }

    /// Handle pen tool mouse move (for dragging to create curves)
    pub fn pen_move(&mut self, x: f64, y: f64) {
        if let PenState::Drawing { drag_handle, is_dragging, .. } = &mut self.pen_state {
            *drag_handle = Some((x, y));
            *is_dragging = true;
        }
    }

    /// Handle pen tool mouse up - confirm the anchor
    pub fn pen_up(&mut self, _x: f64, _y: f64) {
        let new_state = match &self.pen_state {
            PenState::Drawing { commands, start_point, last_anchor, drag_start_anchor, drag_handle, is_dragging } => {
                let mut new_commands = commands.clone();
                
                if *is_dragging {
                    // Use drag_start_anchor as the FIXED endpoint
                    if let (Some((end_x, end_y)), Some((cp2x, cp2y))) = (drag_start_anchor, drag_handle) {
                        // CP1 = start point (C-curve: straight exit from start)
                        // CP2 = mouse position during drag
                        let cp1x = last_anchor.0;
                        let cp1y = last_anchor.1;
                        
                        new_commands.push(PathCommand::CurveTo {
                            x1: cp1x, y1: cp1y,
                            x2: *cp2x, y2: *cp2y,
                            x: *end_x, y: *end_y,
                        });
                        
                        Some(PenState::Drawing {
                            commands: new_commands,
                            start_point: *start_point,
                            last_anchor: (*end_x, *end_y), // New anchor is at endpoint
                            drag_start_anchor: None,
                            drag_handle: None,
                            is_dragging: false,
                        })
                    } else {
                        None
                    }
                } else if let Some((end_x, end_y)) = drag_start_anchor {
                    // Simple click - add a line to where user clicked
                    new_commands.push(PathCommand::LineTo { x: *end_x, y: *end_y });
                    
                    Some(PenState::Drawing {
                        commands: new_commands,
                        start_point: *start_point,
                        last_anchor: (*end_x, *end_y),
                        drag_start_anchor: None,
                        drag_handle: None,
                        is_dragging: false,
                    })
                } else {
                    None
                }
            }
            PenState::Idle => None,
        };
        
        if let Some(state) = new_state {
            self.pen_state = state;
        }
    }

    /// Close the current path and add it to the scene (is_closed = true)
    /// Called when user clicks on start point
    pub fn pen_close(&mut self) -> String {
        if let PenState::Drawing { mut commands, .. } = std::mem::take(&mut self.pen_state) {
            commands.push(PathCommand::ClosePath);
            
            let id = self.scene.generate_id();
            let path = VectorObject::Path { commands, is_closed: true };
            self.scene.add_object(id.clone(), path, TransformMatrix::identity());
            
            self.pen_state = PenState::Idle;
            return id;
        }
        String::new()
    }

    /// Finish the current path without closing it (is_closed = false)
    /// Called when user presses Enter key
    pub fn pen_finish(&mut self) -> String {
        if let PenState::Drawing { commands, .. } = std::mem::take(&mut self.pen_state) {
            // Don't add ClosePath command - leave path open
            if commands.len() < 2 {
                // Need at least 2 points to make a valid open path
                self.pen_state = PenState::Idle;
                return String::new();
            }
            
            let id = self.scene.generate_id();
            let path = VectorObject::Path { commands, is_closed: false };
            self.scene.add_object(id.clone(), path, TransformMatrix::identity());
            
            self.pen_state = PenState::Idle;
            return id;
        }
        String::new()
    }

    /// Cancel pen drawing without saving
    pub fn pen_cancel(&mut self) {
        self.pen_state = PenState::Idle;
    }

    /// Check if pen tool is currently drawing
    pub fn is_pen_drawing(&self) -> bool {
        self.pen_state.is_drawing()
    }

    /// Get current pen path preview as JSON for rendering
    /// Returns: { commands: [...], last_anchor: [x, y], handle: [x, y] | null, is_dragging: bool, preview_curve: {...} | null }
    pub fn get_pen_preview(&self) -> String {
        match &self.pen_state {
            PenState::Drawing { commands, drag_handle, drag_start_anchor, last_anchor, is_dragging, .. } => {
                // If dragging, calculate a preview curve
                // - Start: last_anchor (previous confirmed point)
                // - End: drag_start_anchor (where user clicked - FIXED!)
                // - CP2: drag_handle (mouse position - creates curvature!)
                // - CP1: same as start point (Corner Point - straight exit from start)
                let preview_curve = if *is_dragging {
                    if let (Some((end_x, end_y)), Some((cp2x, cp2y))) = (drag_start_anchor, drag_handle) {
                        // Curve from last_anchor to drag_start_anchor (fixed endpoint)
                        // CP1 = start point (straight exit, no handle at start = C-curve)
                        // CP2 = mouse position (controls the curve toward the end)
                        let cp1x = last_anchor.0;
                        let cp1y = last_anchor.1;
                        
                        Some(serde_json::json!({
                            "type": "CurveTo",
                            "x1": cp1x,
                            "y1": cp1y,
                            "x2": cp2x,
                            "y2": cp2y,
                            "x": end_x,
                            "y": end_y,
                        }))
                    } else {
                        None
                    }
                } else {
                    None
                };

                let preview = serde_json::json!({
                    "commands": commands,
                    "last_anchor": [last_anchor.0, last_anchor.1],
                    "drag_start_anchor": drag_start_anchor,
                    "handle": drag_handle,
                    "is_dragging": is_dragging,
                    "preview_curve": preview_curve,
                });
                serde_json::to_string(&preview).unwrap_or_else(|_| "{}".to_string())
            }
            PenState::Idle => "{}".to_string(),
        }
    }

    // ==============================================
    // Path Editing APIs (Direct Selection Tool)
    // ==============================================

    /// Check if the first selected object is a Path
    pub fn selected_is_path(&self) -> bool {
        if let Some(id) = self.selected_ids.iter().next() {
            if let Some(node) = self.scene.get_node_by_id(id) {
                if let SceneNode::Leaf { object, .. } = node {
                    return matches!(object, VectorObject::Path { .. });
                }
            }
        }
        false
    }

    /// Get path points for the specified object as JSON
    /// Returns: [ { "x": f64, "y": f64, "type": "move"|"line"|"curve" }, ... ]
    pub fn get_path_points(&self, id: &str) -> String {
        if let Some(node) = self.scene.get_node_by_id(id) {
            if let SceneNode::Leaf { object, transform, .. } = node {
                if let VectorObject::Path { commands, .. } = object {
                    let mut points = Vec::new();
                    
                    for cmd in commands {
                        match cmd {
                            PathCommand::MoveTo { x, y } => {
                                // Transform local coords to world coords
                                let (wx, wy) = transform.transform_point(*x, *y);
                                points.push(serde_json::json!({
                                    "x": wx,
                                    "y": wy,
                                    "type": "move"
                                }));
                            }
                            PathCommand::LineTo { x, y } => {
                                let (wx, wy) = transform.transform_point(*x, *y);
                                points.push(serde_json::json!({
                                    "x": wx,
                                    "y": wy,
                                    "type": "line"
                                }));
                            }
                            PathCommand::CurveTo { x, y, .. } => {
                                // For now, just return the endpoint (not control points)
                                let (wx, wy) = transform.transform_point(*x, *y);
                                points.push(serde_json::json!({
                                    "x": wx,
                                    "y": wy,
                                    "type": "curve"
                                }));
                            }
                            PathCommand::ClosePath => {
                                // ClosePath has no coordinates
                            }
                        }
                    }
                    
                    return serde_json::to_string(&points).unwrap_or_else(|_| "[]".to_string());
                }
            }
        }
        "[]".to_string()
    }

    /// Update a path point at the given index
    /// Sets the x, y coordinates of the command at position `index`
    pub fn update_path_point(&mut self, id: &str, index: usize, world_x: f64, world_y: f64) {
        if let Some(node) = self.scene.get_node_by_id_mut(id) {
            if let SceneNode::Leaf { object, transform, .. } = node {
                if let VectorObject::Path { commands, .. } = object {
                    // Transform world coords back to local coords
                    if let Some(inverse) = transform.inverse() {
                        let (local_x, local_y) = inverse.transform_point(world_x, world_y);
                        
                        // Find the command at the given index and update it
                        let mut point_idx = 0;
                        for cmd in commands.iter_mut() {
                            match cmd {
                                PathCommand::MoveTo { x, y } => {
                                    if point_idx == index {
                                        *x = local_x;
                                        *y = local_y;
                                        return;
                                    }
                                    point_idx += 1;
                                }
                                PathCommand::LineTo { x, y } => {
                                    if point_idx == index {
                                        *x = local_x;
                                        *y = local_y;
                                        return;
                                    }
                                    point_idx += 1;
                                }
                                PathCommand::CurveTo { x, y, .. } => {
                                    // Only update endpoint, not control points
                                    if point_idx == index {
                                        *x = local_x;
                                        *y = local_y;
                                        return;
                                    }
                                    point_idx += 1;
                                }
                                PathCommand::ClosePath => {
                                    // No coordinates to update
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Private helper methods (not exposed to Wasm)
impl Editor {
    fn find_id_for_object(&self, target: &VectorObject) -> Option<String> {
        for node in &self.scene.roots {
            if let SceneNode::Leaf { id, object, .. } = node {
                if std::ptr::eq(object, target) {
                    return Some(id.clone());
                }
            }
        }
        None
    }

    fn generate_selection_overlays(&self) -> Vec<SelectionOverlay> {
        let mut overlays = Vec::new();
        
        for (object, transform, _style) in self.scene.iter_leaves() {
            // Check if this object is selected
            if let Some(id) = self.find_id_for_object(object) {
                if self.selected_ids.contains(&id) {
                    // Get local bounding box
                    let local_bounds = match object {
                        VectorObject::Rectangle { x, y, width, height } => {
                            BoundingBox::from_rect(*x, *y, *width, *height)
                        }
                        VectorObject::Ellipse { cx, cy, rx, ry } => {
                            BoundingBox::from_ellipse(*cx, *cy, *rx, *ry)
                        }
                        VectorObject::Path { commands, .. } => {
                            // Calculate bounding box from all path points
                            let mut min_x = f64::MAX;
                            let mut min_y = f64::MAX;
                            let mut max_x = f64::MIN;
                            let mut max_y = f64::MIN;
                            
                            for cmd in commands {
                                match cmd {
                                    PathCommand::MoveTo { x, y } | PathCommand::LineTo { x, y } => {
                                        min_x = min_x.min(*x);
                                        min_y = min_y.min(*y);
                                        max_x = max_x.max(*x);
                                        max_y = max_y.max(*y);
                                    }
                                    PathCommand::CurveTo { x1, y1, x2, y2, x, y } => {
                                        min_x = min_x.min(*x1).min(*x2).min(*x);
                                        min_y = min_y.min(*y1).min(*y2).min(*y);
                                        max_x = max_x.max(*x1).max(*x2).max(*x);
                                        max_y = max_y.max(*y1).max(*y2).max(*y);
                                    }
                                    PathCommand::ClosePath => {}
                                }
                            }
                            
                            if min_x == f64::MAX {
                                continue; // Empty path
                            }
                            BoundingBox { min_x, min_y, max_x, max_y }
                        }
                    };

                    // Transform corners to world space
                    let corners = [
                        transform.transform_point(local_bounds.min_x, local_bounds.min_y),
                        transform.transform_point(local_bounds.max_x, local_bounds.min_y),
                        transform.transform_point(local_bounds.max_x, local_bounds.max_y),
                        transform.transform_point(local_bounds.min_x, local_bounds.max_y),
                    ];

                    overlays.push(SelectionOverlay {
                        id: id.clone(),
                        corners,
                    });
                }
            }
        }

        overlays
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}
