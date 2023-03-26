import {JSONContent} from "@tiptap/react";

export default interface DocumentVersion {
    id: number;
    content: JSONContent;
    documentId: number;
    createdAt: string;
    updatedAt: string
}
