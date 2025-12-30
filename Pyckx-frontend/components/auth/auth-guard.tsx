"use client";

// == Auth Guard - Protects routes requiring authentication // ==

import { useEffect } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "@/lib/auth-context";

interface AuthGuardProps {
  children: React.ReactNode;
}

export function AuthGuard({ children }: AuthGuardProps) {
  const { authState } = useAuth();
  const router = useRouter();

  console.log("[AUTH-GUARD] Rendering, authState:", authState);

  useEffect(() => {
    console.log("[AUTH-GUARD] useEffect triggered, authState:", authState);
    if (authState === "unauthenticated") {
      console.log("[AUTH-GUARD] User is unauthenticated, redirecting to /login");
      router.push("/login");
    }
  }, [authState, router]);

  // Show loading state while checking auth
  if (authState === "loading") {
    console.log("[AUTH-GUARD] Showing loading spinner");
    return (
      <div className="flex min-h-screen items-center justify-center bg-background">
        <div className="flex flex-col items-center gap-4">
          <div className="h-8 w-8 animate-spin rounded-full border-2 border-primary border-t-transparent" />
          <p className="text-sm text-muted-foreground">Loading...</p>
        </div>
      </div>
    );
  }

  // Don't render children if not authenticated
  if (authState === "unauthenticated") {
    console.log("[AUTH-GUARD] Returning null (unauthenticated)");
    return null;
  }

  console.log("[AUTH-GUARD] Rendering children (authenticated)");
  return <>{children}</>;
}
