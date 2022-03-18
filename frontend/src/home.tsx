import "solid-js";
import { render } from 'solid-js/web';
import { SiteHeader } from "./header";
import "./root.css";

const index = () => {
    return (<>
        <SiteHeader />
    </>);
}

render(index, document.getElementById('root') as Node);
