/**
 * Wasm Loader - Handles loading and initialization of the Rust Wasm module
 */

import init, { Editor } from './pkg/rust_core';

let wasmInitialized = false;
let editorInstance: Editor | null = null;
let demoShapesAdded = false;

/**
 * Initialize the Wasm module
 */
export async function initWasm(): Promise<void> {
    if (wasmInitialized) return;

    await init();
    wasmInitialized = true;
    console.log('✅ Wasm module initialized');
}

/**
 * Create a new Editor instance
 */
export function createEditor(): Editor {
    if (!wasmInitialized) {
        throw new Error('Wasm module not initialized. Call initWasm() first.');
    }

    if (!editorInstance) {
        editorInstance = new Editor();
        console.log('✅ Editor instance created');
    }

    return editorInstance;
}

/**
 * Get the current Editor instance
 */
export function getEditor(): Editor | null {
    return editorInstance;
}

/**
 * Check if demo shapes have been added
 */
export function hasDemoShapes(): boolean {
    return demoShapesAdded;
}

/**
 * Mark demo shapes as added
 */
export function setDemoShapesAdded(): void {
    demoShapesAdded = true;
}

/**
 * Reset the editor instance
 */
export function resetEditor(): void {
    if (editorInstance) {
        editorInstance.free();
        editorInstance = null;
    }
    demoShapesAdded = false;
}

export type { Editor };
