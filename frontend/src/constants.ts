import {JSONContent} from "@tiptap/react";

export const API_HOST = "http://localhost:8000/";

// The empty string in ProseMirror
export const EMPTY_DOCUMENT: JSONContent = {
    type: "doc",
    content: [
        {type: "paragraph"}
    ]
}

// The key that the auth token will be stored under in local storage
export const AUTH_TOKEN_STORAGE_KEY = "auth_token";
