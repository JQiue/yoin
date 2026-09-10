import type { LucideIcon } from "lucide-preact";
import { LoaderCircle, Lock, Send, VenetianMask } from "lucide-preact";
import type { RefObject, TargetedEvent } from "preact";
import type { StoredUser } from "@/client/hooks/useStoredUser";
import { IconButton } from "@/shared/components/IconButton";
import { useI18n } from "@/shared/i18n";

function ComposerToggle({
  pressed,
  label,
  Icon: ToggleIcon,
  onToggle,
}: {
  pressed: boolean;
  label: string;
  Icon: LucideIcon;
  onToggle: () => void;
}) {
  const { t } = useI18n();
  return (
    <button
      type="button"
      aria-pressed={pressed}
      aria-label={pressed ? t("client.cancelToggle", { label }) : label}
      title={
        pressed
          ? t("client.toggleOn", { label })
          : t("client.toggleOff", { label })
      }
      onClick={onToggle}
      className={`inline-flex h-7 w-7 items-center justify-center rounded-md transition-colors ${
        pressed
          ? "text-(--yo-text)"
          : "text-(--yo-text-soft) hover:text-(--yo-text-muted)"
      }`}
    >
      <ToggleIcon
        size={16}
        strokeWidth={pressed ? 1.75 : 2}
        fill={pressed ? "currentColor" : "none"}
      />
    </button>
  );
}

interface Props {
  content: string;
  currentUser: Pick<StoredUser, "nickname"> | null;
  submitting: boolean;
  isPrivate: boolean;
  isAnonymous: boolean;
  allowAnonymous: boolean;
  allowPrivate: boolean;
  maxCommentLength: number;
  textareaRef: RefObject<HTMLTextAreaElement>;
  onInputChange: (
    e: TargetedEvent<HTMLTextAreaElement | HTMLInputElement>,
  ) => void;
  onPrivateChange: (checked: boolean) => void;
  onAnonymousChange: (checked: boolean) => void;
}

export default ({
  content,
  currentUser,
  submitting,
  isPrivate,
  isAnonymous,
  allowAnonymous,
  allowPrivate,
  maxCommentLength,
  textareaRef,
  onInputChange,
  onPrivateChange,
  onAnonymousChange,
}: Props) => {
  const { t } = useI18n();
  return (
    <div className="relative">
      <textarea
        ref={textareaRef}
        name="content"
        rows={1}
        placeholder={
          currentUser
            ? t("client.writeAs", { name: currentUser.nickname })
            : t("client.writeComment")
        }
        value={content}
        onChange={onInputChange}
        maxLength={maxCommentLength}
        className="p-3 pb-10 outline-none m-0 min-h-28 w-full resize-none overflow-hidden rounded-md bg-(--yo-surface-soft) placeholder:text-(--yo-text-soft) focus:bg-(--yo-surface-strong) text-(--yo-text)"
        required
      ></textarea>
      <div className="absolute bottom-3 right-2 flex items-center gap-2">
        <span className="text-xs text-(--yo-text-soft)">
          {content.length}/{maxCommentLength}
        </span>
        {allowAnonymous ? (
          <ComposerToggle
            pressed={isAnonymous}
            label={t("client.anonymousComment")}
            Icon={VenetianMask}
            onToggle={() => onAnonymousChange(!isAnonymous)}
          />
        ) : null}
        {allowPrivate ? (
          <ComposerToggle
            pressed={isPrivate}
            label={t("client.privateComment")}
            Icon={Lock}
            onToggle={() => onPrivateChange(!isPrivate)}
          />
        ) : null}
        <IconButton
          type="submit"
          icon={submitting ? LoaderCircle : Send}
          label={submitting ? t("client.sending") : t("client.send")}
          disabled={submitting}
          className={`text-(--yo-text) ${submitting ? "animate-spin" : ""}`}
        />
      </div>
    </div>
  );
};
