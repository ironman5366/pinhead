import React, {useState} from 'react';
import {Button, Divider, Stack, TextInput, Title} from "@mantine/core";
import Editor from "../components/organisms/Editor";

export default function CreateDocument() {
    const [name, setName] = useState("")
    return <Stack>
        <Title>
            Create Document
        </Title>
        <Divider />
        <TextInput label={"Name"} value={name} onChange={(event) => setName(event.currentTarget.value)}/>
        <Editor />
        <Button>Create +</Button>
    </Stack>
}
