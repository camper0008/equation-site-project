import "./assets/header.scss";
import searchIcon from "./assets/searchIcon.svg";
import { createSignal, getOwner } from 'solid-js';
import { useRouter } from "./router";

const autocompleteValues = (): string[] => {
    return [
        "Edge",
        "Firefox",
        "Chrome",
        "Opera",
        "Safari",
    ]
}

const autocomplete = () => {
    const values = autocompleteValues();
    return <datalist id="autocomplete"> {values.map((v) => <option value={v}/>)} </datalist>
}

const isFocusedOrBeingUsed = (event: InputEvent) => {
    const strippedInputValue = event.target.value.replace(/ /g, '');
    const inputFocused = event.target.matches(":focus");

    return (strippedInputValue || inputFocused);
}

export const SiteHeader = (props) => {
    const [focused, setFocused] = createSignal(false);

    const handleInputEvent = (event: InputEvent) => {
        setFocused(isFocusedOrBeingUsed(event));
    }
    
    const [page, { goto }] = useRouter();

    const handleKeyDownEvent = (event: KeyboardEvent) => {
        if (event.key === "Enter") {
            const searchbarValue = event.target.value;
            const encodedValue = encodeURIComponent(searchbarValue)
            goto("/search/" + encodedValue);
        }
    }

    return <header 
            class={
                (focused() ? "focused " : "") +
                (props.small ? "small" : "")}
        >
        <h1>Formelsamling<span class="logo-tld">.dk</span></h1>
        <label class="search-container" for="search">
            <img src={searchIcon} />
            <input 
                type="search" id="search" list="autocomplete" placeholder="Find formler..." 
                onInput={handleInputEvent} onFocus={handleInputEvent} onBlur={handleInputEvent}
                onKeyDown={handleKeyDownEvent}
            />
        </label>
        {autocomplete()}
    </header>
}
