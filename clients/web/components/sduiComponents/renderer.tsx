import { SDUIComponentData } from "../../app/sdui";
import { SDUIButton } from "./button";
import { SDUICard } from "./card";
import { SDUICarousel } from "./carousel";
import { SDUIImage } from "./image";
import { SDUIString } from "./string";

export function SDUIRenderer({ data }: { data: SDUIComponentData }) {
  switch (data.kind) {
    case "string":
      return <SDUIString data={data} />;
    case "button":
      return <SDUIButton data={data} />;
    case "card":
      return <SDUICard data={data} />;
    case "carousel":
      return <SDUICarousel data={data} />;
    case "image":
      return <SDUIImage data={data} />;
    
    default:
      console.error("Unknown SDUI kind", data);
      return null;
  }
}
