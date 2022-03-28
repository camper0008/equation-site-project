import { Component } from "solid-js";

export enum EsComponentContentType {
    Title = "Title",
    Text = "Text",
    Image = "Image",
    Code = "Code",
    Math = "Math",
}

export type EsComponentContentValue = string;

export interface EsComponent {
    toHyperComponent(): Component;
}
