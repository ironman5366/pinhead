import {useMutation} from "react-query";
import {JSONContent} from "@tiptap/react";
import {AxiosError} from "axios";
import {useClientContext} from "../contexts/ClientContext";

export interface CreateDocumentOptions {
    title: string;
    content: JSONContent;
}


export interface CreateDocumentResponse {
    id: number;
}


export default function useCreateDocument() {
    const { client } = useClientContext();
    return useMutation<CreateDocumentResponse, AxiosError, CreateDocumentOptions>((data) =>
        client.post("api/v1/documents/", data)
    )
}
