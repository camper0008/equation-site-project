import "./assets/root.scss";

import { lazy, createSignal, Match, Switch } from "solid-js";
import { render } from 'solid-js/web';
import { SiteHeader } from "./SiteHeader";
import { pathMatches } from "./utils";
import { StateManager } from "./StateManager";

const Home = lazy(() => import("./Home"));
const SearchPage = lazy(() => import("./SearchPage"));

const index = () => {

    const [path, setPath] = createSignal(window.location.pathname);

    const state = new StateManager(path, setPath);

    return (
    <Switch fallback={<div>Not Found</div>}>
      <Match when={pathMatches("/", path())}>
        <Home state={state}/>
      </Match>
      <Match when={pathMatches("/search/:query", path())}>
        <SearchPage state={state}/>
      </Match>
    </Switch>
    );
}

render(index, document.getElementById('root') as Node);
