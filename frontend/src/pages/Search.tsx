import { Component, createResource, For, Show } from "solid-js";
import { SearchWrapper } from "../components/SearchWrapper";
import { SearchItem } from "../components/SearchItem";
import { StateManager } from "../StateManager";
import { urlParams } from "../utils";
import { get, apiUrl, SearchItemProps } from "../api";
import "../assets/search-item.scss";

interface Props {
    state: StateManager;
}

const SearchPage: Component<Props> = ({ state }) => {
    const fetchData = async () => {
        const { query } = urlParams("/search/:query", state.path());
        const res = await get(apiUrl() + `/equations/search/${query}`);

        return res;
    };

    const [res, { refetch }] = createResource(fetchData);

    return (
        <>
            <SearchWrapper small state={state} refetch={refetch} />
            {
                <div class="search-item-container clamped-page-view">
                    <Show
                        when={!res.loading && !res.error}
                        fallback={
                            <Show
                                when={res.loading}
                                fallback={
                                    <h2 class="error">
                                        En fejl opstod, under din anmodning.
                                    </h2>
                                }
                            >
                                <h2>Henter...</h2>
                            </Show>
                        }
                    >
                        <For each={res().equations}>
                            {(item: SearchItemProps, idx) => {
                                return <SearchItem state={state} item={item} />;
                            }}
                        </For>
                    </Show>
                </div>
            }
        </>
    );
};

export default SearchPage;
