import constate from "constate";
import axios from "axios";
import {API_HOST} from "../constants";

const [ClientProvider, useClientContext] = constate(() => {
    const client = axios.create(
        {
            baseURL: API_HOST
        }
    )
    return { client }
})

export { ClientProvider, useClientContext }
