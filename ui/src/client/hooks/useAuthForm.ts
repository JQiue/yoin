import type { TargetedSubmitEvent } from "preact";
import { useCallback, useEffect, useState } from "preact/compat";
import {
  fetchPublicOauthProviders,
  login,
  type PublicOauthProvider,
  register,
  startOauth,
} from "@/shared/api/auth";
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

function persistLogin(user: Login) {
  storage.set("yoin:token", user.token);
  storage.set("yoin:user_info", {
    nickname: user.nickname,
    website: user.website,
    email: user.email,
    avatar: user.avatar,
  });
}

function isOauthLoginPayload(value: unknown): value is Login {
  if (!value || typeof value !== "object") return false;
  const payload = value as Record<string, unknown>;
  return (
    typeof payload.token === "string" &&
    payload.token.length > 0 &&
    typeof payload.nickname === "string" &&
    typeof payload.email === "string"
  );
}

export const useAuthForm = (onSuccess?: () => void) => {
  const [submitting, setSubmitting] = useState(false);
  const [submitStatus, setSubmitStatus] = useState({ type: "", msg: "" });
  const [isLoginView, setIsLoginView] = useState(true);
  const [user, setUser] = useState<Login | null>(null);
  const [oauthProviders, setOauthProviders] = useState<PublicOauthProvider[]>(
    [],
  );

  const toggleView = () => {
    setSubmitStatus({ type: "", msg: "" });
    setIsLoginView(!isLoginView);
  };

  useEffect(() => {
    let alive = true;
    fetchPublicOauthProviders()
      .then((res) => {
        if (!alive) return;
        setOauthProviders(res.data);
      })
      .catch(() => {
        if (!alive) return;
        setOauthProviders([]);
      });
    return () => {
      alive = false;
    };
  }, []);

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
        persistLogin(resData.data);
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

  const handleOauth = useCallback(
    async (provider: string) => {
      setSubmitting(true);
      setSubmitStatus({ type: "", msg: "" });
      try {
        const started = await startOauth(provider);
        const popup = window.open(
          started.data.auth_url,
          "yoin-oauth",
          "width=600,height=720",
        );
        if (!popup) {
          setSubmitStatus({
            type: "error",
            msg: t("auth.oauthPopupBlocked"),
          });
          return;
        }

        await new Promise<void>((resolve, reject) => {
          const cleanup = () => {
            window.removeEventListener("message", onMessage);
            window.clearInterval(closedTimer);
          };
          const onMessage = (event: MessageEvent) => {
            if (event.source !== popup) return;
            const data = event.data as {
              type?: string;
              payload?: unknown;
            };
            if (data?.type !== "yoin-oauth") return;
            cleanup();
            if (!isOauthLoginPayload(data.payload)) {
              reject(new Error(t("auth.oauthFailed")));
              return;
            }
            persistLogin(data.payload);
            setUser(data.payload);
            setSubmitStatus({ type: "success", msg: t("auth.loginSuccess") });
            onSuccess?.();
            resolve();
          };
          const closedTimer = window.setInterval(() => {
            if (!popup.closed) return;
            cleanup();
            reject(new Error(t("auth.oauthFailed")));
          }, 400);
          window.addEventListener("message", onMessage);
        });
      } catch (error: unknown) {
        const msg = error instanceof Error ? error.message : String(error);
        setSubmitStatus({ type: "error", msg: msg || t("auth.oauthFailed") });
      } finally {
        setSubmitting(false);
      }
    },
    [onSuccess],
  );

  return {
    isLoginView,
    submitting,
    submitStatus,
    toggleView,
    handleSubmit,
    handleOauth,
    oauthProviders,
    setSubmitStatus,
    user,
  };
};
