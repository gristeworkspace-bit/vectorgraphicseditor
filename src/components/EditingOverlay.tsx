import React, { useEffect, useRef, useState } from 'react';

interface EditingOverlayProps {
    x: number;
    y: number;
    fontSize: number;
    letterSpacing: number;
    color: string;
    content: string;
    onCommit: (newContent: string) => void;
    onCancel: () => void;
}

export const EditingOverlay: React.FC<EditingOverlayProps> = ({
    x,
    y,
    fontSize,
    letterSpacing,
    color,
    content,
    onCommit,
    onCancel,
}) => {
    const [value, setValue] = useState(content);
    const textareaRef = useRef<HTMLTextAreaElement>(null);

    useEffect(() => {
        if (textareaRef.current) {
            textareaRef.current.focus();
            // Move cursor to end
            textareaRef.current.setSelectionRange(value.length, value.length);
        }
    }, []);

    // Adjust height automatically
    useEffect(() => {
        if (textareaRef.current) {
            textareaRef.current.style.height = 'auto';
            textareaRef.current.style.height = textareaRef.current.scrollHeight + 'px';
        }
    }, [value]);

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
            e.preventDefault();
            onCommit(value);
        } else if (e.key === 'Escape') {
            e.preventDefault();
            onCancel();
        }
    };

    const handleBlur = () => {
        onCommit(value);
    };

    return (
        <textarea
            ref={textareaRef}
            value={value}
            onChange={(e) => setValue(e.target.value)}
            onKeyDown={handleKeyDown}
            onBlur={handleBlur}
            style={{
                position: 'absolute',
                left: `${x}px`,
                top: `${y}px`,
                // Adjust for baseline vs top-left. 
                // Canvas text is usually baseline or alphabetic. 
                // We might need to adjust y based on font metrics.
                // Assuming standard line-height approx 1.2 or similar.
                // For now, let's align top-left roughly.
                // Note: fillText(x, y) draws at baseline y.
                // textarea draws from top-left.
                // We need to shift y up by roughly fontSize (ascent).
                transform: `translateY(-${fontSize * 0.8}px)`, // Approximate baseline offset
                fontSize: `${fontSize}px`,
                fontFamily: 'sans-serif',
                letterSpacing: `${letterSpacing}px`,
                color: color,
                background: 'transparent',
                border: 'none',
                outline: 'none',
                padding: 0,
                margin: 0,
                resize: 'none',
                overflow: 'hidden',
                whiteSpace: 'pre',
                lineHeight: '1',
                zIndex: 1000,
                boxShadow: 'none',
            }}
        />
    );
};
