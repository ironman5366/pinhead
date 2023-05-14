import React from 'react';
import {Button, Divider, Stack, Title} from "@mantine/core";
import {useNavigate} from "react-router-dom";
import {useTranslation} from "react-i18next";
import useContentFields from "../../hooks/useContentFields";

export default function ContentFieldList() {
    const { data } = useContentFields();
    const navigate = useNavigate();
    const { t } = useTranslation();

    return <Stack>
        <Title>
            {t("contentField.list.title")}
        </Title>
        {data && data.results.map((d) => <div key={d.id}>
            {d.name}
            <Divider />
        </div>)}
        <Button onClick={() => navigate("/content_fields/new/")}>
            {t("contentField.list.newField")}
        </Button>
    </Stack>
}
