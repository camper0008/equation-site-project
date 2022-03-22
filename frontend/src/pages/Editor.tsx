import { Logo } from "../components/Logo";
import { StateManager } from "../StateManager";
import { Component, createSignal } from "solid-js";
import { EsParser } from "esdoc";
import "../assets/editor_page.scss";

const exampleText = (): string => {
    return `Her vises der noget tekst.

Dette er et billede:

image""https://upload.wikimedia.org/wikipedia/commons/thumb/d/d8/Example.tiff/lossless-page1-128px-Example.tiff.png""

Her er noget matematik:
math""a^2 + b^2 = c^2""

og her er noget kode:

code""fn main() {
    println!("Hello, world!")
}""`;
};

const exampleHtml = (): string => {
    const doc = new EsParser(exampleText()).parse();
    return doc.toHtml();
};

const Editor: Component<Props> = () => {
    const [previewHtml, setPreviewHtml] = createSignal(exampleHtml());

    const parse = () => {
        const editor = document.getElementById("editor") as HTMLInputElement;

        const doc = new EsParser(editor.value).parse();
        setPreviewHtml(doc.toHtml());
    };

    return (
        <>
            <div id="equation-editor-container">
                <div id="editor-container">
                    <h2>Redigering</h2>
                    <div class="equation-toolbar">
                        <p>Objektindsætter</p>
                        <button data-object-type="math">Matematik</button>
                        <button data-object-type="image">Billede</button>
                        <button data-object-type="code">Kode</button>
                    </div>
                    <textarea id="editor" onInput={parse}>
                        {exampleText()}
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
                        />
                        <button>Gem formel</button>
                    </div>
                    <div id="preview" innerHTML={previewHtml()}></div>
                </div>
            </div>
        </>
    );
};

export default Editor;
