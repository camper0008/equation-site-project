import "solid-js";
import { Component } from "solid-js";
import { SiteHeader } from "./SiteHeader";
import { StateManager } from "./StateManager";

interface Props {
    state: StateManager,
}

const SearchPage: Component<Props> = ({state}) => {
    return (<>
        <SiteHeader small state={state} />
    </>);
}

export default SearchPage;
