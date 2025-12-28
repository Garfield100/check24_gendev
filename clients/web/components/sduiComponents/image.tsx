import { ImageData } from "@/app/sdui";
import { cn } from "@/lib/utils";
import Image from "next/image"

export function SDUIImage({ data }: { data: ImageData }) {
    return (
        <img
            src={data.src}
            alt={data.alt || "Image"}
            className={cn("w-full object-cover rounded-md aspect-[4/3]", data.className)}
            style={{
                height: 220,
                ...data.style
            }}
        ></img>
        // <Image
        //     src={data.src}
        //     alt={data.alt || "Image"}
        //     width={data.width}
        //     height={data.height}
        //     className={cn("max-w-full object-cover rounded-md", data.className)}
        //     style={data.style}
        // ></Image>
    );
}