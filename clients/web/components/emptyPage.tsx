"use client";

import { ReactNode } from "react";
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
} from "@/components/ui/navigation-menu";

import Link from "next/link"
import { ThemeToggle } from "./themeToggle";

// 1. Fixed Function Signature and Types
function NavButton({ href, children }: { href: string; children: ReactNode }) {
  return (
    <NavigationMenuItem>
      <NavigationMenuLink asChild>
        <Link
          href={href}
          className="rounded-md p-4 no-underline transition-all duration-200 focus:shadow-xl"
        >
          {children}
        </Link>
      </NavigationMenuLink>
    </NavigationMenuItem>
  )
}

export default function EmptyPage({ children }: { children: ReactNode }) {
  return (
    <div className="flex min-h-screen justify-center font-sans bg-zinc-100 dark:bg-gray-950">
      <main className="min-h-screen w-full max-w-4xl flex-col justify-start gap-8 pt-4 pb-16 px-16 bg-white dark:bg-black">
        <div className="flex w-full items-center justify-between pb-6">

          <NavigationMenu className="rounded-md shadow-md">
            <NavigationMenuList>
              <NavButton href="/">Home</NavButton>
              <NavButton href="/playground">Playground</NavButton>
            </NavigationMenuList>
          </NavigationMenu>

          <ThemeToggle />

        </div>

        {children}
      </main>
    </div>
  );
}