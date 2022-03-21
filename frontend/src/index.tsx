import "./assets/root.scss";

import { lazy, createSignal, Match, Switch } from "solid-js";
import { render } from 'solid-js/web';
import { pathMatches } from "./utils";
import { StateManager } from "./StateManager";

const NotFound = lazy(() => import("./pages/NotFound"));
const Home = lazy(() => import("./pages/Home"));
const SearchPage = lazy(() => import("./pages/Search"));
const Login = lazy(() => import("./pages/Login"));

const index = () => {

    const [path, setPath] = createSignal(window.location.pathname);

    const state = new StateManager(path, setPath);

    return (
    <Switch fallback={<NotFound state={state} />}>
      <Match when={pathMatches("/", path())}>
        <Home state={state}/>
      </Match>
      <Match when={pathMatches("/login", path())}>
        <Login state={state}/>
      </Match>
      <Match when={pathMatches("/search/:query", path())}>
        <SearchPage state={state}/>
      </Match>
    </Switch>
    );
}

render(index, document.getElementById('root') as Node);
