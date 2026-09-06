import type { RefObject, TargetedEvent } from "preact";
import { Lock, VenetianMask } from "lucide-preact";
import type { LucideIcon } from "lucide-preact";
import type { StoredUser } from "@/client/hooks/useStoredUser";
import { Button } from "@/shared/components/Button";
import Icon from "@/shared/components/Icon";

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
  return (
    <button
      type="button"
      aria-pressed={pressed}
      aria-label={pressed ? `取消${label}` : label}
      title={pressed ? `${label}已开启` : `${label}未开启`}
      onClick={onToggle}
      className={`inline-flex h-7 w-7 items-center justify-center rounded-md transition-colors ${pressed
          ? "text-zinc-800"
          : "text-zinc-400 hover:text-zinc-600"
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
  return (
    <div className="relative">
      <textarea
        ref={textareaRef}
        name="content"
        rows={1}
        placeholder={
          currentUser
            ? `以 ${currentUser.nickname} 的身份发表评论...`
            : "write a comment..."
        }
        value={content}
        onChange={onInputChange}
        maxLength={maxCommentLength}
        className="p-3 pb-10 outline-none m-0 min-h-28 w-full resize-none overflow-hidden rounded-md bg-zinc-200/50 placeholder:text-zinc-400 focus:bg-zinc-200"
        required
      ></textarea>
      <div className="absolute bottom-3 right-2 flex items-center gap-2">
        {allowAnonymous ? (
          <ComposerToggle
            pressed={isAnonymous}
            label="匿名评论"
            Icon={VenetianMask}
            onToggle={() => onAnonymousChange(!isAnonymous)}
          />
        ) : null}
        {allowPrivate ? (
          <ComposerToggle
            pressed={isPrivate}
            label="私密评论"
            Icon={Lock}
            onToggle={() => onPrivateChange(!isPrivate)}
          />
        ) : null}
        <span className="text-xs text-zinc-400">
          {content.length}/{maxCommentLength}
        </span>
        <Button variant="primary" type="submit" size="sm" disabled={submitting}>
          {submitting ? <Icon name="refresh" /> : <Icon name="send" />}
          {submitting ? "发送中" : "发送"}
        </Button>
      </div>
    </div>
  );
};
