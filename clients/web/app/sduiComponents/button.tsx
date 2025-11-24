import { Button } from "@/components/ui/button";

export function SDUIButton({ data }: { data: import("@/app/sdui").ButtonData }) {
  return (
    <Button 
      className={data.className} 
      onClick={() => console.log(`Action triggered: ${data.action}`)}
    >
      {data.text}
    </Button>
  );
}