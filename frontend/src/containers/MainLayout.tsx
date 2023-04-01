import React, { PropsWithChildren } from 'react';
import {
    Container,
    Grid,
    Header,
    rem,
    useMantineTheme,
} from '@mantine/core';
import Wordmark from '../components/atoms/Wordmark';
import NavbarSimple from "../components/organisms/Navbar";

const HEADER_HEIGHT = rem(128);
const SIDEBAR_HEIGHT = `calc(100vh - ${HEADER_HEIGHT})`;

export default function MainLayout({ children }: PropsWithChildren<any>) {
    const theme = useMantineTheme();

    return (
        <>
            <Header
                height={HEADER_HEIGHT}
                style={{
                    borderBottom: `10px solid ${theme.colors.saffron[9]}`,
                }}
            >
                <Wordmark />
            </Header>
            <NavbarSimple height={SIDEBAR_HEIGHT}/>
        </>
    );
}
