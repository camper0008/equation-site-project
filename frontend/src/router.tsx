import { createSignal, createContext, useContext } from "solid-js";
import { pathMatches } from "./utils";

const RouterContext = createContext();

export function Router(props) {
  const currentPage = window.location.pathname;

  const [page, setPage] = createSignal(props.page || currentPage),
    store = [
      page,
      {
        setPage(page) {
          setPage(() => page);
        },
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
    const [page, setPage] = useRouter();

    if (pathMatches(props.pattern, page()))
        return props.children
    else
        return <></>
}
