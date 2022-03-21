export const API_URL = "http://localhost:8080";

type body = BodyInit | null | undefined;

export const post = async (url: string, body: body) => {
    return await (await fetch(url, {
        body,
        method: "POST",
        headers: new Headers({"Content-Type": "application/json"}),
        credentials: "include" // TODO: remove if api and frontend is on same site
    })).json();
}
