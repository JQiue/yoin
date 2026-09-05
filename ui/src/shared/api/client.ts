import { getRuntimeConfig } from "@/config/runtime";
import {
  type Method,
  type RequestConfig,
  type ResData,
  SUCCESS_CODE,
} from "@/shared/api/types";
import { storage } from "@/shared/helper";

const GUEST_ID_HEADER = "x-yoin-guest-id";

function getGuestId() {
  const existing = storage.get("yoin:guest_id");
  if (existing) return existing;
  const created =
    globalThis.crypto?.randomUUID?.().replaceAll("-", "") ??
    `${Date.now().toString(36)}${Math.random().toString(36).slice(2, 12)}`;
  storage.set("yoin:guest_id", created);
  return created;
}

function getApiBase() {
  const configuredBase = getRuntimeConfig().api_base?.trim() || "";
  return configuredBase.replace(/\/+$/, "");
}

function buildUrl(url: string) {
  if (url.startsWith("http")) {
    return new URL(url).toString();
  }
  const base = getApiBase();
  const path = url.startsWith("/") ? url : `/${url}`;
  const target = base ? `${base}${path}` : path;
  return new URL(target, window.location.origin).toString();
}

async function baseRequest<T>(
  url: string,
  method: Method,
  config: RequestConfig = {},
): Promise<ResData<T>> {
  const { params, data, headers, ...rest } = config;
  const fullUrl = new URL(buildUrl(url));

  if (params) {
    Object.keys(params).forEach((key) => {
      const value = params[key];
      if (value == null) return;
      fullUrl.searchParams.append(key, String(value));
    });
  }

  const response = await fetch(fullUrl.toString(), {
    method,
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${storage.get("yoin:token") || ""}`,
      [GUEST_ID_HEADER]: getGuestId(),
      ...headers,
    },
    body: data ? JSON.stringify(data) : undefined,
    ...rest,
  });
  const serverGuestId = response.headers.get(GUEST_ID_HEADER);
  if (serverGuestId) {
    storage.set("yoin:guest_id", serverGuestId);
  }
  if (!response.ok) {
    const errorBody = await response.json().catch(() => ({}));
    throw new Error(errorBody.msg || `网络错误: ${response.status}`);
  }

  const body = (await response.json()) as ResData<T>;
  if (body.code !== SUCCESS_CODE) {
    throw new Error(body.msg || `业务错误: ${body.code}`);
  }
  return body;
}

export const http = {
  get: <T>(url: string, config?: RequestConfig) =>
    baseRequest<T>(url, "GET", config),
  post: <T>(url: string, data?: unknown, config?: RequestConfig) =>
    baseRequest<T>(url, "POST", { ...config, data }),
  put: <T>(url: string, data?: unknown, config?: RequestConfig) =>
    baseRequest<T>(url, "PUT", { ...config, data }),
  patch: <T>(url: string, data?: unknown, config?: RequestConfig) =>
    baseRequest<T>(url, "PATCH", { ...config, data }),
  delete: <T>(url: string, config?: RequestConfig) =>
    baseRequest<T>(url, "DELETE", config),
};
