import React, {useEffect, useState} from 'react';
import {Button, Container, PasswordInput, Stack, TextInput, Title} from "@mantine/core";
import {useNavigate} from "react-router-dom";
import useRegister from "../hooks/useRegister";
import {useToken} from "../contexts/TokenContext";

export default function Register() {
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const { mutate, isLoading, isSuccess, data} = useRegister();
    const { setToken } = useToken();
    const navigate = useNavigate();

    useEffect(() => {
        if (isSuccess && data ) {
            setToken(data.token);
            navigate("/");
        }
    }, [navigate, isSuccess, data, setToken])

    return <Container>
        <Stack>
            <Title>
                Register
            </Title>
            <TextInput type="email" placeholder="Email" value={email} onChange={(val) => setEmail(val.currentTarget.value)}/>
            <PasswordInput placeholder="Password" value={password} onChange={(val) => setPassword(val.currentTarget.value)}/>
            <Button onClick={() => mutate({email, password})} loading={isLoading}>
                Register
            </Button>
            <Button variant="outline" onClick={() => navigate("/login")}>
                Login
            </Button>
        </Stack>
    </Container>
}
