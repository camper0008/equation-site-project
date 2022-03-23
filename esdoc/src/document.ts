import { For, Component } from "solid-js";
import {
    EsComponent,
    EsExportedComponent,
    EsComponentContentType,
} from "./models";
import { EsText, EsTitle, EsImage, EsMath, EsCode } from "./components";
import h from "solid-js/h";

export class EsDocument {
    constructor(public components: EsComponent[]) {}

    public toExportedObject(): EsExportedComponent[] {
        return this.components.map((c) => c.toExportedObject());
    }

    public toHyperComponent(): Component[] {
        return this.components.map((c) => c.toHyperComponent());
    }

    public static fromExportedComponents(
        exportedComponents: EsExportedComponent[],
    ): EsDocument {
        const components = exportedComponents.map((c) => {
            switch (c.content_type) {
                case EsComponentContentType.Text:
                    return new EsText(c.value);
                case EsComponentContentType.Title:
                    return new EsTitle(c.value);
                case EsComponentContentType.Image:
                    return new EsImage(c.value);
                case EsComponentContentType.Math:
                    return new EsMath(c.value);
                case EsComponentContentType.Code:
                    return new EsCode(c.value);
                default:
                    return new EsText(c.value);
            }
        });
        return new EsDocument(components);
    }
}
