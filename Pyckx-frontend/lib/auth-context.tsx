"use client";

// == Auth Context - Global authentication state management // ==

import { createContext, useContext, useState, useEffect, useCallback, useMemo, type ReactNode } from "react";
import { useRouter } from "next/navigation";
import type { User } from "./api/types";
import { checkAuthStatus, login as apiLogin, register as apiRegister, logout as apiLogout } from "./api/auth";
import type { LoginRequest, RegisterRequest } from "./api/types";

// == Types // ==
type AuthState = "loading" | "authenticated" | "unauthenticated";

interface AuthContextType {
  user: User | null;
  authState: AuthState;
  login: (data: LoginRequest) => Promise<void>;
  register: (data: RegisterRequest) => Promise<void>;
  logout: () => Promise<void>;
  refreshAuth: () => Promise<void>;
}

// == Context // ==
const AuthContext = createContext<AuthContextType | undefined>(undefined);

// == Provider // ==
export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [authState, setAuthState] = useState<AuthState>("loading");
  const router = useRouter();

  // Check auth status on mount
  const refreshAuth = useCallback(async () => {
    setAuthState("loading");
    const result = await checkAuthStatus();

    if (result.status === "authenticated") {
      setUser(result.user);
      setAuthState("authenticated");
    } else {
      setUser(null);
      setAuthState("unauthenticated");
    }
  }, []);

  useEffect(() => {
    refreshAuth();
  }, [refreshAuth]);

  // Login handler
  const login = useCallback(async (data: LoginRequest) => {
    const loggedInUser = await apiLogin(data);
    setUser(loggedInUser);
    setAuthState("authenticated");
    router.push("/");
  }, [router]);

  // Register handler
  const register = useCallback(async (data: RegisterRequest) => {
    const newUser = await apiRegister(data);
    setUser(newUser);
    setAuthState("authenticated");
    router.push("/");
  }, [router]);

  // Logout handler
  const logout = useCallback(async () => {
    await apiLogout();
    setUser(null);
    setAuthState("unauthenticated");
    router.push("/login");
  }, [router]);

  // Memoize context value to prevent unnecessary re-renders
  const contextValue = useMemo(() => ({
    user,
    authState,
    login,
    register,
    logout,
    refreshAuth,
  }), [user, authState, login, register, logout, refreshAuth]);

  return (
    <AuthContext.Provider value={contextValue}>
      {children}
    </AuthContext.Provider>
  );
}

// == Hook // ==
export function useAuth() {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
}
