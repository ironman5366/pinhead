import { useQuery } from "react-query";
import { AxiosError } from "axios";
import { useClient } from "../contexts/ClientContext"
import ContentType from "../types/ContentType";

export interface ContentTypeList {
    results: ContentType[]
}

export default function useContentTypes() {
    const { client } = useClient();
    return useQuery<ContentTypeList, AxiosError>('CONTENT_TYPE_LIST', () => client.get<never, ContentTypeList>("/api/v1/content_types/"))
}
