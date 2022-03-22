import { Logo } from "../components/Logo";
import { StateManager } from "../StateManager";
import { Component } from "solid-js";
import "../assets/editor.scss";

const Editor: Component<Props> = () => {
    return (
        <div id="equation-editor-container">
            <div id="editor-container">
                <h2>Redigering</h2>
                <textarea id="editor"></textarea>
            </div>
            <div id="preview-container">
                <h2>Forhåndsvisning</h2>
                <div id="preview"></div>
            </div>
        </div>
    );
};

export default Editor;
