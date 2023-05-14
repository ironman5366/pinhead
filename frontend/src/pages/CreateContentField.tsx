import React from 'react';
import {Button, JsonInput, Stack, TextInput, Title} from "@mantine/core";
import { useForm } from '@mantine/form';
import { useDebouncedValue } from "@mantine/hooks";
import { IconCheck, IconX, IconLoader } from "@tabler/icons-react";
import jsonschema from "jsonschema";
import {useTranslation} from "react-i18next";
import {useCodeAvailable} from "../hooks/useCodeAvailable";


function validateSchema(schema: string): boolean {
    console.log("Validating schema ", schema);
    const schemaObj = JSON.parse(schema);
    const validator = new jsonschema.Validator();
    const res =  validator.addSchema(schemaObj);
    const bool = !!res;
    console.log(res, bool);
    return bool
}

export default function CreateContentField() {
    const form = useForm({
        initialValues: {
            name: '',
            code: '',
            schema: '',
        },
        validate: {
            schema: (value) => validateSchema(value) ? null : "Couldn't validate schema"
        }
    })
    const [debouncedCode] = useDebouncedValue(form.values.code, 200);
    const { data, isLoading } = useCodeAvailable(debouncedCode);
    const { t } = useTranslation();

    let Icon;
    if (isLoading) {
        Icon = IconLoader;
    } else {
        Icon = data?.available ? IconCheck : IconX;
    }


    return <Stack>
        <Title>
            {t("contentField.newFieldForm.title")}
        </Title>
        <form onSubmit={form.onSubmit((values) => console.log(values))}>
            <TextInput label={t("contentField.newFieldForm.name")} {...form.getInputProps("name")} />
            <div>
                <TextInput label={t("contentField.newFieldForm.code")} {...form.getInputProps("code")} />
                <Icon size="3rem"/>
            </div>
            <JsonInput label={t("contentField.newFieldForm.schema")} {...form.getInputProps("schema")} />
            <Button>
                {t("shared.submit")}
            </Button>
        </form>
    </Stack>
}
