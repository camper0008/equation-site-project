import "solid-js";
import { SiteHeader } from "./SiteHeader";

const Home = (props) => {
    return (<>
        <SiteHeader state={props.state} />
    </>);
}

export default Home;
