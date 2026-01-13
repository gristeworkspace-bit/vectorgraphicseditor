/**
 * Canvas Component - Renders the editor canvas and handles input
 */

import { useEffect, useRef, useCallback, useState } from 'react';
import { useEditorStore } from '../store/editorStore';
import { DirectSelectOverlay } from './DirectSelectOverlay';

// Type definitions for render commands from Rust
interface RenderCommand {
    type: string;
    [key: string]: unknown;
}

// Selection overlay from Rust
interface SelectionOverlay {
    id: string;
    corners: [[number, number], [number, number], [number, number], [number, number]];
}

export function Canvas() {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const containerRef = useRef<HTMLDivElement>(null);

    const {
        editor,
        isWasmReady,
        canvasWidth,
        canvasHeight,
        setCanvasSize,
        currentTool,
        setSelectedIds,
        renderVersion,
        triggerRender: storeRender
    } = useEditorStore();

    // Drag state
    const [isDragging, setIsDragging] = useState(false);
    const [dragMode, setDragMode] = useState<'move' | 'resize' | 'rotate' | null>(null);
    const [, setRenderTrigger] = useState(0);

    // Handle hit detection radii
    const HANDLE_HIT_RADIUS = 12; // Inner radius for resize
    const ROTATION_OUTER_RADIUS = 25; // Outer radius for rotation zone

    // Force re-render
    const triggerRender = useCallback(() => {
        setRenderTrigger((n) => n + 1);
    }, []);

    // Execute render commands on canvas
    const executeRenderCommands = useCallback((ctx: CanvasRenderingContext2D, commands: RenderCommand[]) => {
        for (const cmd of commands) {
            switch (cmd.type) {
                case 'SetTransform':
                    ctx.setTransform(
                        cmd.a as number,
                        cmd.b as number,
                        cmd.c as number,
                        cmd.d as number,
                        cmd.e as number,
                        cmd.f as number
                    );
                    break;
                case 'ResetTransform':
                    ctx.resetTransform();
                    break;
                case 'BeginPath':
                    ctx.beginPath();
                    break;
                case 'Rect':
                    ctx.rect(
                        cmd.x as number,
                        cmd.y as number,
                        cmd.width as number,
                        cmd.height as number
                    );
                    break;
                case 'Ellipse':
                    ctx.ellipse(
                        cmd.cx as number,
                        cmd.cy as number,
                        cmd.rx as number,
                        cmd.ry as number,
                        0,
                        0,
                        Math.PI * 2
                    );
                    break;
                case 'MoveTo':
                    ctx.moveTo(cmd.x as number, cmd.y as number);
                    break;
                case 'LineTo':
                    ctx.lineTo(cmd.x as number, cmd.y as number);
                    break;
                case 'BezierCurveTo':
                    ctx.bezierCurveTo(
                        cmd.cp1x as number,
                        cmd.cp1y as number,
                        cmd.cp2x as number,
                        cmd.cp2y as number,
                        cmd.x as number,
                        cmd.y as number
                    );
                    break;
                case 'ClosePath':
                    ctx.closePath();
                    break;
                case 'SetFillStyle':
                    ctx.fillStyle = cmd.color as string;
                    break;
                case 'SetStrokeStyle':
                    ctx.strokeStyle = cmd.color as string;
                    break;
                case 'SetLineWidth':
                    ctx.lineWidth = cmd.width as number;
                    break;
                case 'Fill':
                    ctx.fill();
                    break;
                case 'Stroke':
                    ctx.stroke();
                    break;
            }
        }
    }, []);

    // Draw selection overlay
    const drawSelectionOverlay = useCallback((ctx: CanvasRenderingContext2D, overlays: SelectionOverlay[]) => {
        ctx.resetTransform();

        for (const overlay of overlays) {
            const corners = overlay.corners;

            // Draw bounding box
            ctx.strokeStyle = '#3b82f6';
            ctx.lineWidth = 2;
            ctx.setLineDash([5, 5]);
            ctx.beginPath();
            ctx.moveTo(corners[0][0], corners[0][1]);
            ctx.lineTo(corners[1][0], corners[1][1]);
            ctx.lineTo(corners[2][0], corners[2][1]);
            ctx.lineTo(corners[3][0], corners[3][1]);
            ctx.closePath();
            ctx.stroke();
            ctx.setLineDash([]);

            // Draw corner handles
            const handleSize = 8;
            ctx.fillStyle = '#ffffff';
            ctx.strokeStyle = '#3b82f6';
            ctx.lineWidth = 2;

            for (const corner of corners) {
                ctx.fillRect(
                    corner[0] - handleSize / 2,
                    corner[1] - handleSize / 2,
                    handleSize,
                    handleSize
                );
                ctx.strokeRect(
                    corner[0] - handleSize / 2,
                    corner[1] - handleSize / 2,
                    handleSize,
                    handleSize
                );
            }
        }
    }, []);

    // Render the scene
    const render = useCallback(() => {
        const canvas = canvasRef.current;
        const ctx = canvas?.getContext('2d');
        if (!canvas || !ctx || !editor) return;

        // Clear canvas
        ctx.resetTransform();
        ctx.fillStyle = '#1e1e1e';
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        // Draw grid
        ctx.strokeStyle = 'rgba(255, 255, 255, 0.05)';
        ctx.lineWidth = 1;
        const gridSize = 20;
        for (let x = 0; x <= canvas.width; x += gridSize) {
            ctx.beginPath();
            ctx.moveTo(x, 0);
            ctx.lineTo(x, canvas.height);
            ctx.stroke();
        }
        for (let y = 0; y <= canvas.height; y += gridSize) {
            ctx.beginPath();
            ctx.moveTo(0, y);
            ctx.lineTo(canvas.width, y);
            ctx.stroke();
        }

        // Get render commands from Rust
        const commandsJson = editor.get_render_commands();
        const commands: RenderCommand[] = JSON.parse(commandsJson);

        // Execute render commands
        executeRenderCommands(ctx, commands);

        // Draw selection overlay
        const overlayJson = editor.get_selection_overlay();
        const overlays: SelectionOverlay[] = JSON.parse(overlayJson);
        if (overlays.length > 0) {
            drawSelectionOverlay(ctx, overlays);
        }

        // Draw pen tool preview (path being constructed)
        if (editor.is_pen_drawing()) {
            const previewJson = editor.get_pen_preview();
            try {
                const preview = JSON.parse(previewJson);

                if (preview.commands && preview.commands.length > 0) {
                    ctx.save();

                    // Draw existing path commands (cyan dashed)
                    ctx.strokeStyle = '#00d4ff';
                    ctx.lineWidth = 2;
                    ctx.setLineDash([5, 5]);
                    ctx.beginPath();

                    for (const cmd of preview.commands as RenderCommand[]) {
                        if (cmd.type === 'MoveTo') {
                            ctx.moveTo(cmd.x as number, cmd.y as number);
                        } else if (cmd.type === 'LineTo') {
                            ctx.lineTo(cmd.x as number, cmd.y as number);
                        } else if (cmd.type === 'CurveTo') {
                            ctx.bezierCurveTo(
                                cmd.x1 as number, cmd.y1 as number,
                                cmd.x2 as number, cmd.y2 as number,
                                cmd.x as number, cmd.y as number
                            );
                        }
                    }
                    ctx.stroke();

                    // Draw preview curve as SEPARATE path (solid cyan)
                    if (preview.preview_curve && preview.last_anchor) {
                        const pc = preview.preview_curve;
                        const [startX, startY] = preview.last_anchor;

                        ctx.beginPath();
                        ctx.strokeStyle = '#00d4ff';
                        ctx.lineWidth = 3;
                        ctx.setLineDash([]); // Solid line for preview curve
                        ctx.moveTo(startX, startY); // Explicit start point!
                        ctx.bezierCurveTo(
                            pc.x1 as number, pc.y1 as number,
                            pc.x2 as number, pc.y2 as number,
                            pc.x as number, pc.y as number
                        );
                        ctx.stroke();
                    }

                    // Draw anchor points
                    ctx.setLineDash([]);
                    ctx.fillStyle = '#00d4ff';
                    for (const cmd of preview.commands as RenderCommand[]) {
                        if (cmd.type === 'MoveTo' || cmd.type === 'LineTo') {
                            ctx.beginPath();
                            ctx.arc(cmd.x as number, cmd.y as number, 4, 0, Math.PI * 2);
                            ctx.fill();
                        } else if (cmd.type === 'CurveTo') {
                            ctx.beginPath();
                            ctx.arc(cmd.x as number, cmd.y as number, 4, 0, Math.PI * 2);
                            ctx.fill();
                        }
                    }

                    // Draw drag handle and curve preview if present
                    if (preview.handle && preview.is_dragging && preview.last_anchor) {
                        const [hx, hy] = preview.handle;
                        const [lax, lay] = preview.last_anchor;

                        // Draw preview curve from last anchor to handle position
                        // Control points: cp1 at last anchor, cp2 symmetric to handle around endpoint
                        ctx.strokeStyle = '#ff6b6b';
                        ctx.lineWidth = 2;
                        ctx.setLineDash([3, 3]);
                        ctx.beginPath();
                        ctx.moveTo(lax, lay);
                        // bezierCurveTo(cp1x, cp1y, cp2x, cp2y, endX, endY)
                        // cp1 = last anchor (straight start)
                        // cp2 = symmetric of handle around endpoint (hx, hy)
                        // endpoint = handle position (where curve goes to)
                        const cp2x = 2 * hx - lax;
                        const cp2y = 2 * hy - lay;
                        ctx.bezierCurveTo(lax, lay, cp2x, cp2y, hx, hy);
                        ctx.stroke();
                        ctx.setLineDash([]);

                        // Draw handle whisker line
                        ctx.strokeStyle = '#ff6b6b';
                        ctx.lineWidth = 1;
                        ctx.beginPath();
                        ctx.moveTo(lax, lay);
                        ctx.lineTo(hx, hy);
                        ctx.stroke();

                        // Draw handle point
                        ctx.fillStyle = '#ff6b6b';
                        ctx.beginPath();
                        ctx.arc(hx, hy, 5, 0, Math.PI * 2);
                        ctx.fill();

                        // Draw mirrored handle point
                        ctx.beginPath();
                        ctx.arc(cp2x, cp2y, 4, 0, Math.PI * 2);
                        ctx.strokeStyle = '#ff6b6b';
                        ctx.stroke();
                    }

                    ctx.restore();
                }
            } catch {
                // Ignore parse errors
            }
        }
    }, [editor, executeRenderCommands, drawSelectionOverlay]);

    // Handle resize
    useEffect(() => {
        const container = containerRef.current;
        if (!container) return;

        const resizeObserver = new ResizeObserver((entries) => {
            for (const entry of entries) {
                const { width, height } = entry.contentRect;
                setCanvasSize(Math.floor(width), Math.floor(height));
            }
        });

        resizeObserver.observe(container);
        return () => resizeObserver.disconnect();
    }, [setCanvasSize]);

    // Render when ready or when canvas size changes
    useEffect(() => {
        if (isWasmReady && editor) {
            render();
        }
    }, [isWasmReady, editor, render, canvasWidth, canvasHeight, renderVersion]);

    // Handle Enter key for pen tool - finish open path
    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            // Debug: log all key presses
            console.log('Key pressed:', e.key, 'currentTool:', currentTool);

            if (!editor || !isWasmReady) {
                console.log('  → Early return: no editor or wasm not ready');
                return;
            }

            // Enter key: finish open path when using pen tool
            if (e.key === 'Enter') {
                e.preventDefault();
                e.stopPropagation();

                const isDrawing = editor.is_pen_drawing();
                console.log('  → Enter pressed, currentTool:', currentTool, 'is_pen_drawing:', isDrawing);

                if (currentTool === 'pen' && isDrawing) {
                    console.log('  → Finishing open path...');
                    editor.save_snapshot(); // Save for undo
                    const pathId = editor.pen_finish();
                    console.log('  → Open path finished:', pathId);
                    render();
                    storeRender();
                }
            }
        };

        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    }, [editor, isWasmReady, currentTool, render, storeRender]);

    // Get canvas coordinates from mouse event
    const getCanvasCoords = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
        const canvas = canvasRef.current;
        if (!canvas) return { x: 0, y: 0 };
        const rect = canvas.getBoundingClientRect();
        return {
            x: e.clientX - rect.left,
            y: e.clientY - rect.top
        };
    }, []);

    // Handle mouse down
    const handleMouseDown = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
        if (!editor) return;
        const { x, y } = getCanvasCoords(e);

        if (currentTool === 'select') {
            // First, check if clicking on a handle area (if something is selected)
            if (editor.has_selection()) {
                const handlesJson = editor.get_handle_positions();
                try {
                    const handles: [number, number][] = JSON.parse(handlesJson);
                    if (handles.length === 4) {
                        // Check each handle for hit (resize or rotate zone)
                        for (let i = 0; i < 4; i++) {
                            const [hx, hy] = handles[i];
                            const distance = Math.sqrt((x - hx) ** 2 + (y - hy) ** 2);

                            if (distance <= HANDLE_HIT_RADIUS) {
                                // Inside handle - start resize drag
                                console.log('Resize handle:', i);
                                editor.save_snapshot(); // Save for undo
                                editor.begin_resize_drag(x, y, i);
                                setIsDragging(true);
                                setDragMode('resize');
                                render();
                                return;
                            } else if (distance <= ROTATION_OUTER_RADIUS) {
                                // Outside handle but in rotation zone
                                console.log('Rotation zone near handle:', i);
                                editor.save_snapshot(); // Save for undo
                                editor.begin_rotate_drag(x, y);
                                setIsDragging(true);
                                setDragMode('rotate');
                                render();
                                return;
                            }
                        }
                    }
                } catch {
                    // Invalid JSON, ignore
                }
            }

            // Try to select object at this point (body drag)
            const selectedId = editor.select_at(x, y);
            if (selectedId) {
                console.log('Selected:', selectedId);
                // Begin move drag operation
                editor.save_snapshot(); // Save for undo
                editor.begin_move_drag(x, y);
                setIsDragging(true);
                setDragMode('move');
            } else {
                console.log('Deselected all');
                setDragMode(null);
            }
            render();
        } else if (currentTool === 'rectangle') {
            editor.save_snapshot(); // Save for undo
            editor.add_rectangle(x - 50, y - 30, 100, 60);
            render();
        } else if (currentTool === 'ellipse') {
            editor.save_snapshot(); // Save for undo
            editor.add_ellipse(x, y, 50, 35);
            render();
        } else if (currentTool === 'pen') {
            // Pen tool: start or continue drawing
            const shouldClose = editor.pen_down(x, y);
            if (shouldClose) {
                // Close the path
                editor.save_snapshot(); // Save for undo before creating path
                const pathId = editor.pen_close();
                console.log('Path closed:', pathId);
                setIsDragging(false); // Path is done, no more dragging
            } else {
                // Not closing - start drag for potential curve
                setIsDragging(true);
            }
            render();
        } else if (currentTool === 'direct_select') {
            // Direct selection: use bounding box hit test (same as select tool)
            const selectedId = editor.select_at(x, y);
            if (selectedId) {
                setSelectedIds([selectedId]);
                console.log('Direct select:', selectedId);
            } else {
                setSelectedIds([]);
            }
            render();
        }

        triggerRender();
    }, [editor, currentTool, render, getCanvasCoords, triggerRender, HANDLE_HIT_RADIUS, ROTATION_OUTER_RADIUS]);

    // Handle mouse move
    const handleMouseMove = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
        if (!editor) return;

        const { x, y } = getCanvasCoords(e);

        // Pen tool: always update preview/drag handle (even when not isDragging state)
        if (currentTool === 'pen' && editor.is_pen_drawing()) {
            // Only call pen_move if we're in the middle of a drag (after mousedown, before mouseup)
            if (isDragging) {
                editor.pen_move(x, y);
                render();
            }
            // Note: We could add preview line here if needed, but Rust handles this via get_pen_preview
            return;
        }

        // Other tools: only process if dragging
        if (!isDragging) return;

        if (editor.has_selection()) {
            if (dragMode === 'resize') {
                editor.update_resize_drag(x, y);
            } else if (dragMode === 'rotate') {
                editor.update_rotate_drag(x, y);
            } else {
                editor.update_move_drag(x, y);
            }
            render();
        }
    }, [editor, isDragging, dragMode, currentTool, getCanvasCoords, render]);

    // Handle mouse up
    const handleMouseUp = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
        if (!editor) return;

        if (currentTool === 'pen' && isDragging) {
            const { x, y } = getCanvasCoords(e);
            editor.pen_up(x, y);
            render();
        } else if (isDragging) {
            editor.end_drag();
        }
        setIsDragging(false);
        setDragMode(null);
    }, [editor, isDragging, currentTool, getCanvasCoords, render]);

    // Determine cursor based on tool and hover state
    const getCursor = () => {
        if (isDragging) {
            if (dragMode === 'resize') return 'nwse-resize';
            if (dragMode === 'rotate') return 'crosshair';
            return 'grabbing';
        }
        if (currentTool === 'select') return 'default';
        return 'crosshair';
    };

    return (
        <div ref={containerRef} className="app__canvas-container">
            {!isWasmReady ? (
                <div className="loading">
                    <div className="loading__spinner" />
                    <span>Loading Wasm...</span>
                </div>
            ) : (
                <>
                    <canvas
                        ref={canvasRef}
                        width={canvasWidth}
                        height={canvasHeight}
                        onMouseDown={handleMouseDown}
                        onMouseMove={handleMouseMove}
                        onMouseUp={handleMouseUp}
                        onMouseLeave={handleMouseUp}
                        style={{
                            cursor: getCursor(),
                            display: 'block'
                        }}
                    />
                    <DirectSelectOverlay />
                </>
            )}
        </div>
    );
}
