import { createSignal, createContext, useContext, getOwner } from "solid-js";
import { pathMatches } from "./utils";

export const RouterContext = createContext();

export const Router = (props) => {
  const currentPage = window.location.pathname;

  const [page, setPage] = createSignal(currentPage),
    store = [
      page,
      {
        goto(path) { setPage(path) },
      }
    ];

  return (
    <RouterContext.Provider value={store}>
      {props.children}
    </RouterContext.Provider>
  );
}

export const useRouter = () => useContext(RouterContext);

export const Route = (props) => {
    const [page, {goto}] = useRouter();

    if (pathMatches(props.pattern, page()))
        return <>
            {props.children}
            <p>current page is: {page}</p>
        </>
    else
        return <></>
}
