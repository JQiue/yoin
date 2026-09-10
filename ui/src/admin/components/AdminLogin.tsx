import Login from "@/client/components/auth/Login";

interface Props {
  error?: string;
  onSuccess: () => void;
}

export const AdminLogin = ({ error, onSuccess }: Props) => {
  return (
    <div className="flex min-h-screen items-center justify-center bg-(--yo-bg) px-4 text-(--yo-text)">
      <div className="w-full max-w-md rounded-xl border border-(--yo-surface-strong) bg-(--yo-surface) p-6 shadow-sm">
        {error ? (
          <p className="mt-3 text-center text-sm text-(--yo-danger)">{error}</p>
        ) : null}
        <div className="mt-4">
          <Login onSuccess={onSuccess} />
        </div>
      </div>
    </div>
  );
};
