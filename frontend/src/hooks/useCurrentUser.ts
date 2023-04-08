import {useQuery} from "react-query";
import {AxiosError} from "axios";
import User from "../types/User";
import {useClient} from "../contexts/ClientContext";
import {useToken} from "../contexts/TokenContext";


export default function useCurrentUser() {
    const { token } = useToken();
    const { client } = useClient();

    return useQuery<User, AxiosError>('CURRENT_USER', () => client.get<never, User>("/api/v1/users/current/"), {
        // Only run the query if the token is set
        enabled: !!token
    })
}
