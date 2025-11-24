import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "@/components/ui/carousel"

import { SDUIComponentData, CardData, ButtonData } from "@/app/sdui"


function renderSDUI(componentData: SDUIComponentData) {
  // no native pattern matching :(
  switch (componentData.kind) {
    case "string":
      return componentData.text
    
    case "card":
      return renderCard(componentData)

    case "button":
      return renderButton
  }
}


function renderCard(cardData: CardData) {
  return (
    <Card className={cardData.className}>
      {
        cardData.header && (
          <CardHeader>

            {cardData.header.title && (<CardTitle>{cardData.header.title}</CardTitle>)}

            {cardData.header.description && (<CardDescription>{cardData.header.description}</CardDescription>)}

            {cardData.header.action && (<CardAction>{renderSDUI(cardData.header.action)}</CardAction>)}

          </CardHeader>
        )
      }


      {cardData.content && (
        <CardContent>
          {renderSDUI(cardData.content)}
        </CardContent>
      )}

      {cardData.footer && (
        <CardFooter>
          {renderSDUI(cardData.footer)}
        </CardFooter>
      )}

    </Card>
  )
}

function renderButton(buttonData: ButtonData) {
  // TODO
}

export default function Home() {
  return (
    <div className="flex min-h-screen items-center justify-center bg-zinc-50 font-sans dark:bg-black">
      <main className="flex min-h-screen w-full max-w-4xl flex-col items-center justify-between py-32 px-16 bg-white dark:bg-black sm:items-start">
        


      </main>
    </div>
  );
}
