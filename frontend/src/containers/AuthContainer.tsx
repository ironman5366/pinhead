import React, {PropsWithChildren, useEffect} from 'react';
import {useNavigate} from "react-router-dom";
import {Loader} from "@mantine/core";
import {useUser} from "../contexts/UserContext";
import GuestLayout from "./GuestLayout";

export default function AuthContainer({ children }: PropsWithChildren<any>) {
    const { user, isLoading } = useUser();
    const navigate = useNavigate();

    useEffect(() => {
        if (!user && !isLoading) {
            console.log("Would navigate to login because not user and not isLoading")
        }
    }, [navigate, user, isLoading])

    if (isLoading) {
        return <GuestLayout>
            <Loader />
        </GuestLayout>
    }

    return children
}
