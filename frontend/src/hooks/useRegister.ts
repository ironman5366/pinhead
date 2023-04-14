import { useMutation } from "react-query";
import {AxiosError} from "axios";
import {useClient} from "../contexts/ClientContext";

export interface RegistrationOptions {
    email: string;
    password: string;
}

export interface RegistrationResponse {
    token: string;
}

export default function useRegister() {
    const { client } = useClient();

    return useMutation<RegistrationResponse, AxiosError, RegistrationOptions>((data) => client.post("/api/v1/register/", data))

}
