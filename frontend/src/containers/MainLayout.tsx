import React, { PropsWithChildren } from 'react';
import {
    Grid,
    Header,
    rem,
    useMantineTheme,
} from '@mantine/core';
import {Outlet} from "react-router-dom";
import Wordmark from '../components/atoms/Wordmark';
import Sidebar from "../components/organisms/Sidebar";
import AuthContainer from "./AuthContainer";

const HEADER_HEIGHT = rem(128);
const SIDEBAR_HEIGHT = `calc(100vh - ${HEADER_HEIGHT})`;


export default function MainLayout({ children }: PropsWithChildren<any>) {
    const theme = useMantineTheme();

    return (
        <AuthContainer>
            <Header
                height={HEADER_HEIGHT}
                style={{
                    borderBottom: `10px solid ${theme.colors.saffron[9]}`,
                }}
            >
                <Wordmark />
            </Header>
            <Grid>
                <Grid.Col span={3}>
                    <Sidebar height={SIDEBAR_HEIGHT}/>
                </Grid.Col>
                <Grid.Col span="auto">
                    <Outlet />
                </Grid.Col>
            </Grid>
        </AuthContainer>
    );
}
