import "solid-js";
import { Component } from "solid-js";
import { SiteHeader } from "./SiteHeader";
import { StateManager } from "./StateManager";

interface Props {
    state: StateManager
}

const Home: Component<Props> = ({state}) => {
    return (<>
        <SiteHeader state={state} />
    </>);
}

export default Home;
