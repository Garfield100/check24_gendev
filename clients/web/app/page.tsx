"use client";

import { useEffect, useState } from "react";
import { SDUIComponentData, CardData, ButtonData, StringData } from "@/app/sdui";
import { Button } from "@/components/ui/button";
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
} from "@/components/ui/navigation-menu";

export default function Home() {
  const [widgets, setWidgets] = useState<SDUIComponentData[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchData() {
      try {
        // test UUID
        const res = await fetch("/get_recommendations/3fa85f64-5717-4562-b3fc-2c963f66afa6");
        const json = await res.json();
        
        const parsedWidgets = Object.values(json.recs_by_product).map((widgetStr: any) => 
          JSON.parse(widgetStr)
        );

        setWidgets(parsedWidgets as SDUIComponentData[]);
      } catch (e) {
        console.error(e);
      } finally {
        setLoading(false);
      }
    }
    fetchData();
  }, []);

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
          </NavigationMenuList>
        </NavigationMenu>

        {loading ? (
          <p>Loading...</p>
        ) : widgets.length > 0 ? (
            // TODO add titles
          <div className="flex flex-col gap-8 w-full">
            {widgets.map((widget, index) => (
              <SDUIRenderer key={index} data={widget} />
            ))}
          </div>
        ) : (
          <p>No widgets found</p>
        )}
      </main>
    </div>
  );
}