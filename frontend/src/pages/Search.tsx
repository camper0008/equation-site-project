import "solid-js";
import { Component } from "solid-js";
import { SearchWrapper } from "../components/SearchWrapper";
import { StateManager } from "../StateManager";

interface Props {
    state: StateManager;
}

const SearchPage: Component<Props> = ({ state }) => {
    return (
        <>
            <SearchWrapper small state={state} />
        </>
    );
};

export default SearchPage;
