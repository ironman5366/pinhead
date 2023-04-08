import {useMutation} from "react-query";
import {JSONContent} from "@tiptap/react";
import {AxiosError} from "axios";
import {useClient} from "../contexts/ClientContext";

export interface CreateDocumentOptions {
    title: string;
    content: JSONContent;
}


export default function useCreateDocument() {
    const { client } = useClient();
    return useMutation<Document, AxiosError, CreateDocumentOptions>((data) =>
        client.post("api/v1/documents/", data)
    )
}
