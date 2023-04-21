import {useQuery} from "react-query";
import {AxiosError} from "axios";
import {useClient} from "../contexts/ClientContext";
import {ContentField} from "../types/ContentField";

export interface ContentFieldList {
    results: ContentField[];
}

export default function useContentFields() {
    const { client } = useClient();
    return useQuery<ContentFieldList, AxiosError>('CONTENT_FIELD_LIST', () => client.get<never, ContentFieldList>("/api/v1/content_fields/"))
}
