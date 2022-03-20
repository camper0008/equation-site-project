import "./assets/header.scss";
import { Component, createSignal, getOwner } from 'solid-js';
import { SearchBar } from "./SearchBar";
import { StateManager } from "./StateManager";

const autocompleteValues = (): string[] => {
    return [
        "Edge",
        "Firefox",
        "Chrome",
        "Opera",
        "Safari",
    ]
}

const AutoCompleteList = () => {
    const values = autocompleteValues();
    return <datalist id="autocomplete"> {values.map((v) => <option value={v}/>)} </datalist>
}

interface Props {
    state: StateManager,
    small?: boolean,
}

export const SiteHeader: Component<Props> = ({state, small}) => {
    const [focused, setFocused] = createSignal(false);

    const gotoIndex = () => {
        state.goto("/");
    }

    return <header 
            class={
                (focused() ? "focused " : "") +
                (small ? "small" : "")}
        >
        <h1 onClick={gotoIndex}>Formelsamling<span class="logo-tld">.dk</span></h1>
        <SearchBar state={state} setFocused={setFocused}/>
        <AutoCompleteList />
    </header>
}
