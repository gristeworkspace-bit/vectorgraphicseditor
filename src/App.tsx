/**
 * App Component - Main application layout
 */

import { useEffect, useState } from 'react';
import { Canvas } from './components/Canvas';
import { Toolbar } from './components/Toolbar';
import { PropertiesPanel } from './components/PropertiesPanel';
import { initWasm, createEditor, hasDemoShapes, setDemoShapesAdded } from './wasm/wasmLoader';
import { useEditorStore } from './store/editorStore';

export default function App() {
    const { setEditor, setWasmReady } = useEditorStore();
    const [error, setError] = useState<string | null>(null);

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
                <span style={{ color: 'var(--color-text-muted)', fontSize: '12px' }}>
                    Phase 1: Foundation
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
