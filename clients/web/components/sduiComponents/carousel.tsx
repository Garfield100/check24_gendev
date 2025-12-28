import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "@/components/ui/carousel"
import { SDUIRenderer } from "./renderer";
import { Label } from "@radix-ui/react-label";


export function SDUICarousel({ data }: { data: import("@/app/sdui").CarouselData }) {
  return (
    <Carousel className={data.className} style={data.style}>
      {data.title && (
        <div className="mb-4 flex items-center justify-between px-1">
          <h2 className="text-xl font-semibold tracking-tight text-foreground">
            {data.title}
          </h2>
        </div>
      )}
      <CarouselContent>
        {data.items.map((item, index) => (
          // We assume basic responsive basis classes here, but these could also come from data.className on the item itself if you expand the types
          <CarouselItem key={index} className="md:basis-1/2 lg:basis-1/3 pl-4">
            <div className="p-1 h-full">
              <SDUIRenderer data={item} />
            </div>
          </CarouselItem>
        ))}
      </CarouselContent>
      <CarouselPrevious />
      <CarouselNext />
    </Carousel>
  );
}
