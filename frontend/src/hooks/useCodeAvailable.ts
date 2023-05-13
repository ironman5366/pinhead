import {useQuery, UseQueryResult} from "react-query";
import axios, {AxiosError} from "axios";

export interface CodeAvailableResponse {
    available: string
}

async function fetchCodeAvailable(code: string): Promise<CodeAvailableResponse> {
    const { data } = await axios.get<CodeAvailableResponse>(`/api/v1/content_fields/code_available/${code}`);
    return data;
}

export function useCodeAvailable(code: string): UseQueryResult<CodeAvailableResponse, AxiosError> {
    return useQuery(['CODE_AVAILABLE', code], () => fetchCodeAvailable(code), {
        enabled: !!code,
    });
}
