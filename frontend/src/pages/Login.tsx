import React, {useEffect, useState} from 'react';
import {Button, Container, PasswordInput, Stack, TextInput, Title} from "@mantine/core";
import {useNavigate} from "react-router-dom";
import {useTranslation} from "react-i18next";
import useLogin from "../hooks/useLogin";
import {useToken} from "../contexts/TokenContext";

export default function Login() {
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const { mutate, isLoading, isSuccess, data } = useLogin();
    const {setToken} = useToken();
    const navigate = useNavigate();
    const { t } = useTranslation();

    useEffect(() => {
        if (isSuccess && data) {
            setToken(data.token)
            navigate("/");
        }
    }, [navigate, isSuccess, data, setToken])

    return <Container>
        <Stack>
            <Title>
                {t("shared.login")}
            </Title>
            <TextInput type="email" placeholder={t("shared.email") as string} value={email} onChange={(val) => setEmail(val.currentTarget.value)}/>
            <PasswordInput placeholder={t("shared.password") as string} value={password} onChange={(val) => setPassword(val.currentTarget.value)}/>
            <Button onClick={() => mutate({email, password})} loading={isLoading}>
                {t("shared.login")}
            </Button>
            <Button variant="outline" onClick={() => navigate("/register")}>
                {t("shared.register")}
            </Button>
        </Stack>
    </Container>
}
