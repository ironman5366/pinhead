import React from 'react';
import {Button, Card, Group, Stack, Title} from "@mantine/core";
import {useTranslation} from "react-i18next";
import useContentTypes from "../../hooks/useContentTypes";

export default function ContentTypeList() {
    const { data, isLoading } = useContentTypes();
    const { t } = useTranslation();

    return <Stack>
        <Title>
            {t("contentType.list.title")}
        </Title>
        {data && data.results.map((d) => <Card key={d.id}>
            <Card.Section>
                <Title order={2}>
                    {d.name}
                </Title>
            </Card.Section>
        </Card>)}
        <Group position="left">
            <Button size="md">
                {t("contentType.list.newType")}
            </Button>
        </Group>
    </Stack>
}
