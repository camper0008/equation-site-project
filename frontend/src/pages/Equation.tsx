import { Component, createResource, createEffect, Show } from "solid-js";
import { apiUrl, get, Permission } from "../api";
import { StateManager } from "../StateManager";
import { urlParams } from "../utils";
import "../assets/equation-page.scss";
import { EsParser } from "esdoc";

interface Props {
    state: StateManager;
}

const EditButton: Component<Props> = ({ state }) => {
    const redirect = (event: Event, path: string) => {
        event.preventDefault();
        state.goto(path);
    };

    const { id } = urlParams("/equations/:id/:title", state.path());
    const userLoggedIn = state.userLoggedIn();
    if (userLoggedIn === null || userLoggedIn === undefined) {
        return <></>;
    }

    const permission = state.userLoggedIn()!.permission;
    return (
        <Show
            when={
                permission === Permission.Contributor ||
                permission === Permission.Root
            }
            fallback={<></>}
        >
            <a
                href={"/editor/" + id}
                onClick={(event) => redirect(event, "/editor/" + id)}
                class="edit-button"
            >
                Rediger Formel
            </a>
        </Show>
    );
};

const Equation: Component<Props> = ({ state }) => {
    const errorOccurred = () => {
        return <h1 class="error">En fejl opstod, under din anmodning</h1>;
    };

    const fetchData = async () => {
        const { id } = urlParams("/equations/:id/:title", state.path());
        const res = await get(apiUrl() + `/equations/one/${id}`);

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
            <EditButton state={state} />
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
                    <div>{parseEquation() as unknown as string}</div>
                </Show>
            </Show>
        </article>
    );
};

export default Equation;
