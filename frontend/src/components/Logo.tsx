import "../assets/logo.scss";
import { StateManager } from "../StateManager";
import { Component } from "solid-js";

interface Props {
    state: StateManager,
}

export const Logo: Component<Props> = ({state}) => {

    const gotoIndex = () => {
        state.goto("/");
    }

    return <h1 class="logo" onClick={gotoIndex}>Formelsamling<span class="logo-tld">.dk</span></h1>

}
