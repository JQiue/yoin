import {
  ChevronsDown,
  Lock,
  Pin,
  Reply,
  Trash,
  VenetianMask,
} from "lucide-preact";
import { useState } from "preact/hooks";
import CommentForm from "@/client/components/CommentForm";
import CommentList from "@/client/components/CommentList";
import ReactionBar from "@/client/components/ReactionBar";
import { useCommentReplies } from "@/client/hooks/useCommentReplies";
import { useCommentStore } from "@/client/store";
import { getRuntimeConfig } from "@/config/runtime";
import type { Comment } from "@/shared/api/types";
import { IconButton } from "@/shared/components/IconButton";
import { commentAvatar, formatDate } from "@/shared/helper";
import { useI18n } from "@/shared/i18n";

interface Props {
  comment: Comment;
  onDeleteComment?: (deletedComment: Comment) => void;
  onReplyCreated?: (createdComment: Comment) => void;
}

const toSafeHttpUrl = (raw: string) => {
  try {
    if (!raw) return null;
    const parsed = new URL(raw);
    if (parsed.protocol === "http:" || parsed.protocol === "https:") {
      return parsed.toString();
    }
    return null;
  } catch (_error) {
    return null;
  }
};

const findCommentById = (
  comments: Comment[],
  id: number,
): Comment | undefined => {
  for (const comment of comments) {
    if (comment.id === id) return comment;
    if (comment.replies?.length) {
      const matched = findCommentById(comment.replies, id);
      if (matched) return matched;
    }
  }
};

export default ({ comment, onDeleteComment, onReplyCreated }: Props) => {
  const isRootComment = comment.parent_id == null;
  const comments = useCommentStore((state) => state.comments);
  const deleteComment = useCommentStore((state) => state.deleteComment);
  const setCommentSticky = useCommentStore((state) => state.setCommentSticky);
  const updateCommentReaction = useCommentStore(
    (state) => state.updateCommentReaction,
  );
  const sort = useCommentStore((state) => state.sort);
  const siteId = getRuntimeConfig().site_id;
  const [isReply, setIsReply] = useState(false);
  const safeWebsite = toSafeHttpUrl(comment.website);
  const {
    replies,
    hasMoreReplies,
    isLoadingReplies,
    handleReplyCreated,
    handleDeleteReply,
    handleLoadMoreReplies,
  } = useCommentReplies({
    comment,
    isRootComment,
    siteId,
    sort,
    onReplyCreated,
  });
  const { t } = useI18n();

  const replyNickname = (id: number | null) => {
    if (id == null) return undefined;
    return findCommentById(comments, id)?.nickname;
  };

  const handleClickReply = () => {
    setIsReply(!isReply);
  };

  const handleClickDelete = () => {
    deleteComment(comment.id);
    onDeleteComment?.(comment);
  };

  const handleClickPin = () => {
    return setCommentSticky(comment.id, !comment.is_sticky);
  };

  const handleSelectReaction = (reaction: string) => {
    return updateCommentReaction(comment.id, reaction);
  };

  return (
    <div
      className={`group -mx-4 px-4 py-3 rounded-md transition-all hover:bg-(--yo-surface-soft) ${comment.parent_id ? "ml-8 sm:ml-12" : ""}`}
    >
      <div className="flex items-start gap-2.5">
        <div className={`w-9 h-9 sm:w-11 sm:h-11 rounded-full`}>
          <img
            className="w-full h-full rounded-full bg-(--yo-surface-strong) object-cover"
            src={commentAvatar(comment)}
            alt={
              comment.is_anonymous
                ? t("client.anonymousAvatar")
                : t("common.avatar")
            }
          />
        </div>
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-x-2 text-[12px]">
            <span className="text-[14px] font-bold tracking-tight">
              {safeWebsite ? (
                <a
                  href={safeWebsite}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="hover:underline"
                >
                  {comment.nickname}
                </a>
              ) : (
                <span>{comment.nickname}</span>
              )}
              {comment.parent_id && (
                <span>&gt;{replyNickname(comment.parent_id)}</span>
              )}
            </span>
            <span>{formatDate(comment.created_at)}</span>
            {comment.is_anonymous ? (
              <span
                title={t("client.anonymousComment")}
                className="inline-flex text-(--yo-text-muted)"
              >
                <VenetianMask size={12} strokeWidth={2} />
              </span>
            ) : null}
            {comment.is_private ? (
              <span
                title={t("client.privateComment")}
                className="inline-flex text-(--yo-text-muted)"
              >
                <Lock size={12} strokeWidth={2} />
              </span>
            ) : null}
            {comment.is_sticky ? (
              <span
                title={t("client.stickyComment")}
                className="inline-flex text-(--yo-text-muted)"
              >
                <Pin size={12} strokeWidth={2} fill="currentColor" />
              </span>
            ) : null}
          </div>
          <div
            className="w-full mt-2 comment-content "
            dangerouslySetInnerHTML={{ __html: comment.content }}
          ></div>
          <div className="mt-2 flex flex-wrap items-center gap-2 text-xs">
            <ReactionBar
              summary={comment.reactions}
              onSelect={handleSelectReaction}
              hideEmpty
              showPickerOnHover
            />
            <IconButton
              className="invisible group-hover:visible!"
              icon={Reply}
              label={isReply ? t("client.cancelReply") : t("client.reply")}
              pressed={isReply}
              onClick={handleClickReply}
            />
            {comment.can_pin ? (
              <IconButton
                className="invisible group-hover:visible!"
                icon={Pin}
                label={comment.is_sticky ? t("client.unpin") : t("client.pin")}
                pressed={comment.is_sticky}
                onClick={handleClickPin}
              />
            ) : null}
            {comment.can_delete ? (
              <IconButton
                className="invisible group-hover:visible!"
                icon={Trash}
                label={t("client.deleteComment")}
                onClick={handleClickDelete}
              />
            ) : null}
          </div>
        </div>
      </div>
      {isReply ? (
        <div class="mt-2">
          <CommentForm
            parent_id={comment.id}
            cb={(createdReply) => {
              setIsReply(false);
              handleReplyCreated(createdReply);
            }}
          ></CommentForm>
        </div>
      ) : null}
      {isRootComment ? (
        <div>
          {replies.length > 0 && (
            <CommentList
              comments={replies}
              onDeleteComment={handleDeleteReply}
              onReplyCreated={handleReplyCreated}
            ></CommentList>
          )}
          {hasMoreReplies ? (
            <div className="mt-2 ml-11 sm:ml-14">
              <IconButton
                icon={ChevronsDown}
                label={
                  isLoadingReplies
                    ? t("client.loadingReplies")
                    : t("client.loadMoreReplies")
                }
                disabled={isLoadingReplies}
                onClick={handleLoadMoreReplies}
                className={isLoadingReplies ? "animate-pulse" : ""}
              />
            </div>
          ) : null}
        </div>
      ) : null}
    </div>
  );
};
