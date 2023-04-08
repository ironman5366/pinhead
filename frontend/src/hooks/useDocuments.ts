import {useQuery} from "react-query";
import {AxiosError} from "axios";
import {useClient} from "../contexts/ClientContext";
import PinheadDocument from "../types/PinheadDocument";

export interface DocumentList {
    results: PinheadDocument[];
}

export default function useDocuments() {
    const { client } = useClient();
    return useQuery<DocumentList, AxiosError>('DOCUMENT_LIST', () => client.get<never, DocumentList>("/api/v1/documents/"))
}
