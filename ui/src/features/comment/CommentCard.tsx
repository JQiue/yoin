import { useState } from "preact/hooks";
import CommentForm from "@/features/comment/CommentForm";
import CommentList from "@/features/comment/CommentList";
import { useCommentReplies } from "@/hook/useCommentReplies";
import type { Comment } from "@/shared/api/types";
import { Button } from "@/shared/components/Button";
import { formatDate } from "@/shared/helper";
import { useCommentStore, useConfigStore } from "@/store";

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
  const sort = useCommentStore((state) => state.sort);
  const siteId = useConfigStore((state) => state.config.site_id);
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

  return (
    <div
      className={`-mx-4 px-4 py-3 rounded-md transition-all hover:bg-zinc-100 ${comment.parent_id ? "ml-8 sm:ml-12" : ""}`}
    >
      <div className="flex items-start gap-2.5">
        <div className={`w-9 h-9 sm:w-11 sm:h-11 rounded-full`}>
          <img
            className="w-full h-full rounded-full"
            src={comment.avatar}
            alt="avatar"
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
            <span>{comment.location || "LOCAL"}</span>
          </div>
          <div
            className="w-full mt-2 comment-content "
            dangerouslySetInnerHTML={{ __html: comment.content }}
          ></div>
          <div className="mt-2 flex gap-2 text-xs group">
            <Button variant="ghost" size="sm">
              赞同
              {comment.up_vote || 0}
            </Button>
            <Button variant="ghost" size="sm">
              反对
              {comment.down_vote || 0}
            </Button>
            <Button
              className="invisible group-hover:visible"
              variant="ghost"
              size="sm"
              aria-label="回复评论"
              onClick={handleClickReply}
            >
              回复
            </Button>
            <Button
              className="invisible group-hover:visible"
              variant="ghost"
              size="sm"
              onClick={handleClickDelete}
            >
              删除
            </Button>
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
              <Button
                type="button"
                disabled={isLoadingReplies}
                onClick={handleLoadMoreReplies}
                size="sm"
                variant="ghost"
                className="px-0 py-0 text-zinc-500 hover:text-zinc-900"
              >
                {isLoadingReplies ? "加载中..." : "查看更多回复"}
              </Button>
            </div>
          ) : null}
        </div>
      ) : null}
    </div>
  );
};
