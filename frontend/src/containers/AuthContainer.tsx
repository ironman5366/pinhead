import React, {PropsWithChildren} from 'react';
import {Loader} from "@mantine/core";
import {useUser} from "../contexts/UserContext";
import GuestLayout from "./GuestLayout";

export default function AuthContainer({children}: PropsWithChildren<any>) {
    const { user } = useUser();

    if (!user) {
        return <GuestLayout>
            <Loader />
        </GuestLayout>
    }

    return children
}
