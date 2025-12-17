import type React from "react"
import type { Metadata } from "next"
import { Analytics } from "@vercel/analytics/next"
import { AuthProvider } from "@/lib/auth-context"
import { UserAppsProvider } from "@/lib/user-apps-context"
import { ThemeProvider } from "@/lib/theme-context"
import { PaletteProvider } from "@/lib/palette-context"
import "./globals.css"

export const metadata: Metadata = {
  title: "PYCKX - Your All-in-One Platform",
  description: "Multiple apps in one powerful platform",
  generator: "v0.app",
}

// Script to prevent flash of wrong theme - runs before React hydration
const themeScript = `
  (function() {
    try {
      var theme = localStorage.getItem('pyckx-theme');
      if (theme === 'dark') {
        document.documentElement.classList.add('dark');
      } else {
        document.documentElement.classList.remove('dark');
      }
    } catch (e) {}
  })();
`;

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <head>
        <script dangerouslySetInnerHTML={{ __html: themeScript }} />
      </head>
      <body className="font-sans antialiased">
        <ThemeProvider>
          <PaletteProvider>
            <AuthProvider>
              <UserAppsProvider>{children}</UserAppsProvider>
            </AuthProvider>
          </PaletteProvider>
        </ThemeProvider>
        <Analytics />
      </body>
    </html>
  )
}
