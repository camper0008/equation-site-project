import "../assets/footer.scss";
import { StateManager } from "../StateManager";
import { Component } from "solid-js";

interface Props {
    state: StateManager;
}

export const Footer: Component<Props> = ({ state }) => {
    const legalClickHandler = (event: Event) => {
        event.preventDefault();
        state.goto("/legal");
    };

    return (
        <footer>
            Skabt af{" "}
            <a href="https://tphollebeek.dk" target="_blank">
                Theis Pieter Hollebeek
            </a>{" "}
            og{" "}
            <a href="https://simonfromjakobsen.netlify.app" target="_blank">
                Simon From Jakobsen
            </a>{" "}
            - Sidens{" "}
            <a href="/legal" onClick={legalClickHandler}>
                GPDR
            </a>
            .
        </footer>
    );
};
