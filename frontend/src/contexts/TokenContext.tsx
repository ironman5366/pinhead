import React, {useCallback, useState} from 'react';
import constate from "constate";
import {AUTH_TOKEN_STORAGE_KEY} from "../constants";


export const [TokenProvider, useToken] = constate(() => {
    const [authToken, setAuthToken] = useState(AUTH_TOKEN_STORAGE_KEY);

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
