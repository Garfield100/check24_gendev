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

export function SDUICard({ data }: { data: import("@/app/sdui").CardData }) {
  return (
    <Card className={data.className}>
      {data.header && (
        <CardHeader>
          {data.header.title && <CardTitle>{data.header.title}</CardTitle>}
          
          {data.header.description && (
            <CardDescription>{data.header.description}</CardDescription>
          )}
          
          {/* Recursive rendering for the header action */}
          {data.header.action && (
            <div className="mt-2">
              <SDUIRenderer data={data.header.action} />
            </div>
          )}
        </CardHeader>
      )}

      {/* Recursive rendering for content */}
      {data.content && (
        <CardContent>
          <SDUIRenderer data={data.content} />
        </CardContent>
      )}

      {/* Recursive rendering for footer */}
      {data.footer && (
        <CardFooter>
          <SDUIRenderer data={data.footer} />
        </CardFooter>
      )}
    </Card>
  );
}