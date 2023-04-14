import React, {PropsWithChildren} from 'react';
import {Container} from "@mantine/core";

export default function GuestLayout({children}: PropsWithChildren<any>) {
    return <Container fluid>
        {children}
    </Container>
}
