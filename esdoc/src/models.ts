import { Component } from "solid-js";

export enum EsComponentContentType {
    Title = "Title",
    Text = "Text",
    Image = "Image",
    Code = "Code",
    Math = "Math",
}

export type EsComponentContentValue = string;

export interface EsExportedComponent {
    content_type: EsComponentContentType;
    value: EsComponentContentValue;
}

export interface EsComponent {
    toExportedObject(): EsExportedComponent;
    toHyperComponent(): Component;
}
