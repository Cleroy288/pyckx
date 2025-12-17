"use client"

import { createContext, useContext, useState, useEffect, useCallback, useMemo, useTransition, type ReactNode } from "react"
import { useRouter, usePathname } from "next/navigation"
import { type App, getAppById, availableApps } from "@/lib/apps-data"
import { getUserApps, addUserApp, removeUserApp } from "@/lib/api/user-apps"
import { useAuth } from "@/lib/auth-context"

// == Types // ==
type Page = { type: "dashboard" } | { type: "app"; appId: string }

interface UserAppsContextType {
  userAppIds: string[]
  loading: boolean
  error: string | null
  addApp: (appId: string) => Promise<void>
  removeApp: (appId: string) => Promise<void>
  hasApp: (appId: string) => boolean
  getUserAppsData: () => App[]
  refreshApps: () => Promise<void>
  // Navigation
  currentPage: Page
  navigablePages: Page[]
  currentPageIndex: number
  navigatePrev: () => void
  navigateNext: () => void
  navigateTo: (page: Page) => void
  getCurrentPageName: () => string
  canNavigatePrev: boolean
  canNavigateNext: boolean
}

const UserAppsContext = createContext<UserAppsContextType | undefined>(undefined)

// == Helper: Map backend app name to frontend app ID // ==
function backendNameToFrontendId(backendName: string): string | undefined {
  // Backend stores app names like "collection", frontend uses same IDs
  const app = availableApps.find(a => a.name.toLowerCase() === backendName.toLowerCase() || a.id === backendName)
  return app?.id
}

function frontendIdToBackendName(frontendId: string): string {
  // Frontend ID is the same as backend name (e.g., "collection")
  const app = getAppById(frontendId)
  return app?.id ?? frontendId
}

