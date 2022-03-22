import {
    EsCode,
    EsComponent,
    EsImage,
    EsMath,
    EsText,
    EsTitle,
} from "./components";
import { EsDocument } from "./document";

export const enum EsParserStates {
    TEXT,
    DIRECTIVE,
}

export class EsParser {
    private state: EsParserStates = EsParserStates.TEXT;
    private components: EsComponent[] = [];
    private lastWord = "";
    private lastCharWasWordChar = false;
    private quoteDepth = 0;
    private textBuffer = "";

    constructor(public text: string) {}

    public parse(): EsDocument {
        this.components = [];
        for (const char of this.text) this.parseChar(char);
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
            if (!this.lastCharWasWordChar) this.lastWord = "";
            this.lastWord += char;
            this.lastCharWasWordChar = true;
        } else {
            this.lastCharWasWordChar = false;
        }
        if (char == '"') this.quoteDepth++;
        else this.quoteDepth = 0;
        if (this.quoteDepth === 2) return this.endTextComponent();
        else this.textBuffer += char;
    }

    private endTextComponent() {
        this.components.push(
            new EsText(
                this.textBuffer.replace(
                    new RegExp(`${this.lastWord}\\s*"$`),
                    "",
                ),
            ),
        );
        this.textBuffer = "";
        this.quoteDepth = 0;
        this.state = EsParserStates.DIRECTIVE;
    }

    private walkDirective(char: string) {
        if (char == '"') this.quoteDepth++;
        else this.quoteDepth = 0;
        if (this.quoteDepth === 2) return this.endDirective();
        else this.textBuffer += char;
    }

    private endDirective() {
        this.components.push(this.makeDirective());
        this.textBuffer = "";
        this.lastWord = "";
        this.quoteDepth = 0;
        this.state = EsParserStates.TEXT;
    }

    private makeDirective(): EsComponent {
        const text = this.textBuffer.replace(/"+$/, "");
        switch (this.lastWord) {
            case "title":
                return new EsTitle(text);
            case "image":
                return new EsImage(text, "");
            case "math":
                return new EsMath(text);
            case "code":
                return new EsCode(text);
            default:
                return new EsText(text);
        }
    }
}
