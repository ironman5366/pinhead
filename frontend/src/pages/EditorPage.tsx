import React, {useState} from 'react';
import {Grid, TextInput} from "@mantine/core";
import {JSONContent} from "@tiptap/react";
import Editor from "../components/organisms/Editor";
import {EMPTY_DOCUMENT} from "../constants";

export default function EditorPage() {
    const [editorContent, setEditorContent] = useState<JSONContent>(EMPTY_DOCUMENT);

    return <Grid>
        <Grid.Col md={3}>
            <TextInput placeholder="Title"/>
        </Grid.Col>
        <Grid.Col md={8}>
            {
                // Where the toolbar will go
            }
        </Grid.Col>
        <Grid.Col md={12}>
            {
                // The main editor
            }
            <Editor setEditorContent={setEditorContent} />
        </Grid.Col>
    </Grid>
}
