import React, {useState} from 'react';
import {Button, Divider, Stack, TextInput, Title} from "@mantine/core";
import {JSONContent} from "@tiptap/react";
import Editor from "../components/organisms/Editor";

export default function CreateDocument() {
    const [name, setName] = useState("")
    const [content, setContent] = useState<JSONContent>();

    return <Stack>
        <Title>
            Create Document
        </Title>
        <Divider />
        <TextInput label="Name" value={name} onChange={(event) => setName(event.currentTarget.value)}/>
        <Editor setEditorContent={setContent}/>
        <Button>Create +</Button>
    </Stack>
}
