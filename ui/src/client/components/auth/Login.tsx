import type { TargetedEvent, TargetedSubmitEvent } from "preact";
import { useState } from "preact/hooks";
import { type LoginInput, useAuthForm } from "@/client/hooks/useAuthForm";
import { Button } from "@/shared/components/Button";
import Icon from "@/shared/components/Icon";
import { useI18n } from "@/shared/i18n";

interface Props {
  onSuccess: () => void;
}

export default (props: Props) => {
  const {
    isLoginView,
    toggleView,
    handleSubmit,
    submitStatus,
    submitting,
    setSubmitStatus,
  } = useAuthForm(props.onSuccess);
  const { t } = useI18n();

  return (
    <div>
      <h2 class="mb-6 text-2xl font-bold text-center">
        {isLoginView ? t("auth.welcomeBack") : t("auth.createAccount")}
      </h2>
      {isLoginView ? (
        <LoginForm
          submitStatus={submitStatus}
          submitting={submitting}
          onSubmit={handleSubmit}
        />
      ) : (
        <RegisterForm
          submitStatus={submitStatus}
          submitting={submitting}
          onSubmit={handleSubmit}
          setSubmitStatus={setSubmitStatus}
        />
      )}
      <div className="mt-6 text-center text-sm ">
        {isLoginView ? t("auth.noAccount") : t("auth.hasAccount")}
        <Button variant="ghost" onClick={toggleView}>
          {isLoginView ? t("auth.registerNow") : t("auth.backToLogin")}
        </Button>
      </div>
    </div>
  );
};

interface LoginFormProps {
  submitting: boolean;
  submitStatus: { type: string; msg: string };
  onSubmit: (e: TargetedSubmitEvent<HTMLFormElement>, data: LoginInput) => void;
}

const LoginForm = ({ submitting, submitStatus, onSubmit }: LoginFormProps) => {
  const { t } = useI18n();
  const [credentials, setCredentials] = useState<LoginInput>({
    email: "",
    password: "",
  });

  const handleInputChange = (
    e: TargetedEvent<HTMLTextAreaElement | HTMLInputElement>,
  ) => {
    const { name, value } = e.currentTarget;
    setCredentials({ ...credentials, [name]: value });
  };

  return (
    <form onSubmit={(e) => onSubmit(e, credentials)} className="space-y-4">
      <input
        className="w-full px-3 py-2 rounded-md text-sm outline-none transition-all bg-(--yo-surface-soft) focus:bg-(--yo-surface-strong) placeholder:text-(--yo-text-soft) text-(--yo-text)"
        name="email"
        type="email"
        required
        placeholder={t("auth.email")}
        autocomplete="username"
        onChange={handleInputChange}
      />
      <input
        className="w-full px-3 py-2 rounded-md text-sm outline-none transition-all bg-(--yo-surface-soft) focus:bg-(--yo-surface-strong) placeholder:text-(--yo-text-soft) text-(--yo-text)"
        name="password"
        type="password"
        required
        placeholder={t("auth.password")}
        autocomplete="current-password"
        onChange={handleInputChange}
      />
      <div className="flex items-center gap-2">
        {submitStatus.msg && (
          <span
            className={`text-xs font-bold flex items-center gap-1.5 px-2 py-1 rounded-md ${authStatusClass(submitStatus.type)}`}
          >
            {submitStatus.type === "error" && <Icon name="alert" />}
            {submitStatus.msg}
          </span>
        )}
      </div>
      <Button variant="primary" fullWidth type="submit" disabled={submitting}>
        {submitting ? t("auth.signingIn") : t("auth.signIn")}
      </Button>
    </form>
  );
};

function authStatusClass(type: string) {
  return type === "success"
    ? "text-(--yo-text-muted) bg-(--yo-surface-soft)"
    : "text-(--yo-danger) bg-(--yo-danger-bg)";
}

interface NewUserInput {
  email: string;
  password: string;
  password_confirmation: string;
  nickname: string;
  website: string;
}

interface RegisterFormProps {
  submitting: boolean;
  submitStatus: { type: string; msg: string };
  setSubmitStatus: (status: { type: string; msg: string }) => void;
  onSubmit: (
    e: TargetedSubmitEvent<HTMLFormElement>,
    data: NewUserInput,
  ) => void;
}

const RegisterForm = ({
  submitting,
  submitStatus,
  setSubmitStatus,
  onSubmit,
}: RegisterFormProps) => {
  const { t } = useI18n();
  const [newUser, setNewUser] = useState<NewUserInput>({
    email: "",
    password: "",
    nickname: "",
    password_confirmation: "",
    website: "",
  });

  const handleInputChange = (
    e: TargetedEvent<HTMLTextAreaElement | HTMLInputElement>,
  ) => {
    const { name, value } = e.currentTarget;
    setNewUser({ ...newUser, [name]: value });
  };

  const internalSubmit = (e: TargetedSubmitEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (newUser.password !== newUser.password_confirmation) {
      setSubmitStatus({ type: "error", msg: t("auth.passwordMismatch") });
      return;
    }
    onSubmit(e, newUser);
  };

  return (
    <form onSubmit={(e) => internalSubmit(e)} className="space-y-4">
      <input
        className="w-full px-3 py-2 rounded-md text-sm outline-none transition-all bg-(--yo-surface-soft) focus:bg-(--yo-surface-strong) placeholder:text-(--yo-text-soft) text-(--yo-text)"
        name="email"
        type="email"
        required
        placeholder={t("auth.email")}
        onChange={handleInputChange}
      />
      <input
        className="w-full px-3 py-2 rounded-md text-sm outline-none transition-all bg-(--yo-surface-soft) focus:bg-(--yo-surface-strong) placeholder:text-(--yo-text-soft) text-(--yo-text)"
        name="password"
        type="password"
        required
        placeholder={t("auth.password")}
        onChange={handleInputChange}
      />
      <input
        className="w-full px-3 py-2 rounded-md text-sm outline-none transition-all bg-(--yo-surface-soft) focus:bg-(--yo-surface-strong) placeholder:text-(--yo-text-soft) text-(--yo-text)"
        name="password_confirmation"
        type="password"
        required
        placeholder={t("auth.confirmPassword")}
        onChange={handleInputChange}
      />
      <input
        className="w-full px-3 py-2 rounded-md text-sm outline-none transition-all bg-(--yo-surface-soft) focus:bg-(--yo-surface-strong) placeholder:text-(--yo-text-soft) text-(--yo-text)"
        name="nickname"
        type="text"
        required
        placeholder={t("auth.nickname")}
        onChange={handleInputChange}
      />
      <input
        className="w-full px-3 py-2 rounded-md text-sm outline-none transition-all bg-(--yo-surface-soft) focus:bg-(--yo-surface-strong) placeholder:text-(--yo-text-soft) text-(--yo-text)"
        name="website"
        type="url"
        required
        placeholder={t("auth.website")}
        onChange={handleInputChange}
      />
      <div className="flex items-center gap-2">
        {submitStatus.msg && (
          <span
            className={`text-xs font-bold flex items-center gap-1.5 px-2 py-1 rounded-md ${authStatusClass(submitStatus.type)}`}
          >
            {submitStatus.type === "error" && <Icon name="alert" />}
            {submitStatus.msg}
          </span>
        )}
      </div>
      <Button variant="primary" fullWidth type="submit" disabled={submitting}>
        {submitting ? t("auth.registering") : t("auth.register")}
      </Button>
    </form>
  );
};
