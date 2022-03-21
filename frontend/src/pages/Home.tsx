import "solid-js";
import { Component } from "solid-js";
import { SearchWrapper } from "../components/SearchWrapper";
import { StateManager } from "../StateManager";

interface Props {
    state: StateManager
}

const Home: Component<Props> = ({state}) => {
    return (<>
        <SearchWrapper state={state} />
    </>);
}

export default Home;
