import constate from "constate";
import axios from "axios";
import {useMemo} from "react";
import {API_HOST} from "../constants";
import {useToken} from "./TokenContext";

const [ClientProvider, useClient] = constate(() => {
    const { token } = useToken();
    const client = useMemo(() => {
        const axiosClient = axios.create(
            {
                baseURL: API_HOST,
                headers: {
                    // Before token is set this will be empty, but we'll only be making requests to endpoints that don't
                    // look at the header, so it won't be a problem
                    Authorization: `Bearer ${token}`
                }
            }
        )
        axiosClient.interceptors.response.use(
            // We don't want to have to access response.data.data
            async (response) => response.data
        )

        return axiosClient
    }, [token])

    return { client }
})

export { ClientProvider, useClient }
