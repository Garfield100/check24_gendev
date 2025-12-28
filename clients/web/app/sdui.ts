import { CSSProperties } from "react";

export type SDUIComponentData = StringData | ButtonData | CardData | CarouselData | ImageData;

export interface StringData {
  className?: string;
  style?: CSSProperties;
  kind: "string";
  text: string;
}

export interface ButtonData {
  className?: string;
  style?: CSSProperties;
  kind: "button";
  text: string;
  action?: string; // has to be a string for serialisation
}

export interface CardData {
  className?: string;
  style?: CSSProperties;
  kind: "card";
  header?: {
    title?: string;
    description?: string;
    action?: SDUIComponentData; 
  };
  content?: SDUIComponentData; 
  footer?: SDUIComponentData;
}

export interface CarouselData {
  className?: string;
  style?: CSSProperties;
  kind: "carousel";
  title: string
  items: Array<SDUIComponentData>; 
}

export interface ImageData {
  className?: string;
  style?: CSSProperties;
  kind: "image";
  src: string;
  alt: string;
  width?: number;
  height?: number;
}