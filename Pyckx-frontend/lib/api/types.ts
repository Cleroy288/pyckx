// == API Types - TypeScript interfaces matching backend DTOs // ==

// == Auth Types // ==

/** User data returned by backend (matches Rust AuthResponse) */
export interface User {
  email: string;
  username: string;
  role: string;
}

/** Login request payload (matches Rust LoginRequest) */
export interface LoginRequest {
  email: string;
  password: string;
}

/** Register request payload (matches Rust RegisterRequest) */
export interface RegisterRequest {
  email: string;
  password: string;
  username: string;
  phone_country_code?: string;
  phone_number?: string;
}

// == API Response Types // ==

/** Standard error response from backend (matches Rust ErrorResponse) */
export interface BackendErrorResponse {
  code: string;
  message: string;
  field?: string;
}

/** Legacy API error format (for auth endpoints) */
export interface ApiError {
  success: false;
  error: {
    code: string;
    message: string;
  };
}

/** Auth status check result */
export type AuthStatus =
  | { status: "authenticated"; user: User }
  | { status: "unauthenticated" }
  | { status: "error"; message?: string };

// == DVD Types - MOVED to lib/classes/collection ==
// Import from: import { Dvd, DvdData, CreateDvdInput, UpdateDvdInput } from "@/lib/classes/collection"

/** Response for delete operations (generic) */
export interface DeleteResponse {
  message: string;
  deleted: boolean;
}

// == App Types // ==

/** App data returned by backend (matches Rust AppResponse) */
export interface AppData {
  id: number;
  name: string;
  description: string | null;
  created_at: string;
  updated_at: string;
}

/** Response for app list */
export interface AppListResponse {
  apps: AppData[];
  count: number;
}

/** User app response (when adding an app) */
export interface UserAppResponse {
  id: number;
  app_id: number;
  added_at: string;
}

/** Response for successful user app operations */
export interface UserAppSuccessResponse {
  message: string;
  user_app: UserAppResponse;
}

/** Generic success response */
export interface SuccessResponse {
  message: string;
  success: boolean;
}
