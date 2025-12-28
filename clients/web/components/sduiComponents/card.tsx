import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { SDUIRenderer } from "./renderer";
import { cn } from "@/lib/utils"

export function SDUICard({ data }: { data: import("@/app/sdui").CardData }) {
  return (
    <Card className={cn("h-full", data.className)} style={data.style}>
      {data.header && (
        <CardHeader>
          {data.header.title && <CardTitle className="line-clamp-2">{data.header.title}</CardTitle>}

          {data.header.description && (
            <CardDescription className="line-clamp-2">{data.header.description}</CardDescription>
          )}

          {data.header.action && (
            <div className="mt-2">
              <SDUIRenderer data={data.header.action} />
            </div>
          )}
        </CardHeader>
      )}

      {data.content && (
        <CardContent className="flex-1">
          <SDUIRenderer data={data.content} />
        </CardContent>
      )}

      {data.footer && (
        <CardFooter>
          <SDUIRenderer data={data.footer} />
        </CardFooter>
      )}
    </Card>
  );
}