import { Logo } from "../components/Logo";
import { Component, createSignal } from "solid-js";
import { StateManager } from "../StateManager";
import "../assets/form.scss";
import { API_URL } from "../api";

interface Props {
    state: StateManager,
}

interface fieldIssuesStore {
    username: string,
    password: string,
}

const capitalizeFirstLetter = (msg: string) => {
    return msg.slice(0,1).toUpperCase() + msg.slice(1);
}

const Login: Component<Props> = ({state}) => {

    const [fetching, setFetching] = createSignal(false);

    const [fieldIssues, setFieldIssues] = createSignal({
        username: "",
        password: "",
    } as fieldIssuesStore)

    const validateFields = (state: StateManager) => {
        const issues: fieldIssuesStore = {
            username: "",
            password: "",
        };
    
        const [username, password] = [
            document.getElementById("username") as HTMLInputElement,
            document.getElementById("password") as HTMLInputElement,
        ];

        if (username.value === "") {
            issues.username = "Felt må ikke være tomt"
        }
        if (password.value === "") {
            issues.password = "Felt må ikke være tomt"
        }

        setFieldIssues(issues);

        if (issues.username === "" && issues.password === "") {
            sendRequest(state);
        }
    }

    const sendRequest = async (state: StateManager) => {
        setFetching(true);

        const [username, password] = [
            document.getElementById("username") as HTMLInputElement,
            document.getElementById("password") as HTMLInputElement,
        ];

        const body = JSON.stringify({
            username: username.value,
            password: password.value,
        });

        let res = await (await fetch(API_URL + "/users/login", {
            body,
            method: "POST",
            headers: new Headers({"Content-Type": "application/json"}),
            credentials: "include" // TODO: remove if api and frontend is on same site
        })).json();

        setFetching(false);

        if (!res.ok) {
            setFieldIssues({
                username: capitalizeFirstLetter(res.msg),
                password: capitalizeFirstLetter(res.msg),
            })
        } else {
            state.goto("/");
        }
    }

    return <>
        <Logo state={state} />
        <div class="form" aria-labelledby="form-title">
        <h2 id="form-title">Login</h2>
            <p id="username-error" class="error">{fieldIssues().username}</p>
            <label for="username">Brugernavn</label>
            <input {...{disabled: fetching() ? true : undefined}} id="username"/>
            <p id="password-error" class="error">{fieldIssues().password}</p>
            <label for="password">Adgangskode</label>
            <input {...{disabled: fetching() ? true : undefined}} type="password" id="password"/>
            <button {...{disabled: fetching() ? true : undefined}} 
            id="submit" onClick={() => {validateFields(state)}}>Indsend</button>
        </div>
    </>
}

export default Login;
