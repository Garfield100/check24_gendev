export type SDUIComponentData = StringData | ButtonData | CardData | CarouselData;

export interface StringData {
  className: string;
  kind: "string";
  text: string;
}

export interface ButtonData {
  className: string;
  kind: "button";
  text: string;
  action?: string; // has to be a string for serialisation
}

export interface CardData {
  className: string;
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
  className: string;
  kind: "carousel";
  items: Array<SDUIComponentData>; 
}