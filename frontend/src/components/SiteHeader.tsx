import { StateManager } from "../StateManager";
import { Component } from "solid-js";
import "../assets/site-header.scss";
import "../assets/logo.scss";
import { API_URL, get, post, Permission } from "../api";

interface Props {
    state: StateManager;
}

export const SiteHeader: Component<Props> = ({ state }) => {
    const logoutClickHandler = async (event: Event) => {
        event.preventDefault();
        const res = await post(API_URL + "/users/logout", null);
        if (!res.ok) {
            console.error(`an error occured trying to log out: ${res.msg}`);
        }
        state.refetchUserLoggedIn();
    };

    const redirect = (event: Event, path: string) => {
        event.preventDefault();
        state.goto(path);
    };

    const loginOrLogout = () => {
        const userState = state.userLoggedIn();
        if (userState === null || userState === undefined) {
            return (
                <a href="/login" onClick={(event) => redirect(event, "/login")}>
                    Login
                </a>
            );
        } else {
            return (
                <>
                    <p>{state.userLoggedIn()!.username}</p>
                    <a href="/logout" onClick={logoutClickHandler}>
                        Logout
                    </a>
                    {(() => {
                        const permission = state.userLoggedIn()!.permission;
                        return permission === Permission.Contributor ||
                            permission === Permission.Root ? (
                            <a
                                href="/editor/new"
                                onClick={(event) =>
                                    redirect(event, "/editor/new")
                                }
                            >
                                Ny formel
                            </a>
                        ) : (
                            <></>
                        );
                    })()}
                </>
            );
        }
    };

    return (
        <nav class="main-nav">
            <a href="/" class="logo" onClick={(event) => redirect(event, "/")}>
                Tekniskeformler<span class="logo-tld">.dk</span>
            </a>
            {loginOrLogout}
        </nav>
    );
};
