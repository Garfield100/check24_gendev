"use client";

import { useEffect, useState } from "react";
import { SDUIComponentData } from "@/app/sdui";
import { SDUIRenderer } from "../components/sduiComponents/renderer";
import EmptyPage from "../components/emptyPage";

export default function Home() {
  const [widgets, setWidgets] = useState<SDUIComponentData[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchData() {
      try {
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
    <EmptyPage>
      {loading ? (
        <p>Loading...</p>
      ) : widgets.length > 0 ? (
        <div className="flex flex-col gap-8 w-full">
          {widgets.map((widget, index) => (
            <SDUIRenderer key={index} data={widget} />
          ))}
        </div>
      ) : (
        <p>No widgets found</p>
      )}
    </EmptyPage>
  );
}