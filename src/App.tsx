/**
 * App Component - Main application layout
 */

import { useEffect, useState, useRef, useCallback } from 'react';
import { Canvas } from './components/Canvas';
import { Toolbar } from './components/Toolbar';
import { PropertiesPanel } from './components/PropertiesPanel';
import { initWasm, createEditor, hasDemoShapes, setDemoShapesAdded } from './wasm/wasmLoader';
import { useEditorStore } from './store/editorStore';

export default function App() {
    const { editor, setEditor, setWasmReady, isWasmReady, triggerRender, canvasWidth, canvasHeight } = useEditorStore();
    const [error, setError] = useState<string | null>(null);
    const [canUndo, setCanUndo] = useState(false);
    const [canRedo, setCanRedo] = useState(false);
    const fileInputRef = useRef<HTMLInputElement>(null);

    // Update undo/redo button states
    const updateHistoryState = useCallback(() => {
        if (editor && isWasmReady) {
            setCanUndo(editor.can_undo());
            setCanRedo(editor.can_redo());
        }
    }, [editor, isWasmReady]);

    useEffect(() => {
        async function initializeEditor() {
            try {
                await initWasm();
                const editor = createEditor();
                setEditor(editor);
                setWasmReady(true);

                // Add demo shapes only once (React StrictMode calls useEffect twice)
                if (!hasDemoShapes()) {
                    // Test Shape A: Normal rectangle (blue) - no rotation
                    editor.add_rectangle(150, 150, 150, 100);

                    // Test Shape B: 45-degree ROTATED rectangle (green center)
                    // This is the critical test case for hit detection!
                    editor.add_rotated_rectangle(450, 250, 150, 100, 45);

                    // Test Shape C: Heart-shaped path using Bezier curves
                    editor.add_heart_path(300, 450, 80);

                    setDemoShapesAdded();
                    console.log('✅ Editor ready with test shapes:');
                    console.log('   - Shape A: Normal rectangle at (150, 150)');
                    console.log('   - Shape B: 45° rotated rectangle centered at (450, 250)');
                    console.log('   - Shape C: Heart path centered at (300, 450)');
                } else {
                    console.log('✅ Editor ready (demo shapes already exist)');
                }
            } catch (err) {
                console.error('Failed to initialize Wasm:', err);
                setError(err instanceof Error ? err.message : 'Unknown error');
            }
        }

        initializeEditor();
    }, [setEditor, setWasmReady]);

    // Poll for history state changes
    useEffect(() => {
        const interval = setInterval(updateHistoryState, 200);
        return () => clearInterval(interval);
    }, [updateHistoryState]);

    // Handle Undo
    const handleUndo = useCallback(() => {
        if (!editor || !isWasmReady) return;
        if (editor.undo()) {
            triggerRender();
            updateHistoryState();
            console.log('↩️ Undo performed');
        }
    }, [editor, isWasmReady, triggerRender, updateHistoryState]);

    // Handle Redo
    const handleRedo = useCallback(() => {
        if (!editor || !isWasmReady) return;
        if (editor.redo()) {
            triggerRender();
            updateHistoryState();
            console.log('↪️ Redo performed');
        }
    }, [editor, isWasmReady, triggerRender, updateHistoryState]);

    // Keyboard shortcuts for Undo/Redo
    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            // Check for Ctrl/Cmd key
            const isCtrlOrCmd = e.ctrlKey || e.metaKey;

            if (isCtrlOrCmd && e.key === 'z') {
                if (e.shiftKey) {
                    // Ctrl/Cmd + Shift + Z = Redo
                    e.preventDefault();
                    handleRedo();
                } else {
                    // Ctrl/Cmd + Z = Undo
                    e.preventDefault();
                    handleUndo();
                }
            } else if (isCtrlOrCmd && e.key === 'y') {
                // Ctrl/Cmd + Y = Redo (Windows style)
                e.preventDefault();
                handleRedo();
            }
        };

        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    }, [handleUndo, handleRedo]);

    // Handle Save button click - download JSON file
    const handleSave = () => {
        if (!editor || !isWasmReady) return;

        try {
            const json = editor.export_scene_to_json();

            // Blob作成
            const blob = new Blob([json], { type: 'application/json' });
            const url = URL.createObjectURL(blob);

            // ダウンロード発火（window.openは絶対禁止！）
            const a = document.createElement('a');
            a.href = url;
            a.download = 'drawing.json'; // ★ここでファイル名を強制
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);

            // 後始末
            URL.revokeObjectURL(url);

            console.log('✅ Scene saved to drawing.json');
        } catch (err) {
            console.error('Failed to save scene:', err);
            alert('Failed to save scene. See console for details.');
        }
    };

    // Handle Load button click - open file dialog
    const handleLoadClick = () => {
        fileInputRef.current?.click();
    };

    // Handle file selection
    const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const file = e.target.files?.[0];
        if (!file || !editor || !isWasmReady) return;

        const reader = new FileReader();
        reader.onload = (event) => {
            const json = event.target?.result as string;
            if (json) {
                const success = editor.import_scene_from_json(json);
                if (success) {
                    triggerRender();
                    updateHistoryState();
                    console.log('✅ Scene loaded from file');
                } else {
                    console.error('Failed to parse scene JSON');
                    alert('Failed to load scene. The file may be corrupted or invalid.');
                }
            }
        };
        reader.onerror = () => {
            console.error('Failed to read file');
            alert('Failed to read the selected file.');
        };
        reader.readAsText(file);

        // Reset file input so the same file can be selected again
        e.target.value = '';
    };

    // Handle Export SVG
    const handleExportSVG = () => {
        if (!editor || !isWasmReady) return;

        try {
            const svgData = editor.export_to_svg(canvasWidth, canvasHeight);

            // Blob作成
            const blob = new Blob([svgData], { type: 'image/svg+xml' });
            const url = URL.createObjectURL(blob);

            // ダウンロード発火（window.openは絶対禁止！）
            const a = document.createElement('a');
            a.href = url;
            a.download = 'drawing.svg'; // ★ここでファイル名を強制
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);

            // 後始末
            URL.revokeObjectURL(url);

            console.log('✅ Exported to drawing.svg');
        } catch (err) {
            console.error('Failed to export SVG:', err);
            alert('Failed to export SVG. See console for details.');
        }
    };

    // Handle Export PNG
    const handleExportPNG = () => {
        // Find the canvas element
        const canvas = document.querySelector('canvas');
        if (!canvas) {
            alert('Canvas not found');
            return;
        }

        try {
            canvas.toBlob((blob) => {
                if (!blob) {
                    alert('Failed to create PNG image');
                    return;
                }

                // Blob URLの作成
                const url = URL.createObjectURL(blob);

                // ダウンロード発火（window.openは絶対禁止！）
                const a = document.createElement('a');
                a.href = url;
                a.download = 'drawing.png'; // ★ここでファイル名を強制
                document.body.appendChild(a);
                a.click();
                document.body.removeChild(a);

                // 後始末
                URL.revokeObjectURL(url);

                console.log('✅ Exported to drawing.png');
            }, 'image/png');
        } catch (err) {
            console.error('Failed to export PNG:', err);
            alert('Failed to export PNG. See console for details.');
        }
    };

    if (error) {
        return (
            <div className="app">
                <div className="loading" style={{ color: 'var(--color-danger)' }}>
                    <span>Failed to load Wasm module</span>
                    <span style={{ fontSize: '12px', color: 'var(--color-text-muted)' }}>{error}</span>
                </div>
            </div>
        );
    }

    return (
        <div className="app">
            <header className="app__header">
                <h1 className="app__title">Vector Graphics Editor</h1>

                <div className="app__header-actions">
                    <button
                        className="btn btn--secondary"
                        onClick={handleUndo}
                        disabled={!isWasmReady || !canUndo}
                        title="Undo (Ctrl+Z)"
                    >
                        ↩️ Undo
                    </button>
                    <button
                        className="btn btn--secondary"
                        onClick={handleRedo}
                        disabled={!isWasmReady || !canRedo}
                        title="Redo (Ctrl+Y / Ctrl+Shift+Z)"
                    >
                        ↪️ Redo
                    </button>

                    <div className="app__header-divider" />

                    <button
                        className="btn btn--secondary"
                        onClick={handleSave}
                        disabled={!isWasmReady}
                        title="Save drawing as JSON"
                    >
                        💾 Save
                    </button>
                    <button
                        className="btn btn--secondary"
                        onClick={handleLoadClick}
                        disabled={!isWasmReady}
                        title="Load drawing from JSON"
                    >
                        📂 Load
                    </button>
                    <input
                        ref={fileInputRef}
                        type="file"
                        accept=".json,application/json"
                        style={{ display: 'none' }}
                        onChange={handleFileChange}
                    />

                    <div className="app__header-divider" />

                    <button
                        className="btn btn--primary"
                        onClick={handleExportSVG}
                        disabled={!isWasmReady}
                        title="Export as SVG file"
                    >
                        📄 SVG
                    </button>
                    <button
                        className="btn btn--primary"
                        onClick={handleExportPNG}
                        disabled={!isWasmReady}
                        title="Export as PNG image"
                    >
                        🖼️ PNG
                    </button>
                </div>

                <span style={{ color: 'var(--color-text-muted)', fontSize: '12px' }}>
                    Phase 7: Export
                </span>
            </header>

            <main className="app__main">
                <Toolbar />
                <Canvas />
                <PropertiesPanel />
            </main>
        </div>
    );
}
