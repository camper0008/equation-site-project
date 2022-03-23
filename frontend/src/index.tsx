import "./assets/root.scss";
import "./assets/index.scss";

import { lazy, createSignal, Match, Switch, createResource } from "solid-js";
import { render } from "solid-js/web";
import { pathMatches } from "./utils";
import { StateManager } from "./StateManager";
import { SiteHeader } from "./components/SiteHeader";
import { Footer } from "./components/Footer";
import { User, fetchUserInfo } from "./api";

const NotFound = lazy(() => import("./pages/NotFound"));
const Home = lazy(() => import("./pages/Home"));
const SearchPage = lazy(() => import("./pages/Search"));
const Login = lazy(() => import("./pages/Login"));
const Register = lazy(() => import("./pages/Register"));
const Editor = lazy(() => import("./pages/Editor"));
const Privacy = lazy(() => import("./pages/Privacy"));
const Equation = lazy(() => import("./pages/Equation"));

const index = () => {
    const [loggedInUser, { mutate, refetch }] = createResource(fetchUserInfo);

    const [path, setPath] = createSignal(window.location.pathname);

    const state = new StateManager(path, setPath, loggedInUser, refetch);

    return (
        <>
            <SiteHeader state={state} />
            <div class="page-container">
                <Switch fallback={<NotFound state={state} />}>
                    <Match when={pathMatches("/", path())}>
                        <Home state={state} />
                    </Match>
                    <Match when={pathMatches("/login", path())}>
                        <Login state={state} />
                    </Match>
                    <Match when={pathMatches("/register", path())}>
                        <Register state={state} />
                    </Match>
                    <Match when={pathMatches("/editor", path())}>
                        <Editor state={state} />
                    </Match>
                    <Match when={pathMatches("/privacy", path())}>
                        <Privacy />
                    </Match>
                    <Match when={pathMatches("/search/:query", path())}>
                        <SearchPage state={state} />
                    </Match>
                    <Match when={pathMatches("/equation/:id/:title", path())}>
                        <Equation state={state} />
                    </Match>
                </Switch>
            </div>
            <Footer state={state} />
        </>
    );
};

render(index, document.getElementById("root") as Node);
