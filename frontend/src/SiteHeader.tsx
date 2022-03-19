import "./assets/header.scss";
import { createSignal, getOwner } from 'solid-js';
import { SearchBar } from "./SearchBar";

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

export const SiteHeader = (props) => {
    const [focused, setFocused] = createSignal(false);

    const gotoIndex = () => {
        props.state.goto("/");
    }

    return <header 
            class={
                (focused() ? "focused " : "") +
                (props.small ? "small" : "")}
        >
        <h1 onClick={gotoIndex}>Formelsamling<span class="logo-tld">.dk</span></h1>
        <SearchBar state={props.state} setFocused={setFocused}/>
        <AutoCompleteList />
    </header>
}
