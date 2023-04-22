import React, {useCallback, useEffect, useState} from 'react';
import constate from "constate";
import {AUTH_TOKEN_STORAGE_KEY} from "../constants";


export const [TokenProvider, useToken] = constate(() => {
    const [authToken, setAuthToken] = useState("");

    useEffect(() => {
        const storedToken = localStorage.getItem(AUTH_TOKEN_STORAGE_KEY);
        if (storedToken) {
            setAuthToken(storedToken)
        }
    }, [])

    const setToken = useCallback((token: string) => {
        setAuthToken(token)
        localStorage.setItem(AUTH_TOKEN_STORAGE_KEY, token)
    }, [])

    const clearToken = useCallback(() => setToken(""), [setToken])

    return {
        token: authToken,
        clearToken, setToken
    }
})
