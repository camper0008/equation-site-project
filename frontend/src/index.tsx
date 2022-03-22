import "./assets/root.scss";

import { lazy, createSignal, Match, Switch, createResource } from "solid-js";
import { render } from 'solid-js/web';
import { pathMatches } from "./utils";
import { StateManager } from "./StateManager";
import { SiteHeader } from "./components/SiteHeader";
import { User, fetchUserInfo } from "./api";

const NotFound = lazy(() => import("./pages/NotFound"));
const Home = lazy(() => import("./pages/Home"));
const SearchPage = lazy(() => import("./pages/Search"));
const Login = lazy(() => import("./pages/Login"));
const Register = lazy(() => import("./pages/Register"));
const Editor = lazy(() => import("./pages/Editor"));

const index = () => {
    const [loggedInUser, { mutate, refetch}] = createResource(fetchUserInfo);

    const [path, setPath] = createSignal(window.location.pathname);

    const state = new StateManager(path, setPath, loggedInUser, refetch);

    return (<>
    <SiteHeader state={state}/>
    <Switch fallback={<NotFound state={state} />}>
      <Match when={pathMatches("/", path())}>
        <Home state={state}/>
      </Match>
      <Match when={pathMatches("/login", path())}>
        <Login state={state}/>
      </Match>
      <Match when={pathMatches("/register", path())}>
        <Register state={state}/>
      </Match>
      <Match when={pathMatches("/editor", path())}>
        <Editor state={state}/>
      </Match>
      <Match when={pathMatches("/search/:query", path())}>
        <SearchPage state={state}/>
      </Match>
    </Switch>
    </>);
}

render(index, document.getElementById('root') as Node);
