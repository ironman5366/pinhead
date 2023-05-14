import React from "react";
import {JsonInput, JsonInputProps} from "@mantine/core";

export default function FormattedJSONInput(props: JsonInputProps) {
    return <JsonInput
        minRows={4}
        formatOnBlur
        {...props}
    />
}
