import React, {useEffect, useState} from 'react';
import constate from "constate";
import User from "../types/User";
import useCurrentUser from "../hooks/useCurrentUser";

export const [UserProvider, useUser] = constate(() => {
    const [user, setUser] = useState<User | undefined>();
    const { data, isLoading, } = useCurrentUser();

    useEffect(() => {
        if (data) {
            setUser(data)
        }
    }, [data, setUser]);

    return {
        user,
        isLoading
    }
})
