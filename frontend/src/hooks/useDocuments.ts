import {useQuery} from "react-query";
import {AxiosError} from "axios";
import {useClientContext} from "../contexts/ClientContext";
import PinheadDocument from "../models/pinheadDocument";

export interface DocumentList {
    results: PinheadDocument[];
}

export default function useDocuments() {
    const { client } = useClientContext();
    return useQuery<DocumentList, AxiosError>('DOCUMENT_LIST', () => client.get<never, DocumentList>("/api/v1/documents/"))
}
