import React, {useEffect} from 'react';
import {Box, Button, Checkbox, Flex, Group, JsonInput, Stack, TextInput, Title} from "@mantine/core";
import { useForm } from '@mantine/form';
import { useDebouncedValue } from "@mantine/hooks";
import { IconCheck, IconX, IconLoader } from "@tabler/icons-react";
import jsonschema from "jsonschema";
import {useTranslation} from "react-i18next";
import {useNavigate} from "react-router-dom";
import {useCodeAvailable} from "../hooks/useCodeAvailable";
import useCreateContentField from "../hooks/useCreateContentField";
import FormattedJSONInput from "../components/atoms/FormattedJSONInput";


function validateSchema(schema: string): boolean {
    let schemaObj;
    try {
        schemaObj = JSON.parse(schema);
    } catch (e) {
        return false
    }

    const validator = new jsonschema.Validator();
    const res =  validator.addSchema(schemaObj);
    console.log(res)
    return !!res
}

export default function CreateContentField() {
    const form = useForm({
        initialValues: {
            name: "",
            code: "",
            schema: ""
        },
        validate: {
            schema: (value) => validateSchema(value) ? null : "Couldn't validate schema"
        }
    })

    const { t } = useTranslation();
    const [debouncedCode] = useDebouncedValue(form.values.code, 200);
    const { data, isLoading } = useCodeAvailable(debouncedCode);
    const { mutate, isSuccess } = useCreateContentField();
    const navigate = useNavigate();

    useEffect(() => {
        if (isSuccess) {
            navigate("/content_fields/")
        }
    }, [navigate, isSuccess])


    let Icon;
    let iconColor;
    if (isLoading) {
        Icon = IconLoader;
        iconColor = "black";
    } else if (data?.available) {
        Icon = IconCheck;
        iconColor = "green";
    } else {
        Icon = IconX;
        iconColor = "red"
    }

    return <Stack>
        <Title>
            {t("contentField.newFieldForm.title")}
        </Title>
        <form onSubmit={form.onSubmit(({name, code, schema}) => {
            mutate({
                name,
                code,
                schema: JSON.parse(schema)
            })
        })}>
            <TextInput label={t("contentField.newFieldForm.name")} {...form.getInputProps("name")} />
            <Flex direction="row" align="flex-end">
                <TextInput label={t("contentField.newFieldForm.code")} {...form.getInputProps("code")} />
                <Icon size="2rem" style={{
                    color: iconColor
                }}/>
            </Flex>
            <FormattedJSONInput
                label={t("contentField.newFieldForm.schema")}
                       {...form.getInputProps("schema")}
            />
            <br />
            <Button type="submit">
                {t("shared.submit")}
            </Button>
        </form>
    </Stack>
}
