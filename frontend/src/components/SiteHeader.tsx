import { StateManager } from "../StateManager";
import { Component } from "solid-js";
import "../assets/site-header.scss";

interface Props {
    state: StateManager,
}

export const SiteHeader: Component<Props> = ({state}) => {

    const anchorClickHandler = (event: Event, path: string) => {
        event.preventDefault();
        state.goto(path);
    }

    return <nav class="main-nav">
        <a href="/" class="logo" onClick={(event) => anchorClickHandler(event, "/")}>Formelsamling<span class="logo-tld">.dk</span></a>
        <a href="/login"         onClick={(event) => anchorClickHandler(event, "/login")}>Login</a>
    </nav>
}
