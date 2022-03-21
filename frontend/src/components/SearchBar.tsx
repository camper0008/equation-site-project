import { Component, JSX, Setter } from "solid-js";
import searchIcon from "../assets/icon-search.svg";
import { StateManager } from "../StateManager";
import { pathMatches, urlParams } from "../utils";

const isFocused = (target: HTMLInputElement) => {
    const inputFocused = target.matches(":focus");

    return inputFocused;
}

interface Props {
    state: StateManager,
    setFocused: Setter<boolean>,
}

export const SearchBar: Component<Props> = ({state, setFocused}) => {
    const handleInputEvent: JSX.EventHandler<HTMLInputElement, FocusEvent | InputEvent> = (event) => {
        setFocused(isFocused(event.target as HTMLInputElement));
    }

    const urlSearchValue = () => {
        const pattern = "/search/:query";
        if (pathMatches(pattern, state.path())) {
            const params = urlParams(pattern, state.path());
            
            return decodeURIComponent(params.query);
        }
        return "";
    }
    
    const handleKeyDownEvent: JSX.EventHandler<HTMLInputElement, KeyboardEvent> = (event) => {
        if (event.key === "Enter") {
            const searchbarValue = (event.target as HTMLInputElement).value;
            const encodedValue = encodeURIComponent(searchbarValue)
            state.goto("/search/" + encodedValue);
        }
    }

    return(<label class="search-container" for="search">
        <img src={searchIcon} />
        <input 
            value={urlSearchValue()}

            type="search" id="search" list="autocomplete" placeholder="Find formler..." 
            onInput={handleInputEvent} onFocus={handleInputEvent} onBlur={handleInputEvent}
            onKeyDown={handleKeyDownEvent}
        />
    </label>)
}