export function UserAppsProvider({ children }: { children: ReactNode }) {
  const { authState } = useAuth()
  const isAuthenticated = authState === "authenticated"
  const authLoading = authState === "loading"
  const router = useRouter()
  const pathname = usePathname()
  const [isPending, startTransition] = useTransition()
  
  const [userAppIds, setUserAppIds] = useState<string[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  
  // Derive currentPage from the actual URL pathname
  const currentPage = useMemo<Page>(() => {
    // Check if we're on an app page (e.g., /collection, /intello)
    const appId = pathname?.replace("/", "") || ""
    const app = availableApps.find(a => a.id === appId)
    if (app) {
      return { type: "app", appId: app.id }
    }
    return { type: "dashboard" }
  }, [pathname])

  // == Fetch user apps from backend // ==
  const refreshApps = useCallback(async () => {
    if (!isAuthenticated) {
      setUserAppIds([])
      return
    }

    setLoading(true)
    setError(null)

    try {
      const apps = await getUserApps()
      // Map backend app names to frontend app IDs
      const ids = apps
        .map(app => backendNameToFrontendId(app.name))
        .filter((id): id is string => id !== undefined)
      setUserAppIds(ids)
    } catch (err) {
      console.error("Failed to fetch user apps:", err)
      setError(err instanceof Error ? err.message : "Failed to load apps")
    } finally {
      setLoading(false)
    }
  }, [isAuthenticated])

  // == Load apps on auth change // ==
  useEffect(() => {
    if (!authLoading) {
      refreshApps()
    }
  }, [authLoading, isAuthenticated, refreshApps])

  // == Build navigable pages: Dashboard + user's added apps (memoized) // ==
  const navigablePages = useMemo<Page[]>(() => [
    { type: "dashboard" },
    ...userAppIds.map((appId) => ({ type: "app" as const, appId })),
  ], [userAppIds])

  const currentPageIndex = useMemo(() => navigablePages.findIndex((page) => {
    if (page.type === "dashboard" && currentPage.type === "dashboard") return true
    if (page.type === "app" && currentPage.type === "app" && page.appId === currentPage.appId) return true
    return false
  }), [navigablePages, currentPage])

  const canNavigatePrev = currentPageIndex > 0
  const canNavigateNext = currentPageIndex < navigablePages.length - 1

  // == Add app (calls backend) // ==
  const addApp = async (appId: string) => {
    if (userAppIds.includes(appId)) return

    // Optimistic update
    setUserAppIds((prev) => [...prev, appId])
    setError(null)

    try {
      const backendName = frontendIdToBackendName(appId)
      await addUserApp(backendName)
    } catch (err) {
      // Rollback on error
      setUserAppIds((prev) => prev.filter((id) => id !== appId))
      const message = err instanceof Error ? err.message : "Failed to add app"
      setError(message)
      console.error("Failed to add app:", err)
      throw err
    }
  }

  // == Remove app (calls backend) // ==
  const removeApp = async (appId: string) => {
    if (!userAppIds.includes(appId)) return

    // Optimistic update
    const previousIds = [...userAppIds]
    setUserAppIds((prev) => prev.filter((id) => id !== appId))
    setError(null)

    // If currently on that app, go to dashboard
    if (currentPage.type === "app" && currentPage.appId === appId) {
      router.push("/")
    }

    try {
      const backendName = frontendIdToBackendName(appId)
      await removeUserApp(backendName)
    } catch (err) {
      // Rollback on error
      setUserAppIds(previousIds)
      const message = err instanceof Error ? err.message : "Failed to remove app"
      setError(message)
      console.error("Failed to remove app:", err)
      throw err
    }
  }

  const hasApp = useCallback((appId: string) => userAppIds.includes(appId), [userAppIds])

  // Memoized user apps data to prevent recreating array on every render
  const userAppsData = useMemo(() => {
    return userAppIds.map((id) => getAppById(id)).filter((app): app is App => app !== undefined)
  }, [userAppIds])

  const getUserAppsData = useCallback(() => userAppsData, [userAppsData])

  // Use startTransition for smoother navigation (React 19 concurrent feature)
  const navigatePrev = useCallback(() => {
    if (currentPageIndex > 0) {
      const prevPage = navigablePages[currentPageIndex - 1]
      startTransition(() => {
        if (prevPage.type === "dashboard") {
          router.push("/")
        } else {
          router.push(`/${prevPage.appId}`)
        }
      })
    }
  }, [currentPageIndex, navigablePages, router, startTransition])

  const navigateNext = useCallback(() => {
    if (currentPageIndex < navigablePages.length - 1) {
      const nextPage = navigablePages[currentPageIndex + 1]
      startTransition(() => {
        if (nextPage.type === "dashboard") {
          router.push("/")
        } else {
          router.push(`/${nextPage.appId}`)
        }
      })
    }
  }, [currentPageIndex, navigablePages, router, startTransition])

  const navigateTo = useCallback((page: Page) => {
    startTransition(() => {
      if (page.type === "dashboard") {
        router.push("/")
      } else {
        router.push(`/${page.appId}`)
      }
    })
  }, [router, startTransition])

  const getCurrentPageName = useCallback(() => {
    if (currentPage.type === "dashboard") return "Dashboard"
    const app = getAppById(currentPage.appId)
    return app?.name || "Unknown"
  }, [currentPage])

  // Memoize context value to prevent unnecessary re-renders
  const contextValue = useMemo(() => ({
    userAppIds,
    loading,
    error,
    addApp,
    removeApp,
    hasApp,
    getUserAppsData,
    refreshApps,
    currentPage,
    navigablePages,
    currentPageIndex,
    navigatePrev,
    navigateNext,
    navigateTo,
    getCurrentPageName,
    canNavigatePrev,
    canNavigateNext,
  }), [
    userAppIds,
    loading,
    error,
    addApp,
    removeApp,
    hasApp,
    getUserAppsData,
    refreshApps,
    currentPage,
    navigablePages,
    currentPageIndex,
    navigatePrev,
    navigateNext,
    navigateTo,
    getCurrentPageName,
    canNavigatePrev,
    canNavigateNext,
  ])

  return (
    <UserAppsContext.Provider value={contextValue}>
      {children}
    </UserAppsContext.Provider>
  )
}

export function useUserApps() {
  const context = useContext(UserAppsContext)
  if (context === undefined) {
    throw new Error("useUserApps must be used within a UserAppsProvider")
  }
  return context
}
