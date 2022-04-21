export const apiUrl = (): string => {
    if (window.location.hostname === "localhost") {
        return "http://localhost:8080/"
    } else {
        return "/api"
    }
}

type body = BodyInit | null | undefined;

export const post = async (url: string, body: body) => {
    return await (
        await fetch(url, {
            body,
            method: "POST",
            headers: new Headers({ "Content-Type": "application/json" }),
            credentials: "include", // TODO: remove if api and frontend is on same site
        })
    ).json();
};

export const get = async (url: string) => {
    return await (
        await fetch(url, {
            method: "GET",
            headers: new Headers({ "Content-Type": "application/json" }),
            credentials: "include", // TODO: remove if api and frontend is on same site
        })
    ).json();
};

export enum Permission {
    User = "User",
    Contributor = "Contributor",
    Root = "Root",
}

export interface User {
    id: string; // randomly generated
    username: string;
    permission: Permission;
    posts: string[]; // equation ids
    date_created: string; // ISO string
}

interface FetchUserInfoValue {
    value: User | null | undefined;
    refetching?: unknown;
}

export interface SearchItemProps {
    id: string;
    title: string;
    dateCreated: string;
}

export async function fetchUserInfo(
    source: User | null,
    { value, refetching }: FetchUserInfoValue,
) {
    const res = await get(apiUrl() + "/users/info");

    if (res.ok) {
        return res.user;
    } else {
        return null;
    }
}
