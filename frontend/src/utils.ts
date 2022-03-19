export const urlParams = (pattern: string, path: string) => {
    const params = {};

    const splitPattern = pattern.split("/");
    const splitPath = path.split("/");

    if (splitPath.length != splitPattern.length) {
        console.log("unexpected error occured during url params fetching");
        return params;
    }

    for (let i = 0; i < splitPattern.length; i++) {
        if (splitPattern[i][0] == ":") {
            const paramName = splitPattern[i].slice(1);
            params[paramName] = splitPath[i];
        }
    }
    return params;
}

export const pathMatches = (pattern: string, path: string) => {
    const splitPattern = pattern.split("/");
    const splitPath = path.split("/");

    console.log(pattern, path);

    if (splitPath.length != splitPattern.length)
        return false;

    for (let i = 0; i < splitPattern.length; i++) {
        if (splitPattern[i] != splitPath[i] && splitPattern[i][0] != ":")
            return false;
    }

    return true;
}
