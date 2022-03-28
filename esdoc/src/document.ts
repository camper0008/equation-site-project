import { For, Component } from "solid-js";
import { EsComponent, EsComponentContentType } from "./models";
import { EsText, EsTitle, EsImage, EsMath, EsCode } from "./components";
import h from "solid-js/h";

export class EsDocument {
    constructor(public components: EsComponent[]) {}

    public toHyperComponent(): Component[] {
        return this.components.map((c) => c.toHyperComponent());
    }
}
