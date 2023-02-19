import React, {useState} from 'react';
import {Button, Divider, Stack, TextInput, Title} from "@mantine/core";

export default function CreateDocument() {
    const [name, setName] = useState("")
    return <Stack>
        <Title>
            Create Document
        </Title>
        <Divider />
        <TextInput label={"Name"} value={name} onChange={(event) => setName(event.currentTarget.value)}/>
        <Button>Create +</Button>
    </Stack>
}
