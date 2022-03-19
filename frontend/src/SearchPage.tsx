import "solid-js";
import { SiteHeader } from "./SiteHeader";

const SearchPage = (props) => {
    return (<>
        <SiteHeader small state={props.state} />
    </>);
}

export default SearchPage;
