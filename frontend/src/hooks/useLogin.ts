import {useMutation} from "react-query";
import {AxiosError} from "axios";
import {useClient} from "../contexts/ClientContext";
import User from "../types/User";

export interface LoginOptions {
    email: string;
    password: string;
}

export interface LoginResponse {
    user: User,
    token: string
}

export default function useLogin() {
    const { client } = useClient();
    return useMutation<LoginResponse, AxiosError, LoginOptions>((data) =>
        client.post("api/v1/login/", data)
    )
}
