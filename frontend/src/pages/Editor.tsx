import { Logo } from "../components/Logo";
import { StateManager } from "../StateManager";
import {
    Component,
    createSignal,
    createEffect,
    createResource,
    onMount,
} from "solid-js";
import { EsParser, EsDocument } from "esdoc";
import { apiUrl, post, get } from "../api";
import { urlParams } from "../utils";
import "../assets/editor-page.scss";

const defaultText = (): string => {
    return `Her vises der noget tekst.

Dette er et billede:

image""https://upload.wikimedia.org/wikipedia/commons/thumb/d/d8/Example.tiff/lossless-page1-128px-Example.tiff.png""

Her er noget matematik:
math""a^2 + b^2 = c^2""

title""Her er en undertitel""

og her er noget kode:

code""fn main() {
    println!("Hello, world!")
}""`;
};

interface Props {
    state: StateManager;
}

interface Post {
    id: string | null;
    title: string;
    content: string;
}

const Editor: Component<Props> = ({ state }) => {
    let titleElement: HTMLInputElement;
    let editorElement: HTMLTextAreaElement;
    const [previewHtml, setPreviewHtml] = createSignal(<></>);

    const parse = () => {
        const doc = new EsParser(editorElement!.value).parse();
        setPreviewHtml(doc.toHyperComponent() as unknown as string);
    };

    const fetchData = async (): Promise<Post> => {
        if (state.path() === "/editor/new") {
            return {
                id: null,
                title: "",
                content: defaultText(),
            };
        }

        const { id } = urlParams("/editor/:id", state.path());
        const res = await get(apiUrl() + `/equations/one/${id}`);

        if (res.ok && res.equation) {
            const { title, content } = res.equation;
            return {
                id,
                title,
                content,
            };
        }

        return {
            id: null,
            title: "",
            content: defaultText(),
        };
    };

    const [existingEquation] = createResource(fetchData);

    const saveEquation = () => {
        let endpoint = "";
        if (state.path() === "/editor/new") {
            endpoint = "/equations/create";
        } else {
            const { id } = urlParams("/editor/:id", state.path());
            endpoint = `/equations/edit/${id}`;
        }

        post(
            apiUrl() + endpoint,
            JSON.stringify({
                title: titleElement!.value,
                content: editorElement!.value,
            }),
        );
    };

    createEffect(() => {
        existingEquation();
        parse();
    });

    return (
        <div id="equation-editor-container">
            <div id="editor-container">
                <h2>Redigering</h2>
                <div class="equation-toolbar">
                    <p>Objektindsætter</p>
                    <button>Test</button>
                </div>
                <textarea
                    id="editor"
                    onInput={parse}
                    ref={editorElement!}
                    disabled={
                        existingEquation.loading || existingEquation.error
                    }
                >
                    {!existingEquation.loading
                        ? existingEquation()?.content
                        : ""}
                </textarea>
            </div>
            <div id="preview-container">
                <h2>Forhåndsvisning</h2>
                <div class="equation-toolbar">
                    <p>Formelindstillinger</p>
                    <label for="equation-title-input">Titel:</label>
                    <input
                        id="equation-title-input"
                        placeholder="Formel titel"
                        value={
                            !existingEquation.loading
                                ? existingEquation()?.title
                                : ""
                        }
                        ref={titleElement!}
                    />
                    <button onClick={saveEquation}>Gem formel</button>
                </div>
                <div id="preview">{previewHtml()}</div>
            </div>
        </div>
    );
};

export default Editor;
