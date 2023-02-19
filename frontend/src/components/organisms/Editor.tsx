import React from 'react';
import {EditorContent, useEditor} from "@tiptap/react";
import {StarterKit} from "@tiptap/starter-kit";

export default function Editor() {
    const editor = useEditor({
        extensions: [
            StarterKit
        ],
        content: ''
    })

    return <EditorContent editor={editor} />
}
