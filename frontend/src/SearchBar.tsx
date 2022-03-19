import searchIcon from "./assets/searchIcon.svg";
import { pathMatches, urlParams } from "./utils";

const isFocused = (event: InputEvent) => {
    const inputFocused = event.target.matches(":focus");

    return inputFocused;
}

export const SearchBar = (props) => {
    const handleInputEvent = (event: InputEvent) => {
        props.setFocused(isFocused(event));
    }

    const urlSearchValue = () => {
        const pattern = "/search/:query";
        if (pathMatches(pattern, props.state.path())) {
            const params = urlParams(pattern, props.state.path());
            
            return decodeURIComponent(params.query);
        }
        return "";
    }
    
    const handleKeyDownEvent = (event: KeyboardEvent) => {
        if (event.key === "Enter") {
            const searchbarValue = event.target.value;
            const encodedValue = encodeURIComponent(searchbarValue)
            props.state.goto("/search/" + encodedValue);
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
