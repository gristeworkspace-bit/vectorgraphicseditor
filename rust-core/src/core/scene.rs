//! Scene module - Scene graph and vector object definitions
//!
//! Uses the Composite Pattern for hierarchical scene structure

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::math::TransformMatrix;

/// Unique identifier for scene objects
pub type ObjectId = String;

/// Vector object types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VectorObject {
    Rectangle {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    Ellipse {
        cx: f64,
        cy: f64,
        rx: f64,
        ry: f64,
    },
    Path {
        commands: Vec<PathCommand>,
        /// Whether the path is closed (ends with ClosePath command)
        /// Default true for backward compatibility with existing save files
        #[serde(default = "default_true")]
        is_closed: bool,
    },
}

/// Default function for is_closed field (backward compatibility)
fn default_true() -> bool {
    true
}

/// SVG-compatible path commands
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PathCommand {
    MoveTo { x: f64, y: f64 },
    LineTo { x: f64, y: f64 },
    CurveTo { x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64 },
    ClosePath,
}

/// Scene node - either a group or a leaf object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SceneNode {
    Group {
        id: ObjectId,
        children: Vec<SceneNode>,
        transform: TransformMatrix,
    },
    Leaf {
        id: ObjectId,
        object: VectorObject,
        transform: TransformMatrix,
        style: ObjectStyle,
    },
}

/// Visual style for objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectStyle {
    pub fill_color: Option<String>,
    pub stroke_color: Option<String>,
    pub stroke_width: f64,
}

impl Default for ObjectStyle {
    fn default() -> Self {
        ObjectStyle {
            fill_color: Some("#3b82f6".to_string()), // Blue
            stroke_color: Some("#1e40af".to_string()), // Dark blue
            stroke_width: 2.0,
        }
    }
}

/// Scene graph - manages all objects in the scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneGraph {
    /// Root nodes (top-level objects)
    pub roots: Vec<SceneNode>,
    /// Counter for generating unique IDs
    id_counter: u64,
    /// Quick lookup for object transforms (for future spatial indexing)
    #[serde(skip)]
    transform_cache: HashMap<ObjectId, TransformMatrix>,
}

impl SceneGraph {
    /// Create a new empty scene graph
    pub fn new() -> Self {
        SceneGraph {
            roots: Vec::new(),
            id_counter: 0,
            transform_cache: HashMap::new(),
        }
    }

    /// Generate a unique object ID
    pub fn generate_id(&mut self) -> ObjectId {
        self.id_counter += 1;
        format!("obj_{}", self.id_counter)
    }

    /// Add an object to the scene root
    pub fn add_object(&mut self, id: ObjectId, object: VectorObject, transform: TransformMatrix) {
        self.transform_cache.insert(id.clone(), transform);
        let node = SceneNode::Leaf {
            id,
            object,
            transform,
            style: ObjectStyle::default(),
        };
        self.roots.push(node);
    }

    /// Get the total number of objects in the scene
    pub fn object_count(&self) -> usize {
        self.count_nodes(&self.roots)
    }

    fn count_nodes(&self, nodes: &[SceneNode]) -> usize {
        nodes.iter().map(|node| match node {
            SceneNode::Leaf { .. } => 1,
            SceneNode::Group { children, .. } => 1 + self.count_nodes(children),
        }).sum()
    }

    /// Iterate over all leaf nodes with their accumulated transforms
    pub fn iter_leaves(&self) -> Vec<(&VectorObject, TransformMatrix, &ObjectStyle)> {
        let mut result = Vec::new();
        self.collect_leaves(&self.roots, TransformMatrix::identity(), &mut result);
        result
    }

    fn collect_leaves<'a>(
        &'a self,
        nodes: &'a [SceneNode],
        parent_transform: TransformMatrix,
        result: &mut Vec<(&'a VectorObject, TransformMatrix, &'a ObjectStyle)>,
    ) {
        for node in nodes {
            match node {
                SceneNode::Leaf { object, transform, style, .. } => {
                    let world_transform = parent_transform.multiply(transform);
                    result.push((object, world_transform, style));
                }
                SceneNode::Group { children, transform, .. } => {
                    let world_transform = parent_transform.multiply(transform);
                    self.collect_leaves(children, world_transform, result);
                }
            }
        }
    }

    /// Get a node by ID (immutable)
    pub fn get_node_by_id(&self, target_id: &str) -> Option<&SceneNode> {
        self.find_node_by_id(&self.roots, target_id)
    }

    fn find_node_by_id<'a>(&'a self, nodes: &'a [SceneNode], target_id: &str) -> Option<&'a SceneNode> {
        for node in nodes {
            match node {
                SceneNode::Leaf { id, .. } if id == target_id => return Some(node),
                SceneNode::Group { id, children, .. } => {
                    if id == target_id {
                        return Some(node);
                    }
                    if let Some(found) = self.find_node_by_id(children, target_id) {
                        return Some(found);
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Get a node by ID (mutable)
    /// Note: For deep hierarchies, this may not find nested nodes. Use for flat scenes.
    pub fn get_node_by_id_mut(&mut self, target_id: &str) -> Option<&mut SceneNode> {
        self.roots.iter_mut().find(|node| {
            match node {
                SceneNode::Leaf { id, .. } => id == target_id,
                SceneNode::Group { id, .. } => id == target_id,
            }
        })
    }

    /// Bring a node to the front (end of the vector = top of z-order)
    pub fn bring_to_front(&mut self, target_id: &str) -> bool {
        // Find the index of the node with the given ID
        if let Some(index) = self.roots.iter().position(|node| {
            match node {
                SceneNode::Leaf { id, .. } => id == target_id,
                SceneNode::Group { id, .. } => id == target_id,
            }
        }) {
            // Only move if not already at the end
            if index < self.roots.len() - 1 {
                let node = self.roots.remove(index);
                self.roots.push(node);
                return true;
            }
        }
        false
    }

    /// Send a node to the back (beginning of the vector = bottom of z-order)
    pub fn send_to_back(&mut self, target_id: &str) -> bool {
        // Find the index of the node with the given ID
        if let Some(index) = self.roots.iter().position(|node| {
            match node {
                SceneNode::Leaf { id, .. } => id == target_id,
                SceneNode::Group { id, .. } => id == target_id,
            }
        }) {
            // Only move if not already at the beginning
            if index > 0 {
                let node = self.roots.remove(index);
                self.roots.insert(0, node);
                return true;
            }
        }
        false
    }
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_object() {
        let mut scene = SceneGraph::new();
        let id = scene.generate_id();
        scene.add_object(
            id.clone(),
            VectorObject::Rectangle { x: 0.0, y: 0.0, width: 100.0, height: 100.0 },
            TransformMatrix::identity(),
        );
        assert_eq!(scene.object_count(), 1);
    }

    #[test]
    fn test_iter_leaves() {
        let mut scene = SceneGraph::new();
        let id1 = scene.generate_id();
        let id2 = scene.generate_id();
        scene.add_object(
            id1,
            VectorObject::Rectangle { x: 0.0, y: 0.0, width: 100.0, height: 100.0 },
            TransformMatrix::translate(10.0, 20.0),
        );
        scene.add_object(
            id2,
            VectorObject::Ellipse { cx: 50.0, cy: 50.0, rx: 30.0, ry: 20.0 },
            TransformMatrix::identity(),
        );
        
        let leaves = scene.iter_leaves();
        assert_eq!(leaves.len(), 2);
    }
}
