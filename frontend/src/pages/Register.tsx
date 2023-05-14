import React, {useEffect, useState} from 'react';
import {Button, Container, PasswordInput, Stack, TextInput, Title} from "@mantine/core";
import {useNavigate} from "react-router-dom";
import {useTranslation} from "react-i18next";
import useRegister from "../hooks/useRegister";
import {useToken} from "../contexts/TokenContext";

export default function Register() {
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const { mutate, isLoading, isSuccess, data} = useRegister();
    const { setToken } = useToken();
    const navigate = useNavigate();
    const { t } = useTranslation();

    useEffect(() => {
        if (isSuccess && data ) {
            setToken(data.token);
            navigate("/");
        }
    }, [navigate, isSuccess, data, setToken])

    return <Container>
        <Stack>
            <Title>
                {t("shared.register")}
            </Title>
            <TextInput type="email" placeholder={t("shared.email") as string} value={email} onChange={(val) => setEmail(val.currentTarget.value)}/>
            <PasswordInput placeholder={t("shared.password") as string} value={password} onChange={(val) => setPassword(val.currentTarget.value)}/>
            <Button onClick={() => mutate({email, password})} loading={isLoading}>
                {t("shared.register")}
            </Button>
            <Button variant="outline" onClick={() => navigate("/login")}>
                {t("shared.login")}
            </Button>
        </Stack>
    </Container>
}
