import React from 'react';
import {Editor} from "@tiptap/react";

export interface MenuBarProps {
    editor: Editor
}

export default function MenuBar({editor}: MenuBarProps) {
    return <>
        <button
            onClick={() => editor.chain().focus().toggleBold().run()}
            disabled={
                !editor.can()
                    .chain()
                    .focus()
                    .toggleBold()
                    .run()
            }
            className={editor.isActive('bold') ? 'is-active' : ''}
        >
            bold
        </button>
    </>
}
