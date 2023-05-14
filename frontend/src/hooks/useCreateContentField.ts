import {useMutation} from "react-query";
import {AxiosError} from "axios";
import {useClient} from "../contexts/ClientContext";
import {ContentField} from "../types/ContentField";

export interface CreateContentFieldOptions {
    name: string | null,
    code: string,
    schema: string
}

export default function useCreateContentField() {
 const { client } = useClient();
 return useMutation<ContentField, AxiosError, CreateContentFieldOptions>((data) => client.post("api/v1/content_fields/", data))
}
