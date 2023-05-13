import {useQuery, UseQueryResult} from "react-query";
import {AxiosError} from "axios";
import {useClient} from "../contexts/ClientContext";

export interface CodeAvailableResponse {
    available: string
}

export function useCodeAvailable(code: string): UseQueryResult<CodeAvailableResponse, AxiosError> {
    const {client} = useClient();
    return useQuery(['CODE_AVAILABLE', code], () => client.get<CodeAvailableResponse>(`/api/v1/content_fields/code_available/${code}/`), {
        enabled: !!code,
    });
}
