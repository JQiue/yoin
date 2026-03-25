import { useEffect, useState } from "preact/hooks";

import { fetchCommentReplies } from "@/shared/api/comment";
import type { Comment } from "@/shared/api/types";

const REPLY_PAGE_SIZE = 3;

type UseCommentRepliesOptions = {
  comment: Comment;
  isRootComment: boolean;
  siteId?: number;
  sort: string;
  onReplyCreated?: (createdComment: Comment) => void;
};

export const useCommentReplies = ({
  comment,
  isRootComment,
  siteId,
  sort,
  onReplyCreated,
}: UseCommentRepliesOptions) => {
  const [replies, setReplies] = useState(comment.replies ?? []);
  const [hasMoreReplies, setHasMoreReplies] = useState(
    Boolean(comment.has_more),
  );
  const [replyPage, setReplyPage] = useState(comment.replies?.length ? 1 : 0);
  const [isLoadingReplies, setIsLoadingReplies] = useState(false);

  useEffect(() => {
    if (!isRootComment) return;
    setReplies(comment.replies ?? []);
    setHasMoreReplies(Boolean(comment.has_more));
    setReplyPage(comment.replies?.length ? 1 : 0);
  }, [comment, isRootComment]);

  const handleReplyCreated = (createdReply?: Comment) => {
    if (!createdReply) return;

    if (!isRootComment && onReplyCreated) {
      onReplyCreated(createdReply);
      return;
    }

    onReplyCreated?.(createdReply);
    setReplies((prev) => [...prev, createdReply]);
  };

  const handleDeleteReply = (deletedReply: Comment) => {
    setReplies((prev) => prev.filter((reply) => reply.id !== deletedReply.id));
  };

  const handleLoadMoreReplies = async () => {
    if (isLoadingReplies || siteId == null) return;

    const nextPage = replyPage + 1;
    setIsLoadingReplies(true);

    try {
      const {
        data: { items, total_pages, page_offset },
      } = await fetchCommentReplies(
        comment.id,
        siteId,
        nextPage,
        REPLY_PAGE_SIZE,
        location.pathname,
        sort,
      );
      setReplies((prev) => [...prev, ...items]);
      setReplyPage(page_offset);
      setHasMoreReplies(page_offset < total_pages);
    } finally {
      setIsLoadingReplies(false);
    }
  };

  return {
    replies,
    hasMoreReplies,
    isLoadingReplies,
    handleReplyCreated,
    handleDeleteReply,
    handleLoadMoreReplies,
  };
};
