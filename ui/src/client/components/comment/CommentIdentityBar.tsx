import { LogIn, LogOut } from "lucide-preact";
import type { TargetedEvent } from "preact";
import type { StoredUser } from "@/client/hooks/useStoredUser";
import type { CommentForm } from "@/client/types";
import { IconButton } from "@/shared/components/IconButton";
import { ANONYMOUS_AVATAR, guestLabel } from "@/shared/helper";
import { useI18n } from "@/shared/i18n";

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
  const { t } = useI18n();
  return (
    <div className="rounded-md bg-(--yo-surface-soft) px-3 py-3">
      <div className="flex items-center justify-between gap-3">
        {currentUser ? (
          <div className="flex min-w-0 items-center gap-3">
            <img
              className="h-10 w-10 rounded-full object-cover"
              src={currentUser.avatar}
              alt={currentUser.nickname}
            />
            <div className="min-w-0">
              <div className="truncate text-sm font-semibold text-(--yo-text)">
                {currentUser.nickname}
              </div>
              <div className="truncate text-xs text-(--yo-text-muted)">
                {currentUser.email}
              </div>
            </div>
          </div>
        ) : (
          <div className="flex min-w-0 items-center gap-3">
            <span className="flex h-10 w-10 items-center justify-center overflow-hidden rounded-full bg-(--yo-surface-strong) text-(--yo-text-muted)">
              <img
                src={ANONYMOUS_AVATAR}
                alt={t("client.guestAvatar")}
                className="h-full w-full object-cover"
              />
            </span>
            <div className="min-w-0">
              <div className="text-sm font-semibold text-(--yo-text)">
                {guestLabel()}
              </div>
              <div className="text-xs text-(--yo-text-muted)">
                {t("client.guestHint")}
              </div>
            </div>
          </div>
        )}
        {currentUser ? (
          <IconButton
            icon={LogOut}
            label={t("client.logout")}
            onClick={onLogout}
          />
        ) : (
          <IconButton
            icon={LogIn}
            label={t("client.login")}
            onClick={onLoginClick}
          />
        )}
      </div>
      {currentUser ? null : (
        <div className="mt-3 grid grid-cols-1 gap-3 md:grid-cols-3">
          {fields.map((field) => (
            <input
              className="w-full rounded-md bg-(--yo-surface) px-3 py-2 outline-none placeholder:text-(--yo-text-soft) focus:bg-(--yo-surface) text-(--yo-text)"
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
