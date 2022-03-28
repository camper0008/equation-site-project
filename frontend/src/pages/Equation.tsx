import { Component, createResource, createEffect, Show } from "solid-js";
import { API_URL, get } from "../api";
import { StateManager } from "../StateManager";
import { urlParams } from "../utils";
import "../assets/equation-page.scss";
import { EsParser } from "esdoc";

interface Props {
    state: StateManager;
}

const Equation: Component<Props> = ({ state }) => {
    const errorOccurred = () => {
        return <h1 class="error">En fejl opstod, under din anmodning</h1>;
    };

    const fetchData = async () => {
        const { id } = urlParams("/equations/:id/:title", state.path());
        const res = await get(API_URL + `/equations/one/${id}`);

        return res;
    };

    const [res] = createResource(fetchData);

    const parseEquation = () => {
        if (res().ok && res().equation) {
            const parser = new EsParser(res().equation.content);
            const doc = parser.parse();
            return doc.toHyperComponent();
        }
        return "";
    };

    return (
        <article class="equation clamped-page-view">
            <Show
                when={!res.loading && !res.error}
                fallback={
                    <Show
                        when={res.loading && !res.error}
                        fallback={errorOccurred()}
                    >
                        <h1>Henter...</h1>
                    </Show>
                }
            >
                <Show
                    when={res().ok && res().equation}
                    fallback={errorOccurred()}
                >
                    <h1>{res().equation.title}</h1>
                    <div>{parseEquation()}</div>
                </Show>
            </Show>
        </article>
    );
};

export default Equation;
