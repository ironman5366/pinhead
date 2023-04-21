import React from 'react';
import {Button, Card, Stack, Title} from "@mantine/core";
import useContentTypes from "../../hooks/useContentTypes";

export default function ContentTypeList() {
    const { data, isLoading } = useContentTypes();

    return <Stack>
        <Title>
            Content Types
        </Title>
        {data && data.results.map((d) => <Card key={d.id}>
            <Card.Section>
                <Title order={2}>
                    {d.name}
                </Title>
            </Card.Section>
        </Card>)}
        <Button>
            + New Content Type
        </Button>
    </Stack>
}
