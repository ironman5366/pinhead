import React from 'react';
import {Button, Divider, Stack, Title} from "@mantine/core";
import useContentFields from "../../hooks/useContentFields";

export default function ContentFieldList() {
    const { data } = useContentFields();

    return <Stack>
        <Title>
            Available Fields
        </Title>
        {data && data.results.map((d) => <div>
            {d.name}
            <Divider />
        </div>)}
        <Button>
            + New Field
        </Button>
    </Stack>
}
