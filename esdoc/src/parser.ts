import { EsCode, EsImage, EsMath, EsText, EsTitle } from "./components";
import { EsComponent } from "./models";
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
    private uninterruptedNewlines = 0;

    private text: string;

    constructor(text: string) {
        this.text = text;
    }

    public parse(): EsDocument {
        this.components = [];
        for (const char of this.text) this.parseChar(char);
        this.endTextComponent(EsParserStates.TEXT);
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
            if (this.lastWord.length > 1000)
                this.lastWord = this.lastWord.at(-1);
            this.lastCharWasWordChar = true;
        } else {
            this.lastCharWasWordChar = false;
        }
        if (char === "\n") {
            this.uninterruptedNewlines++;
            if (this.uninterruptedNewlines >= 2) {
                this.uninterruptedNewlines = 0;
                return this.endTextComponent(EsParserStates.TEXT);
            }
        } else if (/\S/.test(char)) {
            this.uninterruptedNewlines = 0;
        }
        if (char == '"') this.quoteDepth++;
        else this.quoteDepth = 0;
        if (this.quoteDepth === 2)
            return this.endTextComponent(EsParserStates.DIRECTIVE);
        else this.textBuffer += char;
    }

    private endTextComponent(newState: EsParserStates) {
        if (/\S/.test(this.textBuffer)) {
            this.components.push(
                new EsText(
                    this.textBuffer.replace(
                        new RegExp(`${this.lastWord}\\s*"$`),
                        "",
                    ),
                ),
            );
        }
        this.textBuffer = "";
        this.quoteDepth = 0;
        this.state = newState;
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
                return new EsImage(text);
            case "math":
                return new EsMath(text);
            case "code":
                return new EsCode(text);
            default:
                return new EsText(text);
        }
    }
}
