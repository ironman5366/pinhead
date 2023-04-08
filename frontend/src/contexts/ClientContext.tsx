import constate from "constate";
import axios from "axios";
import {API_HOST} from "../constants";
import {useToken} from "./TokenContext";

const [ClientProvider, useClient] = constate(() => {
    const { token } = useToken();
    const client = axios.create(
        {
            baseURL: API_HOST,
            headers: {
                // Before token is set this will be empty, but we'll only be making requests to endpoints that don't
                // look at the header, so it won't be a problem
                Authorization: `Bearer ${token}`
            }
        }
    )
    return { client }
})

export { ClientProvider, useClient }
