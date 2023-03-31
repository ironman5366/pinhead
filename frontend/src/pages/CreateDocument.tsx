import React, {useState} from 'react';
import {Button, Divider, Stack, TextInput, Title} from "@mantine/core";
import {JSONContent} from "@tiptap/react";
import Editor from "../components/organisms/Editor";
import useCreateDocument from "../hooks/useCreateDocument";
import {EMPTY_DOCUMENT} from "../constants";

export default function CreateDocument() {
    const [name, setName] = useState("")
    const [content, setContent] = useState<JSONContent>();
    const { isLoading, mutate, data } = useCreateDocument();

    return <Stack>
        <Title>
            Create Document
        </Title>
        <Divider />
        <TextInput label="Name" value={name} onChange={(event) => setName(event.currentTarget.value)}/>
        <Editor setEditorContent={setContent}/>
        <Button onClick={() => {
            mutate({
                title: name,
                content: content || EMPTY_DOCUMENT
            })
        }}>Create +</Button>
    </Stack>
}
