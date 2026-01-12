/**
 * Properties Panel - Style editing for selected objects
 */

import { useEffect, useState, useCallback } from 'react';
import { useEditorStore } from '../store/editorStore';

interface StyleData {
    fill: string | null;
    stroke: string | null;
    strokeWidth: number;
}

export function PropertiesPanel() {
    const { editor, isWasmReady, triggerRender } = useEditorStore();
    const [style, setStyle] = useState<StyleData | null>(null);
    const [hasSelection, setHasSelection] = useState(false);

    // Poll for selection changes and update style
    const updateSelection = useCallback(() => {
        if (!editor || !isWasmReady) return;

        const selected = editor.has_selection();
        setHasSelection(selected);

        if (selected) {
            try {
                const styleJson = editor.get_selected_style();
                const parsed = JSON.parse(styleJson);
                setStyle({
                    fill: parsed.fill || '#3b82f6',
                    stroke: parsed.stroke || '#1e40af',
                    strokeWidth: parsed.strokeWidth || 2,
                });
            } catch {
                setStyle(null);
            }
        } else {
            setStyle(null);
        }
    }, [editor, isWasmReady]);

    // Update on mount and periodically
    useEffect(() => {
        updateSelection();
        const interval = setInterval(updateSelection, 100);
        return () => clearInterval(interval);
    }, [updateSelection]);

    // Handle fill color change (supports both onChange and onInput for real-time updates)
    const handleFillChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (!editor || !style) return;
        const newFill = e.target.value;
        editor.update_style(newFill, style.stroke || 'none', style.strokeWidth);
        setStyle({ ...style, fill: newFill });
        triggerRender(); // Force canvas re-render
    };

    // Handle stroke color change
    const handleStrokeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (!editor || !style) return;
        const newStroke = e.target.value;
        editor.update_style(style.fill || 'none', newStroke, style.strokeWidth);
        setStyle({ ...style, stroke: newStroke });
        triggerRender(); // Force canvas re-render
    };

    // Handle stroke width change
    const handleStrokeWidthChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (!editor || !style) return;
        const newWidth = parseFloat(e.target.value) || 1;
        editor.update_style(style.fill || 'none', style.stroke || 'none', newWidth);
        setStyle({ ...style, strokeWidth: newWidth });
        triggerRender(); // Force canvas re-render
    };

    // Handle bring to front (layer ordering)
    const handleBringToFront = () => {
        if (!editor) return;
        editor.bring_to_front();
        triggerRender(); // Force canvas re-render
    };

    // Handle send to back (layer ordering)
    const handleSendToBack = () => {
        if (!editor) return;
        editor.send_to_back();
        triggerRender(); // Force canvas re-render
    };

    return (
        <aside className="properties-panel">
            <h2 className="properties-panel__title">Properties</h2>

            {!hasSelection ? (
                <div className="properties-panel__empty">
                    No Selection
                </div>
            ) : (
                <div className="properties-panel__content">
                    {/* Fill Color */}
                    <div className="properties-panel__field">
                        <label>Fill</label>
                        <input
                            type="color"
                            value={style?.fill || '#3b82f6'}
                            onChange={handleFillChange}
                            onInput={handleFillChange as React.FormEventHandler<HTMLInputElement>}
                        />
                    </div>

                    {/* Stroke Color */}
                    <div className="properties-panel__field">
                        <label>Stroke</label>
                        <input
                            type="color"
                            value={style?.stroke || '#1e40af'}
                            onChange={handleStrokeChange}
                            onInput={handleStrokeChange as React.FormEventHandler<HTMLInputElement>}
                        />
                    </div>

                    {/* Stroke Width */}
                    <div className="properties-panel__field">
                        <label>Stroke Width</label>
                        <input
                            type="number"
                            min="0"
                            max="20"
                            step="0.5"
                            value={style?.strokeWidth || 2}
                            onChange={handleStrokeWidthChange}
                        />
                    </div>

                    {/* Layer Ordering */}
                    <div className="properties-panel__section">
                        <h3 className="properties-panel__section-title">Layer Order</h3>
                        <div className="properties-panel__buttons">
                            <button
                                className="properties-panel__button"
                                onClick={handleBringToFront}
                                title="Bring to Front"
                            >
                                ⬆️ Front
                            </button>
                            <button
                                className="properties-panel__button"
                                onClick={handleSendToBack}
                                title="Send to Back"
                            >
                                ⬇️ Back
                            </button>
                        </div>
                    </div>
                </div>
            )}
        </aside>
    );
}
