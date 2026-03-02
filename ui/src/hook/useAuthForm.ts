import type { TargetedSubmitEvent } from "preact";
import { storage } from "../helper";
import { login, register } from "../api/auth";
import { useState } from "preact/compat";

export interface LoginInput {
  email: string;
  password: string;
}

export interface NewUserInput extends LoginInput {
  password_confirmation: string;
  nickname: string;
  url: string;
}

export const useAuthForm = (onSuccess?: ()=>void) => {
  const [submitting, setSubmitting] = useState(false);
  const [submitStatus, setSubmitStatus] = useState({ type: "", msg: "" });
  const [isLoginView, setIsLoginView] = useState(true);
  const [user, setUser] = useState<any>(null);

  const toggleView = () => {
    setSubmitStatus({ type: "", msg: "" });
    setIsLoginView(!isLoginView);
  };

  const handleSubmit = async (
    e: TargetedSubmitEvent<HTMLFormElement>,
    data: NewUserInput | LoginInput
  ) => {
    e.preventDefault();
    setSubmitting(true);
    setSubmitStatus({ type: "", msg: "" });

    try {
      if (isLoginView) {
        const loginData = data as LoginInput;
        const resData = await login(loginData.email, loginData.password);
        if (resData.code === 0) {
          storage.set("yoin:token", resData.data.token);
          storage.set("yoin:user_info", {
            nickname: resData.data.nickname,
            website: resData.data.website,
            email: resData.data.email,
          });
          setUser(resData.data);
          setSubmitStatus({ type: "success", msg: "登录成功" });
          if (onSuccess) {
            onSuccess();
          }
        } else {
          setSubmitStatus({ type: "error", msg: resData.msg });
        }
      } else {
        const regData = data as NewUserInput;
        const resData = await register(
          regData.email,
          regData.password,
          regData.nickname,
          regData.url
        );
        if (resData.code === 0) {
          setSubmitStatus({ type: "success", msg: "注册成功，请登录" });
        } else {
          setSubmitStatus({ type: "error", msg: resData.msg });
        }
      }
    } catch (error: any) {
      setSubmitStatus({ type: "error", msg: error.toString() });
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
    user
  };
};