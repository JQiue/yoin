import { storage } from "@/shared/helper";
import { http } from "./client";
import type { Login } from "./types";

export const login = (
  email: string,
  password: string,
) => {
  storage.set("yoin:token", "");
  return http.post<Login>("/api/auth/login", {
      email,
      password,
  });
};

export const register = (email: string, password: string, nickname: string, website: string ) => {
  return http.post("/api/auth/register", {
      email,
      password,
      nickname,
      website,
  });
}
