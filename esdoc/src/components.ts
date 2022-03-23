import { renderToString } from "katex";

export interface EsComponent {
    toHtml(): string;
    toJson(): string;
    toRustJson(): string;
    toMarkdown(): string;
}

export class EsText implements EsComponent {
    constructor(public text: string) {}

    public toHtml(): string {
        const text = this.text
            .trim()
            .replace(/\n[ \t]*\n/, "<br>")
            .replace(/\s+/g, " ");
        return /*html*/ `<p class="escomponent estext">${text}</p>`;
    }

    public toJson(): string {
        return `{"type":"text","text":"${this.text.replace(/\n/g, "\\n")}"}`;
    }

    public toRustJson(): string {
        return `{"content_type":"Text","text":"${this.text.replace(
            /\n/g,
            "\\n",
        )}"}`;
    }

    public toMarkdown(): string {
        return this.text;
    }
}

export class EsTitle implements EsComponent {
    constructor(public text: string) {}

    public toHtml(): string {
        return /*html*/ `<h2 class="escomponent estitle">${this.text}</h2>`;
    }

    public toJson(): string {
        return `{"type":"title","text":"${this.text}"}`;
    }

    public toRustJson(): string {
        return `{"type":"title","text":"${this.text}"}`;
    }

    public toMarkdown(): string {
        return `## ${this.text}`;
    }
}

export class EsImage implements EsComponent {
    constructor(public src: string, public alt: string) {}

    public toHtml(): string {
        return /*html*/ `<img class="escomponent esimage" src="${this.src}" alt="${this.alt}">`;
    }

    public toJson(): string {
        return `{"type":"image","src":"${this.src}","alt":"${this.alt}"}`;
    }

    public toRustJson(): string {
        return `{"content_type":"Image","src":"${this.src}","alt":"${this.alt}"}`;
    }

    public toMarkdown(): string {
        return `![${this.alt}](${this.src})`;
    }
}

export class EsMath implements EsComponent {
    constructor(public latex: string) {}

    public toHtml(): string {
        return renderToString(this.latex, { throwOnError: true }).replace(
            '<span class="katex">',
            '<span class="escomponent esmath katex">',
        );
    }

    public toJson(): string {
        return `{"type":"math","latex":"${this.latex.replace(/\\/g, "\\\\")}"}`;
    }

    public toRustJson(): string {
        return `{"content_type":"Math","value":"${this.latex.replace(
            /\\/g,
            "\\\\",
        )}"}`;
    }

    public toMarkdown(): string {
        return this.latex;
    }
}

export class EsCode implements EsComponent {
    constructor(public code: string, public lang?: string) {}

    public toHtml(): string {
        const langClass = this.lang ? `lang-${this.lang}` : "";
        return /*html*/ `<code class="escomponent escode ${langClass}"><pre>${this.code}</pre></code>`;
    }

    public toJson(): string {
        return `{"type":"code","lang":"${this.lang}","code":${JSON.stringify(
            this.code,
        )}}`;
    }

    public toRustJson(): string {
        return `{"content_type":"Code","lang":"${
            this.lang
        }","value":${JSON.stringify(this.code)}}`;
    }

    public toMarkdown(): string {
        const backticks = "```";
        return `${backticks}${
            this.lang ?? ""
        }\n${this.code.trim()}\n${backticks}`;
    }
}
