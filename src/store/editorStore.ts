/**
 * Editor Store - Zustand store for managing editor state
 */

import { create } from 'zustand';
import type { Editor } from '../wasm/wasmLoader';

export type Tool = 'select' | 'rectangle' | 'ellipse' | 'pen';

interface EditorState {
    // Wasm Editor
    editor: Editor | null;
    isWasmReady: boolean;

    // Tool State
    currentTool: Tool;

    // Selection
    selectedIds: string[];

    // Canvas
    canvasWidth: number;
    canvasHeight: number;

    // Render trigger (increment to force re-render)
    renderVersion: number;

    // Actions
    setEditor: (editor: Editor) => void;
    setWasmReady: (ready: boolean) => void;
    setCurrentTool: (tool: Tool) => void;
    setSelectedIds: (ids: string[]) => void;
    setCanvasSize: (width: number, height: number) => void;
    triggerRender: () => void;
}

export const useEditorStore = create<EditorState>((set) => ({
    // Initial State
    editor: null,
    isWasmReady: false,
    currentTool: 'select',
    selectedIds: [],
    canvasWidth: 800,
    canvasHeight: 600,
    renderVersion: 0,

    // Actions
    setEditor: (editor) => set({ editor }),
    setWasmReady: (ready) => set({ isWasmReady: ready }),
    setCurrentTool: (tool) => set({ currentTool: tool }),
    setSelectedIds: (ids) => set({ selectedIds: ids }),
    setCanvasSize: (width, height) => set({ canvasWidth: width, canvasHeight: height }),
    triggerRender: () => set((state) => ({ renderVersion: state.renderVersion + 1 })),
}));
