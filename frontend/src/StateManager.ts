import { Accessor, Setter, Resource } from "solid-js";
import { User } from "./api";

export class StateManager {
    constructor(
        public path: Accessor<string>,
        public setPath: Setter<string>,
        public userLoggedIn: Resource<User | null | undefined>,
        public refetchUserLoggedIn: (info?: unknown) => void,
    ) {}

    goto(path: string) {
        window.history.pushState(path, "", path);
        this.setPath(path);
    }
}
