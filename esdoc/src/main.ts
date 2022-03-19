import { renderToString } from "katex";

interface EsComponent {
    toHtml(): string;
    toJson(): string;
    toMarkdown(): string;
}

export class EsText implements EsComponent {
    constructor(public text: string) { }

    public toHtml(): string {
        const text = this.text
            .trim()
            .replace(/\n[ \t]*\n/, '<br>')
            .replace(/\s+/g, ' ')
        return /*html*/ `<p class="escomponent estext">${text}</p>`;
    }

    public toJson(): string {
        return `{"type":"text","text":"${this.text.replace(/\n/g, '\\n')}"}`;
    }

    public toMarkdown(): string {
        return this.text;
    }
}

export class EsTitle implements EsComponent {
    constructor(public text: string) { }

    public toHtml(): string {
        return /*html*/ `<h2 class="escomponent estitle">${this.text}</h2>`;
    }

    public toJson(): string {
        return `{"type":"title","text":"${this.text}"}`;
    }

    public toMarkdown(): string {
        return `## ${this.text}`;
    }
}

export class EsImage implements EsComponent {
    constructor(public src: string, public alt: string) { }

    public toHtml(): string {
        return /*html*/ `<img class="escomponent esimage" src="${this.src}" alt="${this.alt}">`;
    }

    public toJson(): string {
        return `{"type":"image","src":"${this.src}","alt":"${this.alt}"}`;
    }

    public toMarkdown(): string {
        return `![${this.alt}](${this.src})`;
    }
}

export class EsMath implements EsComponent {
    constructor(public latex: string) { }

    public toHtml(): string {
        return renderToString(this.latex, { throwOnError: true })
            .replace('<span class="katex">', '<span class="escomponent esmath katex">');
    }

    public toJson(): string {
        return `{"type":"math","latex":"${this.latex.replace(/\\/g, '\\\\')}"}`;
    }

    public toMarkdown(): string {
        return this.latex;
    }
}

export class EsCode implements EsComponent {
    constructor(public code: string, public lang?: string) { }

    public toHtml(): string {
        const langClass = this.lang ? `lang-${this.lang}` : '';
        return /*html*/ `<code class="escomponent escode ${langClass}"><pre>${this.code}</pre></code>`;
    }

    public toJson(): string {
        return `{"type":"code","lang":"${this.lang}","code":${JSON.stringify(this.code)}}`;
    }

    public toMarkdown(): string {
        const backticks = '```';
        return `${backticks}${this.lang ?? ''}\n${this.code.trim()}\n${backticks}`;
    }
}

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

export const enum EsParserStates {
    TEXT,
    DIRECTIVE,
}

export class EsParser {
    private state: EsParserStates = EsParserStates.TEXT;
    private components: EsComponent[] = [];
    private lastWord = '';
    private lastCharWasWordChar = false;
    private quoteDepth = 0;
    private textBuffer = '';

    constructor(public text: string) { }

    public parse(): EsDocument {
        this.components = [];
        for (const char of this.text)
            this.parseChar(char);
        this.endTextComponent();
        return new EsDocument(this.components);
    }

    private parseChar(char: string) {
        switch (this.state) {
            case EsParserStates.TEXT:
                return this.walkText(char);
            case EsParserStates.DIRECTIVE:
                return this.walkDirective(char);
        }
    }

    private walkText(char: string) {
        if (/[a-zA-Z]/.test(char)) {
            if (!this.lastCharWasWordChar)
                this.lastWord = '';
            this.lastWord += char;
            this.lastCharWasWordChar = true;
        } else {
            this.lastCharWasWordChar = false;
        }
        if (char == '"')
            this.quoteDepth++;
        else
            this.quoteDepth = 0;
        if (this.quoteDepth === 2)
            return this.endTextComponent();
        else
            this.textBuffer += char;
    }

    private endTextComponent() {
        this.components.push(new EsText(
            this.textBuffer
                .replace(new RegExp(`${this.lastWord}\\s*"$`), '')
        ));
        this.textBuffer = '';
        this.quoteDepth = 0;
        this.state = EsParserStates.DIRECTIVE;
    }

    private walkDirective(char: string) {
        if (char == '"') 
            this.quoteDepth++;
        else
            this.quoteDepth = 0;
        if (this.quoteDepth === 2)
            return this.endDirective();
        else
            this.textBuffer += char;
    }

    private endDirective() {
        this.components.push(this.makeDirective());
        this.textBuffer = '';
        this.lastWord = '';
        this.quoteDepth = 0;
        this.state = EsParserStates.TEXT;
    }

    private makeDirective(): EsComponent {
        const text = this.textBuffer.replace(/"+$/, '');
        switch (this.lastWord) {
            case 'title':
                return new EsTitle(text);
            case 'image':
                return new EsImage(text, '');
            case 'math':
                return new EsMath(text);
            case 'code':
                return new EsCode(text);
            default:
                return new EsText(text);
        }
    }
}

