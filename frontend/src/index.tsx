import "./assets/root.scss";

import { lazy } from "solid-js";
import { render } from 'solid-js/web';
import { SiteHeader } from "./header";
import { Route, Router } from "./router";

const Home = lazy(() => import("./home"));
const SearchPage = lazy(() => import("./searchPage"));

const index = () => {
    return (

    /* TODO: look into this
    <Switch fallback={<div>Not Found</div>}>
      <Match when={state.path === "home"}>
        <Home />
      </Match>
      <Match when={state.route === "settings"}>
        <SearchPage />
      </Match>
    </Switch>
    */

    <Router>
        <Route pattern="/">
            <Home />
        </Route>
        <Route pattern="/search/:query">
            <SearchPage />
        </Route>
    </Router>);
}

render(index, document.getElementById('root') as Node);
