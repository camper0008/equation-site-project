import "../assets/search-wrapper.scss";
import { Component, createSignal, getOwner } from "solid-js";
import { SearchBar } from "./SearchBar";
import { Logo } from "./Logo";
import { StateManager } from "../StateManager";

const autoCompleteValues = (): string[] => {
    return ["Edge", "Firefox", "Chrome", "Opera", "Safari"];
};

const AutoCompleteList = () => {
    const values = autoCompleteValues();
    return (
        <datalist id="autocomplete">
            {values.map((v) => (
                <option value={v} />
            ))}
        </datalist>
    );
};

interface Props {
    state: StateManager;
    small?: boolean;
}

export const SearchWrapper: Component<Props> = ({ state, small }) => {
    const [focused, setFocused] = createSignal(false);

    return (
        <div
            class={
                "search-wrapper" +
                (focused() ? " focused" : "") +
                (small ? " small" : "")
            }
        >
            <Logo state={state} />
            <SearchBar state={state} setFocused={setFocused} />
            <AutoCompleteList />
        </div>
    );
};
