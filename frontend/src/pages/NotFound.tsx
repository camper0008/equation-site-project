import { Logo } from "../components/Logo";
import { StateManager } from "../StateManager";
import { Component } from "solid-js";

interface Props {
    state: StateManager;
}

const NotFound: Component<Props> = ({ state }) => {
    return (
        <>
            <Logo state={state} />
            <p style="text-align: center; padding-top: 1rem;">
                Fejlkode <code>404</code>, siden kunne ikke findes af routeren.
            </p>
            <p style="text-align: center">
                Hvis du mener dette er en fejl, kontakt ejerne.
            </p>
        </>
    );
};

export default NotFound;
