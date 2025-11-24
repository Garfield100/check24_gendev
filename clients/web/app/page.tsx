
import { SDUIComponentData, CardData, ButtonData, StringData } from "@/app/sdui"
import { Button } from "@/components/ui/button"
import { SDUIRenderer } from "./sduiComponents/renderer";

const testSDUI: StringData = {
  kind: "string",
  className: "",
  text: "Test string"
}

export default function Home() {
  return (
    <div className="flex min-h-screen items-center justify-center bg-zinc-50 font-sans dark:bg-black">
      <main className="flex min-h-screen w-full max-w-4xl flex-col items-center justify-between py-32 px-16 bg-white dark:bg-black sm:items-start">

        <SDUIRenderer data={testSDUI}></SDUIRenderer>

      </main>
    </div>
  );
}
