import React from 'react';
import {Button, Col, Divider, Grid, Stack, Title} from "@mantine/core";
import {useNavigate} from "react-router-dom";

export default function Home() {
    const navigate = useNavigate()
    return <Stack>
        <Title>
            Pinhead
        </Title>
        <Divider />
        <Title order={2}>
            Your Documents
        </Title>
        <Divider />
        <Button onClick={() => navigate("/documents/create")}>
            Add Document +
        </Button>
    </Stack>
}
