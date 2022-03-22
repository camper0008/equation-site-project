import "../assets/logo.scss";
import { StateManager } from "../StateManager";
import { Component } from "solid-js";

interface Props {
    state: StateManager;
}

export const Logo: Component<Props> = ({ state }) => {
    const gotoIndex = (event: Event) => {
        event.preventDefault();
        state.goto("/");
    };

    return (
        <h1 class="logo">
            <a href="/" onClick={gotoIndex}>
                Formelsamling<span class="logo-tld">.dk</span>
            </a>
        </h1>
    );
};
