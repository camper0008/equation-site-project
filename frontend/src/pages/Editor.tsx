import { Logo } from "../components/Logo";
import { StateManager } from "../StateManager";
import { Component } from "solid-js"

interface Props {
    state: StateManager;
}

const Editor: Component<Props> = ({state}) => {
    return <>
        <Logo state={state}/>
    </>
}

export default Editor;
