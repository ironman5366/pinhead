import React, {useState} from 'react';
import {EditorContent, useEditor} from "@tiptap/react";
import {StarterKit} from "@tiptap/starter-kit";
import MenuBar from "../molecules/MenuBar";
import {Loader} from "@mantine/core";

export interface EditorProps {
    initialContent: string
}

export default function Editor({ initialContent }: EditorProps) {
    const [content, setContent] = useState(initialContent);

    const editor = useEditor({
        extensions: [
            StarterKit
        ],
        content
    })

    if (!editor) {
        return <Loader />
    }

    return <div>
        <MenuBar editor={editor} />
        <EditorContent editor={editor} />
    </div>
}
