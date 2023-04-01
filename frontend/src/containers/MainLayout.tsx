import React, {PropsWithChildren} from 'react';
import {Container, createStyles, Grid, Header, Image, Navbar, rem, Stack, Title, useMantineTheme} from "@mantine/core";
import Wordmark from "../components/atoms/Wordmark";

const HEADER_HEIGHT = rem(128);
const SIDEBAR_HEIGHT = `calc(100vh - ${HEADER_HEIGHT})`

export default function MainLayout({children}: PropsWithChildren<any>) {
    const theme = useMantineTheme();

    return <>
        <Header height={HEADER_HEIGHT} style={{
            borderBottom: `10px solid ${theme.colors.saffron[9]}`
        }}>
            <Wordmark />
        </Header>
        <Grid style={{
            minHeight: SIDEBAR_HEIGHT
        }} columns={24}>
            <Grid.Col md={5} style={{
                borderRight: "3px solid #F3BAC0",
                minHeight: SIDEBAR_HEIGHT
            }}>
                <Navbar>
                   Test
                </Navbar>
            </Grid.Col>
            <Grid.Col md={19}>
                <Container>
                    {children}
                </Container>
            </Grid.Col>
        </Grid>
    </>
}
