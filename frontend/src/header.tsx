import "./assets/header.scss";
import searchIcon from "./assets/searchIcon.svg";
import { createSignal } from 'solid-js';

export const SiteHeader = (props) => {
    const [focused, setFocused] = createSignal(false);

    const handleInputEvent = (event: InputEvent) => {
        const strippedInputValue = event.target.value.replace(/ /g, '');
        const inputFocused = event.target.matches(":focus");

        setFocused(strippedInputValue || inputFocused);
    }

    return <header class={(focused() ? "focused" : "") + " " + (props.small ? "small" : "")}>
        <h1>Formelsamling<span class="logo-tld">.dk</span></h1>
        <label class="search-container" for="search">
            <img src={searchIcon} />
            <input type="search" id="search" list="search-results" placeholder="Find formler..." onInput={handleInputEvent} onFocus={handleInputEvent} onBlur={handleInputEvent}/>
        </label>

        <datalist id="search-results">
          <option value="Edge" />
          <option value="Firefox" />
          <option value="Chrome" />
          <option value="Opera" />
          <option value="Safari" />
        </datalist>
    </header>
}
