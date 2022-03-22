import "../assets/footer.scss";
import { StateManager } from "../StateManager";
import { Component } from "solid-js";

interface Props {
    state: StateManager;
}

export const Footer: Component<Props> = ({ state }) => {
    const legalClickHandler = (event: Event) => {
        event.preventDefault();
        state.goto("/privacy");
    };

    return (
        <footer>
            Skabt af{" "}
            <a href="https://tphollebeek.dk" target="_blank">
                Theis Pieter Hollebeek
            </a>{" "}
            med input fra{" "}
            <a href="https://simonfromjakobsen.netlify.app" target="_blank">
                Simon From Jakobsen
            </a>{" "}
            | Vores{" "}
            <a href="/privacy" onClick={legalClickHandler}>
                Privatlivspolitik
            </a>{" "}
            | Koden bag denne hjemmeside findes på{" "}
            <a
                href="https://github.com/camper0008/equation-site-project"
                target="_blank"
            >
                GitHub
            </a>
        </footer>
    );
};
