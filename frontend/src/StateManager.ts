import { Accessor, Setter } from "solid-js";

export class StateManager {
    constructor(
        public path: Accessor<string>,
        public setPath: Setter<string>
    ) {}

    goto(path: string) {
        window.history.pushState(path, "", path)
        this.setPath(path);
    }
}
