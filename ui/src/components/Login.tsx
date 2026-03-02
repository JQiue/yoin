import type { TargetedEvent, TargetedSubmitEvent } from "preact";
import { useState } from "preact/hooks";
import Icon from "./Icon";
import { useAuthForm, type LoginInput } from "../hook/useAuthForm";

interface Props {
  onSuccess: () => void;
}

export default (props: Props) => {
  const { isLoginView, toggleView, handleSubmit, submitStatus, submitting, setSubmitStatus } = useAuthForm(props.onSuccess);

  return (
    <div>
      <h2 class="text-2xl font-bold text-gray-800 mb-6 text-center">{isLoginView ? '欢迎回来' : '创建新账号'}</h2>
      {isLoginView ? <LoginForm submitStatus={submitStatus} submitting={submitting} onSubmit={handleSubmit} /> : <RegisterForm submitStatus={submitStatus} submitting={submitting} onSubmit={handleSubmit} setSubmitStatus={setSubmitStatus} />}
      <div className="mt-6 text-center text-sm text-gray-600">
        {isLoginView ? "还没有账号？" : "已有账号？"}
        <button
          onClick={toggleView}
          className="ml-2 text-blue-600 hover:underline font-medium"
        >
          {isLoginView ? '立即注册' : '返回登录'}
        </button>
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
  const [credentials, setCredentials] = useState<LoginInput>({
    email: "", password: ""
  });

  const handleInputChange = (
    e: TargetedEvent<HTMLTextAreaElement | HTMLInputElement>,
  ) => {
    const { name, value } = e.currentTarget;
    setCredentials({ ...credentials, [name]: value });
  };

  return <form onSubmit={(e) => onSubmit(e, credentials)} className="space-y-4">
    <input className="w-full px-3 py-2 border rounded-sm bg-zinc-50 text-sm outline-none transition-all focus:bg-white focus:ring-1 placeholder:text-zinc-400" name="email" type="email" required placeholder="邮箱" autocomplete="username" onChange={handleInputChange} />
    <input className="w-full px-3 py-2 border rounded-sm bg-zinc-50 text-sm outline-none transition-all focus:bg-white focus:ring-1 placeholder:text-zinc-400" name="password" type="password" required placeholder="密码" autocomplete="current-password" onChange={handleInputChange} />

    <div className="flex items-center gap-2">
      {submitStatus.msg && (
        <span
          className={`text-xs font-bold flex items-center gap-1.5 px-2 py-1 rounded-md ${submitStatus.type === "success"
            ? "text-zinc-600 bg-zinc-100"
            : "text-red-600 bg-red-50"
            }`}
        >
          {submitStatus.type === "error" && <Icon name="alert" />}
          {submitStatus.msg}
        </span>
      )}
    </div>
    <div className="">
      <button
        type="submit"
        disabled={submitting}
        className="flex items-center justify-center gap-2 w-full px-3 py-2 shadow-sm rounded-sm bg-zinc-800 text-white text-xs font-bold hover:bg-zinc-600 disabled:bg-zinc-300 disabled:cursor-not-allowed transition-all active:scale-95"
      >
        {submitting ? "登录中" : "登录"}
      </button>
    </div>
  </form >
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
  onSubmit: (e: TargetedSubmitEvent<HTMLFormElement>, data: NewUserInput) => void;
}

const RegisterForm = ({ submitting, submitStatus, setSubmitStatus, onSubmit }: RegisterFormProps) => {
  const [newUser, setNewUser] = useState<NewUserInput>({ email: "", password: "", nickname: "", password_confirmation: "", website: "" });

  const handleInputChange = (
    e: TargetedEvent<HTMLTextAreaElement | HTMLInputElement>,
  ) => {
    const { name, value } = e.currentTarget;
    setNewUser({ ...newUser, [name]: value });
  };

  const internalSubmit = (e: TargetedSubmitEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (newUser.password !== newUser.password_confirmation) {
      setSubmitStatus({ type: "error", msg: "两次密码不一致" });
      return;
    }
    onSubmit(e, newUser);
  };

  return <form onSubmit={(e) => internalSubmit(e)} className="space-y-4">
    <input className="w-full px-3 py-2 border rounded-sm bg-zinc-50 text-sm outline-none transition-all focus:bg-white focus:ring-1 placeholder:text-zinc-400" name="email" type="email" required placeholder="邮箱" onChange={handleInputChange} />
    <input className="w-full px-3 py-2 border rounded-sm bg-zinc-50 text-sm outline-none transition-all focus:bg-white focus:ring-1 placeholder:text-zinc-400" name="password" type="password" required placeholder="密码" onChange={handleInputChange} />
    <input className="w-full px-3 py-2 border rounded-sm bg-zinc-50 text-sm outline-none transition-all focus:bg-white focus:ring-1 placeholder:text-zinc-400" name="password_confirmation" type="password" required placeholder="确认密码" onChange={handleInputChange} />
    <input className="w-full px-3 py-2 border rounded-sm bg-zinc-50 text-sm outline-none transition-all focus:bg-white focus:ring-1 placeholder:text-zinc-400" name="nickname" type="text" required placeholder="昵称" onChange={handleInputChange} />
    <input className="w-full px-3 py-2 border rounded-sm bg-zinc-50 text-sm outline-none transition-all focus:bg-white focus:ring-1 placeholder:text-zinc-400" name="website" type="url" required placeholder="网址：https://www.example.com" onChange={handleInputChange} />
    <div className="flex items-center gap-2">
      {submitStatus.msg && (
        <span
          className={`text-xs font-bold flex items-center gap-1.5 px-2 py-1 rounded-md ${submitStatus.type === "success"
            ? "text-zinc-600 bg-zinc-100"
            : "text-red-600 bg-red-50"
            }`}
        >
          {submitStatus.type === "error" && <Icon name="alert" />}
          {submitStatus.msg}
        </span>
      )}
    </div>
    <div className="">
      <button
        type="submit"
        disabled={submitting}
        className="flex items-center justify-center gap-2 w-full px-3 py-2 shadow-sm rounded-sm bg-zinc-800 text-white text-xs font-bold hover:bg-zinc-600 disabled:bg-zinc-300 disabled:cursor-not-allowed transition-all active:scale-95"
      >
        {submitting ? "注册中" : "注册"}
      </button>
    </div>
  </form>
}
