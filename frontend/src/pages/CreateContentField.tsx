import React from 'react';
import {Stack, TextInput, Title} from "@mantine/core";
import { useForm } from '@mantine/form';
import { useDebouncedValue } from "@mantine/hooks";
import { IconCheck, IconX, IconLoader } from "@tabler/icons-react";
import {useCodeAvailable} from "../hooks/useCodeAvailable";

export default function CreateContentField() {
    const form = useForm({
        initialValues: {
            name: '',
            code: '',

        }
    })
    const [debouncedCode] = useDebouncedValue(form.values.code, 200);
    const { data, isLoading } = useCodeAvailable(debouncedCode);

    let Icon;
    if (isLoading) {
        Icon = IconLoader;
    } else {
        Icon = data?.available ? IconCheck : IconX;
    }


    return <Stack>
        <Title>
            Add Content Field
        </Title>
        <form>
            <TextInput label="Name" {...form.getInputProps("name")} />
            <div>
                <TextInput label="Code" {...form.getInputProps("code")} />
                <Icon size="3rem"/>
            </div>
        </form>
    </Stack>
}
