import { EsComponent, EsText, EsTitle, EsImage, EsMath, EsCode } from "./components";

export type EsJsonifiedComponent = {
    type: 'text',
    text: string,
} | {
    type: 'title',
    text: string,
} | {
    type: 'image',
    src: string,
    alt: string,
} | {
    type: 'math',
    latex: string,
} | {
    type: 'code',
    lang: string,
    code: string,
};

export class EsDocument {
    constructor(public components: EsComponent[]) { }

    public toHtml(): string {
        return (this.components.map(c => c.toHtml())).join('');
    }

    public toDiv(): string {
        return /*html*/ `<div class="esdocument">${this.toHtml()}</div>`
    }

    public toJson(): string {
        return `[${(this.components.map(c => c.toJson())).join(',')}]`;
    }

    public toMarkdown(): string {
        return (this.components.map(c => c.toMarkdown())).join('');
    }

    public static fromJson(json: string): EsDocument {
        const safeJson = json
            .replace(/\n/g, '\\n')
            ;
        const jsonComponents = JSON.parse(safeJson) as EsJsonifiedComponent[];
        const components = jsonComponents.map(c => {
            switch (c.type) {
                case 'text':
                    return new EsText(c.text);
                case 'title':
                    return new EsTitle(c.text);
                case 'image':
                    return new EsImage(c.src, c.alt);
                case 'math':
                    return new EsMath(c.latex);
                case 'code':
                    return new EsCode(c.code, c.lang);
            }
        });
        return new EsDocument(components);
    }
}
