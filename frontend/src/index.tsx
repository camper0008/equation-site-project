import "./root.css";


import { lazy } from "solid-js";
import { render } from 'solid-js/web';
import { SiteHeader } from "./header";
import { Route, Router } from "./router";


const Home = lazy(() => import("./home"));

const index = () => {
    return (<Router>
        <Route pattern="/">
            <Home />
        </Route>
        <Route pattern="/search/:query">
            <Home />
        </Route>
    </Router>);
}

render(index, document.getElementById('root') as Node);
