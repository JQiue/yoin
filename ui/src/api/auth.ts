import { http } from "./client";
import type { Login } from "./types";

export const login = (
  email: string,
  password: string,
) => {
  return http.post<Login>("/api/auth/login", {
      email,
      password,
  });
};

export const register = (email: string, password: string, nickname: string, url: string, ) => {
  return http.post("/api/auth/register", {
      email,
      password,
      nickname,
      url,
  });
}