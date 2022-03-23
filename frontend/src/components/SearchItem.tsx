import { Component } from "solid-js";
import { StateManager } from "../StateManager";
import { SearchItemProps } from "../api";

interface Props {
    state: StateManager;
    item: SearchItemProps;
}

const sanitizePageTitle = (title: string): string => title.split(" ").join("-");

export const SearchItem: Component<Props> = ({ state, item }) => {
    const redirect = (event: Event, item: SearchItemProps) => {
        event.preventDefault();
        state.goto(`/equations/${item.id}/${sanitizePageTitle(item.title)}`);
    };

    return (
        <div class="search-item">
            <h2 class="search-title">{item.title}</h2>
            <a
                href={`/equations/${item.id}/${sanitizePageTitle(item.title)}`}
                class="search-anchor"
                onClick={(event: Event) => redirect(event, item)}
            >
                Se formelside
            </a>
        </div>
    );
};
