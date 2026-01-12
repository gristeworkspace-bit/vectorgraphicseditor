/**
 * Toolbar Component - Tool selection buttons
 */

import { useEditorStore, type Tool } from '../store/editorStore';

interface ToolButtonProps {
    tool: Tool;
    label: string;
    icon: string;
}

function ToolButton({ tool, label, icon }: ToolButtonProps) {
    const { currentTool, setCurrentTool } = useEditorStore();
    const isActive = currentTool === tool;

    return (
        <button
            className={`tool-btn btn ${isActive ? 'active' : 'btn--ghost'}`}
            onClick={() => setCurrentTool(tool)}
            title={label}
        >
            {icon}
        </button>
    );
}

export function Toolbar() {
    const { editor } = useEditorStore();
    const objectCount = editor?.object_count() ?? 0;

    return (
        <div className="app__sidebar">
            <h3 style={{ marginBottom: 'var(--spacing-md)', color: 'var(--color-text-secondary)' }}>
                Tools
            </h3>

            <div style={{ display: 'flex', flexWrap: 'wrap', gap: 'var(--spacing-sm)' }}>
                <ToolButton tool="select" label="Select (V)" icon="↖" />
                <ToolButton tool="rectangle" label="Rectangle (R)" icon="▢" />
                <ToolButton tool="ellipse" label="Ellipse (O)" icon="○" />
                <ToolButton tool="pen" label="Pen (P)" icon="✎" />
            </div>

            <div style={{
                marginTop: 'var(--spacing-lg)',
                padding: 'var(--spacing-md)',
                backgroundColor: 'var(--color-surface)',
                borderRadius: 'var(--radius-md)'
            }}>
                <h4 style={{ marginBottom: 'var(--spacing-sm)', color: 'var(--color-text-secondary)' }}>
                    Scene Info
                </h4>
                <p style={{ color: 'var(--color-text-muted)' }}>
                    Objects: {objectCount}
                </p>
            </div>

            <div style={{
                marginTop: 'var(--spacing-lg)',
                padding: 'var(--spacing-md)',
                backgroundColor: 'var(--color-accent-light)',
                borderRadius: 'var(--radius-md)',
                border: '1px solid var(--color-accent)'
            }}>
                <h4 style={{ marginBottom: 'var(--spacing-sm)', color: 'var(--color-accent)' }}>
                    Quick Start
                </h4>
                <p style={{ fontSize: '12px', color: 'var(--color-text-secondary)' }}>
                    Select Rectangle or Ellipse tool, then click on canvas to add shapes.
                </p>
            </div>
        </div>
    );
}
