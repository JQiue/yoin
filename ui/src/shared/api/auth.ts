import { http } from "@/shared/api/client";
import type { Login } from "@/shared/api/types";
import { storage } from "@/shared/helper";

export type PublicOauthProvider = {
  provider_code: string;
};

export const login = (email: string, password: string) => {
  storage.set("yoin:token", "");
  return http.post<Login>("/api/auth/login", {
    email,
    password,
  });
};

export const fetchPublicOauthProviders = () => {
  return http.get<PublicOauthProvider[]>("/api/auth/oauth/providers");
};

export const startOauth = (provider: string) => {
  return http.get<{ auth_url: string }>(`/api/auth/oauth/${provider}/start`);
};

export const register = (
  email: string,
  password: string,
  nickname: string,
  website: string,
) => {
  return http.post("/api/auth/register", {
    email,
    password,
    nickname,
    website,
  });
};
