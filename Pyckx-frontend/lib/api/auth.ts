// == Auth API - Clean functions for authentication // ==

import { endpoints } from "./config";
import type { User, LoginRequest, RegisterRequest, AuthStatus, ApiError } from "./types";

// == Check Auth Status // ==
/** Check if user is authenticated by calling /user/me */
export async function checkAuthStatus(): Promise<AuthStatus> {
  try {
    const res = await fetch(endpoints.user.me(), {
      method: "GET",
      credentials: "include", // Include cookies for session
    });

    if (res.ok) {
      const user: User = await res.json();
      return { status: "authenticated", user };
    }

    // 401/403 = not authenticated
    if (res.status === 401 || res.status === 403) {
      return { status: "unauthenticated" };
    }

    // Server error
    return { status: "error", message: `Server error: ${res.status}` };
  } catch (error) {
    return { status: "error", message: "Network error" };
  }
}

// == Login // ==
/** Login with email and password */
export async function login(data: LoginRequest): Promise<User> {
  const res = await fetch(endpoints.auth.login(), {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    credentials: "include",
    body: JSON.stringify(data),
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    const message = (errorBody as ApiError)?.error?.message ?? `Login failed (${res.status})`;
    throw new Error(message);
  }

  return res.json() as Promise<User>;
}

// == Register // ==
/** Register a new user */
export async function register(data: RegisterRequest): Promise<User> {
  const res = await fetch(endpoints.auth.register(), {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    credentials: "include",
    body: JSON.stringify(data),
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    const message = (errorBody as ApiError)?.error?.message ?? `Registration failed (${res.status})`;
    throw new Error(message);
  }

  return res.json() as Promise<User>;
}

// == Logout // ==
/** Logout and clear session */
export async function logout(): Promise<void> {
  const res = await fetch(endpoints.auth.logout(), {
    method: "POST",
    credentials: "include",
  });

  if (!res.ok) {
    throw new Error(`Logout failed (${res.status})`);
  }
}
