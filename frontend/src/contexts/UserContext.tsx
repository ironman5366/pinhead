import React, {useEffect, useState} from 'react';
import constate from "constate";
import {useNavigate} from "react-router-dom";
import User from "../types/User";
import useCurrentUser from "../hooks/useCurrentUser";
import {useToken} from "./TokenContext";

export const [UserProvider, useUser] = constate(() => {
    const { clearToken } = useToken();
    const [user, setUser] = useState<User | undefined>();
    const { data, isError, isSuccess} = useCurrentUser();
    const navigate = useNavigate();

    useEffect(() => {
        if (isError || isSuccess) {
            if (data) {
                console.debug("Logging in user ", data);
                setUser(data)
            } else {
                console.debug("Clearing token and going to login");
                clearToken();
                navigate("/login")
            }
        }
    }, [isError, isSuccess, data, setUser, clearToken, navigate]);

    return {
        user
    }
})
