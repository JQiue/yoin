import type { TargetedSubmitEvent } from "preact";
import { useState } from "preact/compat";
import { login, register } from "@/shared/api/auth";
import type { Login } from "@/shared/api/types";
import { storage } from "@/shared/helper";
import { t } from "@/shared/i18n";

export interface LoginInput {
  email: string;
  password: string;
}

export interface NewUserInput extends LoginInput {
  password_confirmation: string;
  nickname: string;
  website: string;
}

export const useAuthForm = (onSuccess?: () => void) => {
  const [submitting, setSubmitting] = useState(false);
  const [submitStatus, setSubmitStatus] = useState({ type: "", msg: "" });
  const [isLoginView, setIsLoginView] = useState(true);
  const [user, setUser] = useState<Login | null>(null);

  const toggleView = () => {
    setSubmitStatus({ type: "", msg: "" });
    setIsLoginView(!isLoginView);
  };

  const handleSubmit = async (
    e: TargetedSubmitEvent<HTMLFormElement>,
    data: NewUserInput | LoginInput,
  ) => {
    e.preventDefault();
    setSubmitting(true);
    setSubmitStatus({ type: "", msg: "" });

    try {
      if (isLoginView) {
        const loginData = data as LoginInput;
        const resData = await login(loginData.email, loginData.password);
        storage.set("yoin:token", resData.data.token);
        storage.set("yoin:user_info", {
          nickname: resData.data.nickname,
          website: resData.data.website,
          email: resData.data.email,
          avatar: resData.data.avatar,
        });
        setUser(resData.data);
        setSubmitStatus({ type: "success", msg: t("auth.loginSuccess") });
        onSuccess?.();
      } else {
        const regData = data as NewUserInput;
        await register(
          regData.email,
          regData.password,
          regData.nickname,
          regData.website,
        );
        setSubmitStatus({ type: "success", msg: t("auth.registerSuccess") });
      }
    } catch (error: unknown) {
      const msg = error instanceof Error ? error.message : String(error);
      setSubmitStatus({ type: "error", msg });
    } finally {
      setSubmitting(false);
    }
  };

  return {
    isLoginView,
    submitting,
    submitStatus,
    toggleView,
    handleSubmit,
    setSubmitStatus,
    user,
  };
};
