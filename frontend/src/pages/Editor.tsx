import { Logo } from "../components/Logo";
import { StateManager } from "../StateManager";
import { Component, createSignal } from "solid-js";
import { EsParser } from "esdoc";
import "../assets/editor.scss";

const Editor: Component<Props> = () => {
    const [previewHtml, setPreviewHtml] = createSignal(
        "<p>Her vises der noget tekst.</p>",
    );

    const parse = () => {
        const editor = document.getElementById("editor") as HTMLInputElement;

        const doc = new EsParser(editor.value).parse();
        setPreviewHtml(doc.toHtml());
    };

    return (
        <div id="equation-editor-container">
            <div id="editor-container">
                <h2>Redigering</h2>
                <textarea id="editor" onInput={parse}>
                    Her vises der noget tekst.
                </textarea>
            </div>
            <div id="preview-container">
                <h2>Forhåndsvisning</h2>
                <div id="preview" innerHTML={previewHtml()}></div>
            </div>
        </div>
    );
};

export default Editor;
