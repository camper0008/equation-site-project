import { render } from "katex";
import { Component, onMount } from "solid-js";
import h from "solid-js/h";

import {
    EsComponent,
    EsComponentContentType,
    EsComponentContentValue,
} from "./models";

export class EsText implements EsComponent {
    constructor(public text: string) {}

    public toHyperComponent(): Component {
        return h("p", this.text);
    }
}

export class EsTitle implements EsComponent {
    constructor(public text: string) {}

    public toHyperComponent(): Component {
        return h("h2", this.text);
    }
}

export class EsImage implements EsComponent {
    constructor(public src: string) {}

    public toHyperComponent(): Component {
        return h("img", { src: this.src });
    }
}

export class EsMath implements EsComponent {
    constructor(public latex: string) {}

    public toHyperComponent(): Component {
        let katexRenderReference: HTMLElement;

        onMount(() => {
            render(this.latex, katexRenderReference, {
                throwOnError: true,
            });
        });

        return h("div", {
            ref: (el: HTMLElement) => (katexRenderReference = el),
        });
    }
}

export class EsCode implements EsComponent {
    constructor(public code: string) {}

    public toHyperComponent(): Component {
        return h("code", h("pre", this.code));
    }
}
