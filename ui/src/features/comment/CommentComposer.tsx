import type { RefObject, TargetedEvent } from "preact";
import type { StoredUser } from "@/hook/useStoredUser";
import { Button } from "@/shared/components/Button";
import Icon from "@/shared/components/Icon";

interface Props {
  content: string;
  currentUser: Pick<StoredUser, "nickname"> | null;
  submitting: boolean;
  textareaRef: RefObject<HTMLTextAreaElement>;
  onInputChange: (
    e: TargetedEvent<HTMLTextAreaElement | HTMLInputElement>,
  ) => void;
}

export default ({
  content,
  currentUser,
  submitting,
  textareaRef,
  onInputChange,
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
        className="p-3 outline-none m-0 min-h-28 w-full resize-none overflow-hidden rounded-md bg-zinc-200/50  placeholder:text-zinc-400 focus:bg-zinc-200"
        required
      ></textarea>
      <div className="absolute bottom-3 right-2 flex items-center gap-2">
        <span className="text-xs text-zinc-400">{content.length}</span>
        <Button variant="primary" type="submit" size="sm" disabled={submitting}>
          {submitting ? <Icon name="refresh" /> : <Icon name="send" />}
          {submitting ? "发送中" : "发送"}
        </Button>
      </div>
    </div>
  );
};
