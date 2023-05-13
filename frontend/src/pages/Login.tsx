import React, {useEffect, useState} from 'react';
import {Button, Container, PasswordInput, Stack, TextInput, Title} from "@mantine/core";
import {useNavigate} from "react-router-dom";
import useLogin from "../hooks/useLogin";
import {useToken} from "../contexts/TokenContext";

export default function Login() {
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const { mutate, isLoading, isSuccess, data } = useLogin();
    const {setToken} = useToken();
    const navigate = useNavigate();

    useEffect(() => {
        if (isSuccess && data) {
            setToken(data.token)
            navigate("/");
        }
    }, [navigate, isSuccess, data, setToken])

    return <Container>
        <Stack>
            <Title>
                Login
            </Title>
            <TextInput type="email" placeholder="Email" value={email} onChange={(val) => setEmail(val.currentTarget.value)}/>
            <PasswordInput placeholder="Password" value={password} onChange={(val) => setPassword(val.currentTarget.value)}/>
            <Button onClick={() => mutate({email, password})} loading={isLoading}>
                Login
            </Button>
            <Button variant="outline" onClick={() => navigate("/register")}>
                Register
            </Button>
        </Stack>
    </Container>
}
