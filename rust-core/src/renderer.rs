//! Renderer module - Generates render commands for the frontend
//!
//! Outputs JSON-serializable commands that the React Canvas component can execute

use serde::{Deserialize, Serialize};


use crate::core::scene::{SceneGraph, VectorObject};

/// Render command types that map to Canvas 2D API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RenderCommand {
    SetTransform {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    },
    BeginPath,
    Rect {
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
    MoveTo {
        x: f64,
        y: f64,
    },
    LineTo {
        x: f64,
        y: f64,
    },
    BezierCurveTo {
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    },
    ClosePath,
    SetFillStyle {
        color: String,
    },
    SetStrokeStyle {
        color: String,
    },
    SetLineWidth {
        width: f64,
    },
    Fill,
    Stroke,
    ResetTransform,
}

/// Selection overlay data for drawing bounding boxes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionOverlay {
    pub id: String,
    /// Corners in world space: [top-left, top-right, bottom-right, bottom-left]
    pub corners: [(f64, f64); 4],
}

/// Generate render commands from the scene graph
pub fn generate_render_commands(scene: &SceneGraph) -> Vec<RenderCommand> {
    let mut commands = Vec::new();

    for (object, transform, style) in scene.iter_leaves() {
        // Set transform
        commands.push(RenderCommand::SetTransform {
            a: transform.a,
            b: transform.c, // Note: Canvas uses different row/column order
            c: transform.b,
            d: transform.d,
            e: transform.tx,
            f: transform.ty,
        });

        // Set style
        if let Some(ref fill) = style.fill_color {
            commands.push(RenderCommand::SetFillStyle { color: fill.clone() });
        }
        if let Some(ref stroke) = style.stroke_color {
            commands.push(RenderCommand::SetStrokeStyle { color: stroke.clone() });
        }
        commands.push(RenderCommand::SetLineWidth { width: style.stroke_width });

        // Begin path
        commands.push(RenderCommand::BeginPath);

        // Draw shape
        match object {
            VectorObject::Rectangle { x, y, width, height } => {
                commands.push(RenderCommand::Rect {
                    x: *x,
                    y: *y,
                    width: *width,
                    height: *height,
                });
            }
            VectorObject::Ellipse { cx, cy, rx, ry } => {
                commands.push(RenderCommand::Ellipse {
                    cx: *cx,
                    cy: *cy,
                    rx: *rx,
                    ry: *ry,
                });
            }
            VectorObject::Path { commands: path_commands } => {
                for cmd in path_commands {
                    match cmd {
                        crate::core::scene::PathCommand::MoveTo { x, y } => {
                            commands.push(RenderCommand::MoveTo { x: *x, y: *y });
                        }
                        crate::core::scene::PathCommand::LineTo { x, y } => {
                            commands.push(RenderCommand::LineTo { x: *x, y: *y });
                        }
                        crate::core::scene::PathCommand::CurveTo { x1, y1, x2, y2, x, y } => {
                            commands.push(RenderCommand::BezierCurveTo {
                                cp1x: *x1,
                                cp1y: *y1,
                                cp2x: *x2,
                                cp2y: *y2,
                                x: *x,
                                y: *y,
                            });
                        }
                        crate::core::scene::PathCommand::ClosePath => {
                            commands.push(RenderCommand::ClosePath);
                        }
                    }
                }
            }
        }

        // Fill and stroke
        if style.fill_color.is_some() {
            commands.push(RenderCommand::Fill);
        }
        if style.stroke_color.is_some() {
            commands.push(RenderCommand::Stroke);
        }

        // Reset transform for next object
        commands.push(RenderCommand::ResetTransform);
    }

    commands
}

/// Generate SVG string from the scene graph
pub fn generate_svg(scene: &SceneGraph, width: u32, height: u32) -> String {
    let mut svg = String::new();
    
    // SVG header
    svg.push_str(&format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">
"#,
        width, height, width, height
    ));
    
    // Background
    svg.push_str(&format!(
        "  <rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"#1e1e1e\"/>\n",
        width, height
    ));
    
    // Export each object
    for (object, transform, style) in scene.iter_leaves() {
        // Build transform attribute
        let transform_attr = format!(
            "matrix({},{},{},{},{},{})",
            transform.a, transform.c, transform.b, transform.d, transform.tx, transform.ty
        );
        
        // Build style attributes
        let fill = style.fill_color.as_ref()
            .map(|c| c.clone())
            .unwrap_or_else(|| "none".to_string());
        let stroke = style.stroke_color.as_ref()
            .map(|c| c.clone())
            .unwrap_or_else(|| "none".to_string());
        let stroke_width = style.stroke_width;
        
        match object {
            VectorObject::Rectangle { x, y, width, height } => {
                svg.push_str(&format!(
                    r#"  <rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" transform="{}"/>
"#,
                    x, y, width, height, fill, stroke, stroke_width, transform_attr
                ));
            }
            VectorObject::Ellipse { cx, cy, rx, ry } => {
                svg.push_str(&format!(
                    r#"  <ellipse cx="{}" cy="{}" rx="{}" ry="{}" fill="{}" stroke="{}" stroke-width="{}" transform="{}"/>
"#,
                    cx, cy, rx, ry, fill, stroke, stroke_width, transform_attr
                ));
            }
            VectorObject::Path { commands: path_commands } => {
                let mut d = String::new();
                for cmd in path_commands {
                    match cmd {
                        crate::core::scene::PathCommand::MoveTo { x, y } => {
                            d.push_str(&format!("M{},{} ", x, y));
                        }
                        crate::core::scene::PathCommand::LineTo { x, y } => {
                            d.push_str(&format!("L{},{} ", x, y));
                        }
                        crate::core::scene::PathCommand::CurveTo { x1, y1, x2, y2, x, y } => {
                            d.push_str(&format!("C{},{} {},{} {},{} ", x1, y1, x2, y2, x, y));
                        }
                        crate::core::scene::PathCommand::ClosePath => {
                            d.push_str("Z ");
                        }
                    }
                }
                svg.push_str(&format!(
                    r#"  <path d="{}" fill="{}" stroke="{}" stroke-width="{}" transform="{}"/>
"#,
                    d.trim(), fill, stroke, stroke_width, transform_attr
                ));
            }
        }
    }
    
    // Close SVG
    svg.push_str("</svg>\n");
    
    svg
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::math::TransformMatrix;

    #[test]
    fn test_generate_rectangle_commands() {
        let mut scene = SceneGraph::new();
        let id = scene.generate_id();
        scene.add_object(
            id,
            VectorObject::Rectangle { x: 10.0, y: 20.0, width: 100.0, height: 50.0 },
            TransformMatrix::identity(),
        );

        let commands = generate_render_commands(&scene);
        assert!(!commands.is_empty());
        
        // Should contain SetTransform, BeginPath, Rect, Fill, Stroke, ResetTransform
        let has_rect = commands.iter().any(|cmd| matches!(cmd, RenderCommand::Rect { .. }));
        assert!(has_rect);
    }
}
