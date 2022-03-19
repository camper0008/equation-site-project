export class StateManager {
    constructor(getPath: () => string, setPath: (string) => void) {
        this.path = getPath;
        this.setPath = setPath;
    }

    goto(path: string) {
        window.history.pushState(path, "", path)
        this.setPath(path);
    }
}
