
import { SDUIComponentData, CardData, ButtonData, StringData } from "@/app/sdui"
import { Button } from "@/components/ui/button"
import { SDUIRenderer } from "./sduiComponents/renderer";

import {
  NavigationMenu,
  NavigationMenuContent,
  NavigationMenuIndicator,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  NavigationMenuTrigger,
  NavigationMenuViewport,
} from "@/components/ui/navigation-menu"

// const testSDUI: StringData = {
//   kind: "string",
//   className: "",
//   text: "Test string"
// }

const sdui_data = fetch("get_recommendations/3fa85f64-5717-4562-b3fc-2c963f66afa6");
console.log(sdui_data);

export default function Home() {
  return (
    <div className="flex min-h-screen justify-center bg-zinc-50 font-sans dark:bg-black">
      <main className="min-h-screen w-full max-w-4xl flex-col justify-start gap-8 pt-4 pb-16 px-16 bg-white dark:bg-black">
        <NavigationMenu className="pb-6">
          <NavigationMenuList>
            <NavigationMenuItem>
              <NavigationMenuTrigger>Item One</NavigationMenuTrigger>
              <NavigationMenuContent>
                <NavigationMenuLink>Link</NavigationMenuLink>
              </NavigationMenuContent>
            </NavigationMenuItem>

            <NavigationMenuItem>
              {/* <NavigationMenuLink>EEpy</NavigationMenuLink> */}
            </NavigationMenuItem>


          </NavigationMenuList>
        </NavigationMenu>
        

        {/* <SDUIRenderer data={sdui_data}></SDUIRenderer> */}

      </main>
    </div>
  );
}
