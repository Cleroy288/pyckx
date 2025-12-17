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

  useEffect(() => {
    if (authState === "unauthenticated") {
      router.push("/login");
    }
  }, [authState, router]);

  // Show loading state while checking auth
  if (authState === "loading") {
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
    return null;
  }

  return <>{children}</>;
}
