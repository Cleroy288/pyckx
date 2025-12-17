// == User Apps API - Functions for user app management // ==

import { endpoints } from "./config";
import type { AppData, AppListResponse, UserAppSuccessResponse, BackendErrorResponse } from "./types";

// == Helper to extract error message from backend response // ==
function extractErrorMessage(errorBody: unknown, fallback: string): string {
  if (!errorBody || typeof errorBody !== "object") {
    return fallback;
  }
  
  // Backend sends { code, message, field? } directly
  const backendError = errorBody as BackendErrorResponse;
  if (backendError.message) {
    return backendError.message;
  }
  
  return fallback;
}

// == Get User Apps // ==
export async function getUserApps(): Promise<AppData[]> {
  const res = await fetch(endpoints.apps.userApps(), {
    method: "GET",
    credentials: "include",
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to fetch user apps (${res.status})`));
  }

  const data: AppListResponse = await res.json();
  return data.apps;
}

// == Add User App // ==
export async function addUserApp(appName: string): Promise<UserAppSuccessResponse> {
  const res = await fetch(endpoints.apps.userApps(), {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    credentials: "include",
    body: JSON.stringify({ app_name: appName }),
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to add app (${res.status})`));
  }

  return res.json();
}

// == Remove User App // ==
export async function removeUserApp(appName: string): Promise<void> {
  const res = await fetch(`${endpoints.apps.userApps()}/${encodeURIComponent(appName)}`, {
    method: "DELETE",
    credentials: "include",
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to remove app (${res.status})`));
  }
}

// == Get All Available Apps // ==
export async function getAllApps(): Promise<AppData[]> {
  const res = await fetch(endpoints.apps.list(), {
    method: "GET",
    credentials: "include",
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to fetch apps (${res.status})`));
  }

  const data: AppListResponse = await res.json();
  return data.apps;
}
