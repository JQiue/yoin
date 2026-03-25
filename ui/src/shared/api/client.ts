import type { Method, RequestConfig, ResData } from "@/shared/api/types";
import { storage } from "@/shared/helper";
import { useConfigStore } from "@/store/useConfigStore";

function getApiBase() {
  const configuredBase =
    useConfigStore.getState().config.api_base?.trim() || "";
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
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${storage.get("yoin:token") || ""}`,
      ...headers,
    },
    body: data ? JSON.stringify(data) : undefined,
    ...rest,
  });
  // console.log(response);
  if (!response.ok) {
    const errorBody = await response.json().catch(() => ({}));
    throw new Error(errorBody.msg || `网络错误: ${response.status}`);
  }

  return response.json();
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
