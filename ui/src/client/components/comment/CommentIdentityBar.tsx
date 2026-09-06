import type { TargetedEvent } from "preact";
import type { StoredUser } from "@/client/hooks/useStoredUser";
import type { CommentForm } from "@/client/types";
import { Button } from "@/shared/components/Button";
import { guestLabel } from "@/shared/helper";

interface Props {
  currentUser: StoredUser | null;
  fields: {
    name: keyof Omit<CommentForm, "content" | "is_private" | "is_anonymous">;
    placeholder: string;
    type: string;
  }[];
  fieldValues: {
    nickname: string;
    email: string;
    website: string;
  };
  onInputChange: (
    e: TargetedEvent<HTMLTextAreaElement | HTMLInputElement>,
  ) => void;
  onLoginClick: () => void;
  onLogout: () => void;
}

export default ({
  currentUser,
  fields,
  fieldValues,
  onInputChange,
  onLoginClick,
  onLogout,
}: Props) => {
  return (
    <div className="rounded-md bg-zinc-100/80 px-3 py-3">
      <div className="flex items-center justify-between gap-3">
        {currentUser ? (
          <div className="flex min-w-0 items-center gap-3">
            <img
              className="h-10 w-10 rounded-full object-cover"
              src={currentUser.avatar}
              alt={currentUser.nickname}
            />
            <div className="min-w-0">
              <div className="truncate text-sm font-semibold text-zinc-900">
                {currentUser.nickname}
              </div>
              <div className="truncate text-xs text-zinc-500">
                {currentUser.email}
              </div>
            </div>
          </div>
        ) : (
          <div className="flex min-w-0 items-center gap-3">
            <span className="flex h-10 w-10 items-center justify-center rounded-full bg-zinc-200 text-sm font-medium text-zinc-600">
              访
            </span>
            <div className="min-w-0">
              <div className="text-sm font-semibold text-zinc-900">
                {guestLabel()}
              </div>
              <div className="text-xs text-zinc-500">
                未登录访客，填写昵称后可公开显示
              </div>
            </div>
          </div>
        )}
        {currentUser ? (
          <Button type="button" variant="ghost" size="sm" onClick={onLogout}>
            退出登录
          </Button>
        ) : (
          <Button
            type="button"
            className="shrink-0 text-zinc-500 hover:text-zinc-900"
            onClick={onLoginClick}
            size="sm"
            variant="ghost"
          >
            登录
          </Button>
        )}
      </div>
      {currentUser ? null : (
        <div className="mt-3 grid grid-cols-1 gap-3 md:grid-cols-3">
          {fields.map((field) => (
            <input
              className="w-full rounded-md bg-white px-3 py-2 outline-none placeholder:text-zinc-400 focus:bg-white"
              key={field.name}
              type={field.type}
              name={field.name}
              placeholder={field.placeholder}
              onChange={onInputChange}
              required
              value={fieldValues[field.name]}
            />
          ))}
        </div>
      )}
    </div>
  );
};
