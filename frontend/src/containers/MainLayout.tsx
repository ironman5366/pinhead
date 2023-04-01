import React, {PropsWithChildren} from 'react';
import {Container, createStyles, Header, Image, rem, Stack, Title, useMantineTheme} from "@mantine/core";
import Logo from "../assets/images/logo.png"
import Wordmark from "../components/atoms/Wordmark";

const HEADER_HEIGHT = rem(128)

const useStyles = createStyles((theme) => ({
    inner: {
        height: HEADER_HEIGHT,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
    }
}));

export default function MainLayout({children}: PropsWithChildren<any>) {
    const {classes} = useStyles();
    const theme = useMantineTheme();
    return <>
        <Header height={HEADER_HEIGHT} style={{
            borderBottom: `10px solid ${theme.colors.saffron[9]}`
        }}>
            <Wordmark />
        </Header>
        <div>
            <Stack>
                <Title>
                    Documents
                </Title>
            </Stack>
            <Container>
                {children}
            </Container>
        </div>

    </>
}
