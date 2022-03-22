import { Logo } from "../components/Logo";
import { Component, createSignal } from "solid-js";
import { StateManager } from "../StateManager";
import "../assets/form.scss";
import { API_URL, post } from "../api";

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
    } as fieldIssuesStore);

    const validateFields = (state: StateManager) => {

        const issues: fieldIssuesStore = { username: "", password: "" };

        const usernameElement = document.getElementById("username") as HTMLInputElement;
        const passwordElement = document.getElementById("password") as HTMLInputElement;

        if (usernameElement.value === "") {
            issues.username = "Felt må ikke være tomt"
        }
        if (passwordElement.value === "") {
            issues.password = "Felt må ikke være tomt"
        }

        setFieldIssues(issues);

        if (issues.username === "" && issues.password === "") {
            sendRequest(state);
        }
    }

    const sendRequest = async (state: StateManager) => {
        const usernameElement = document.getElementById("username") as HTMLInputElement;
        const passwordElement = document.getElementById("password") as HTMLInputElement;

        setFetching(true);

        const body = JSON.stringify({
            username: usernameElement.value,
            password: passwordElement.value,
        });

        let res = await post(API_URL + "/users/login", body);

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

    const redirectToRegister = (event: Event) => {
        event.preventDefault();
        state.goto("/register");
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
            <input {...{disabled: fetching() ? true : undefined}} 
            type="password" id="password"
            onKeyDown={ (event: KeyboardEvent) => { if (event.code === "Enter") validateFields(state); } }/>
            
            <button {...{disabled: fetching() ? true : undefined}} 
            id="submit" onClick={() => {validateFields(state)}}>Indsend</button>
            
            <p>Har du ikke en bruger? <a href="/register" onClick={redirectToRegister}>Opret ny bruger</a> i stedet.</p>
        </div>
    </>
}

export default Login;
