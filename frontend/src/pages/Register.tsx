import { Logo } from "../components/Logo";
import { Component, createSignal } from "solid-js";
import { StateManager } from "../StateManager";
import "../assets/form.scss";
import { apiUrl, post } from "../api";

interface Props {
    state: StateManager;
}

interface fieldIssuesStore {
    username: string;
    password: string;
    gpdr: string;
}

const capitalizeFirstLetter = (msg: string) => {
    return msg.slice(0, 1).toUpperCase() + msg.slice(1);
};

const Register: Component<Props> = ({ state }) => {
    const [fetching, setFetching] = createSignal(false);

    const [fieldIssues, setFieldIssues] = createSignal({
        username: "",
        password: "",
        gpdr: "",
    } as fieldIssuesStore);

    const validateFields = (state: StateManager) => {
        const usernameElement = document.getElementById(
            "username",
        ) as HTMLInputElement;
        const passwordElement = document.getElementById(
            "password",
        ) as HTMLInputElement;
        const gpdrElement = document.getElementById("gpdr") as HTMLInputElement;

        const issues: fieldIssuesStore = {
            username: "",
            password: "",
            gpdr: "",
        };

        if (usernameElement.value === "") {
            issues.username = "Felt må ikke være tomt";
        }
        if (passwordElement.value === "") {
            issues.password = "Felt må ikke være tomt";
        }
        if (gpdrElement.checked === false) {
            issues.gpdr = "Felt må ikke være tomt";
        }

        setFieldIssues(issues);

        if (
            issues.username === "" &&
            issues.password === "" &&
            issues.gpdr === ""
        ) {
            sendRequest(state);
        }
    };

    const sendRequest = async (state: StateManager) => {
        const usernameElement = document.getElementById(
            "username",
        ) as HTMLInputElement;
        const passwordElement = document.getElementById(
            "password",
        ) as HTMLInputElement;

        setFetching(true);

        const body = JSON.stringify({
            username: usernameElement.value,
            password: passwordElement.value,
        });

        let res = await post(apiUrl() + "/users/create", body);

        setFetching(false);

        if (!res.ok) {
            if (res.msg === "invalid username") {
                setFieldIssues({
                    username: capitalizeFirstLetter(
                        "Brugernavn allerede i brug",
                    ),
                    password: "",
                    gpdr: "",
                });
            }
        } else {
            state.goto("/login");
        }
    };

    const redirect = (event: Event, path: string) => {
        event.preventDefault();
        state.goto(path);
    };

    return (
        <>
            <Logo state={state} />
            <div class="form" aria-labelledby="form-title">
                <h2 id="form-title">Opret bruger</h2>
                <p class="error">{fieldIssues().username}</p>
                <label for="username">Brugernavn</label>
                <input
                    {...{ disabled: fetching() ? true : undefined }}
                    id="username"
                />
                <p class="error">{fieldIssues().password}</p>
                <label for="password">Adgangskode</label>
                <input
                    {...{ disabled: fetching() ? true : undefined }}
                    type="password"
                    id="password"
                    onKeyDown={(event: KeyboardEvent) => {
                        if (event.code === "Enter") validateFields(state);
                    }}
                />
                <p class="error">{fieldIssues().gpdr}</p>
                <div>
                    <input type="checkbox" id="gpdr" />{" "}
                    <label for="gpdr">
                        Ved at sætte kryds erkender jeg, at jeg har læst{" "}
                        <a
                            href="/privacy"
                            onClick={(event: Event) =>
                                redirect(event, "/privacy")
                            }
                        >
                            privatlivspolitikken
                        </a>{" "}
                        og giver samtykke.
                    </label>
                </div>
                <button
                    {...{ disabled: fetching() ? true : undefined }}
                    id="submit"
                    onClick={() => {
                        validateFields(state);
                    }}
                >
                    Indsend
                </button>
                <p>
                    Har du allerede en bruger?{" "}
                    <a
                        href="/login"
                        onClick={(event: Event) => redirect(event, "/login")}
                    >
                        Login
                    </a>{" "}
                    i stedet.
                </p>
            </div>
        </>
    );
};

export default Register;
