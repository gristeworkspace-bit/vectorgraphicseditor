/**
 * DirectSelectOverlay - Displays path anchor points as SVG rects
 * 
 * Phase 8: Direct Selection Tool
 * - Step 2: Read-only vertex display ✅
 * - Step 3: Drag to update vertices ✅
 */

import { useEffect, useState, useRef, useCallback } from 'react';
import { useEditorStore } from '../store/editorStore';

interface PathPoint {
    x: number;
    y: number;
    type: 'move' | 'line' | 'curve';
}

export function DirectSelectOverlay() {
    const { editor, currentTool, selectedIds, renderVersion, triggerRender } = useEditorStore();
    const [points, setPoints] = useState<PathPoint[]>([]);
    const [draggingIndex, setDraggingIndex] = useState<number | null>(null);
    const svgRef = useRef<SVGSVGElement>(null);

    // Get the first selected ID
    const selectedId = selectedIds.length > 0 ? selectedIds[0] : null;

    // Fetch path points when selection changes or tool changes
    useEffect(() => {
        // Only active for direct_select tool
        if (!editor || currentTool !== 'direct_select') {
            setPoints([]);
            return;
        }

        if (!selectedId) {
            setPoints([]);
            return;
        }

        try {
            // Unconditionally call get_path_points
            // If not a path, it returns "[]"
            const pointsJson = editor.get_path_points(selectedId);
            const parsedPoints: PathPoint[] = JSON.parse(pointsJson);

            console.log('DirectSelectOverlay: Got', parsedPoints.length, 'points for', selectedId);
            setPoints(parsedPoints);
        } catch (err) {
            console.warn('DirectSelectOverlay: Error fetching points', err);
            setPoints([]);
        }
    }, [editor, currentTool, selectedId, renderVersion]);

    // Handle pointer down on a vertex
    const handlePointerDown = useCallback((e: React.PointerEvent, index: number) => {
        e.preventDefault();
        e.stopPropagation();

        if (!editor || !selectedId) return;

        // Save snapshot for undo before starting drag
        editor.save_snapshot();

        setDraggingIndex(index);

        // Capture pointer for smooth dragging
        (e.target as Element).setPointerCapture(e.pointerId);
    }, [editor, selectedId]);

    // Handle pointer move during drag
    const handlePointerMove = useCallback((e: React.PointerEvent, index: number) => {
        if (draggingIndex !== index || !editor || !selectedId || !svgRef.current) return;

        // Get SVG coordinates
        const svg = svgRef.current;
        const rect = svg.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;

        // Update the path point in Rust
        editor.update_path_point(selectedId, index, x, y);

        // Update local state for immediate visual feedback
        setPoints(prev => prev.map((p, i) =>
            i === index ? { ...p, x, y } : p
        ));

        // Trigger canvas re-render
        triggerRender();
    }, [draggingIndex, editor, selectedId, triggerRender]);

    // Handle pointer up - end drag
    const handlePointerUp = useCallback((e: React.PointerEvent, index: number) => {
        if (draggingIndex !== index) return;

        (e.target as Element).releasePointerCapture(e.pointerId);
        setDraggingIndex(null);

        // Final render
        triggerRender();
    }, [draggingIndex, triggerRender]);

    // Don't render if no points or not in direct_select mode
    if (points.length === 0 || currentTool !== 'direct_select') {
        return null;
    }

    const ANCHOR_SIZE = 8;
    const HALF_SIZE = ANCHOR_SIZE / 2;

    return (
        <svg
            ref={svgRef}
            style={{
                position: 'absolute',
                top: 0,
                left: 0,
                width: '100%',
                height: '100%',
                pointerEvents: 'none', // SVG container doesn't capture events
                zIndex: 10
            }}
        >
            {points.map((point, index) => (
                <rect
                    key={index}
                    x={point.x - HALF_SIZE}
                    y={point.y - HALF_SIZE}
                    width={ANCHOR_SIZE}
                    height={ANCHOR_SIZE}
                    fill={draggingIndex === index ? '#3b82f6' : 'white'}
                    stroke="#3b82f6"
                    strokeWidth={2}
                    style={{
                        pointerEvents: 'auto', // Individual rects capture clicks
                        cursor: 'move'
                    }}
                    onPointerDown={(e) => handlePointerDown(e, index)}
                    onPointerMove={(e) => handlePointerMove(e, index)}
                    onPointerUp={(e) => handlePointerUp(e, index)}
                />
            ))}
        </svg>
    );
}
