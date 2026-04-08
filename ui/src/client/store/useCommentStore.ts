import { create } from "zustand";
import { getRuntimeConfig } from "@/config/runtime";
import { deleteComment, fetchCommentsList, vote } from "@/shared/api";
import type { Comment } from "@/shared/api/types";
import type { CommentsState } from "@/client/types";

const removeCommentById = (comments: Comment[], id: number): Comment[] => {
  return comments
    .filter((comment) => comment.id !== id)
    .map((comment) => ({
      ...comment,
      replies: comment.replies
        ? removeCommentById(comment.replies, id)
        : comment.replies,
    }));
};

export const useCommentStore = create<CommentsState>((set, get) => ({
  comments: [],
  total: 0,
  pageOffset: 1,
  pageSize: 10,
  totalPages: 0,
  sort: "created_desc",
  isLoading: false,
  setComments: (comments) => set({ comments }),
  fetchComments: async (pageOffset = 1, append = false) => {
    const config = getRuntimeConfig();
    const { comments: oldComments, pageSize, sort } = get();
    if (config.site_id == null) {
      return;
    }
    set({ isLoading: true });
    try {
      const {
        data: { items, total, total_pages },
      } = await fetchCommentsList(
        config.site_id,
        pageOffset,
        pageSize,
        location.pathname,
        sort,
      );
      set({
        comments: append ? [...oldComments, ...items] : items,
        total,
        pageOffset,
        totalPages: total_pages,
      });
    } catch (error) {
      console.error("Failed to fetch comments", error);
    } finally {
      set({ isLoading: false });
    }
  },
  fetchNextPage: async () => {
    const { pageOffset, fetchComments } = get();
    fetchComments(pageOffset + 1, true);
  },
  changeSort: (newSort: string) => {
    set({ sort: newSort, pageOffset: 1 });
    get().fetchComments();
  },
  deleteComment: async (id: number) => {
    const { comments, total } = get();
    set({ comments: removeCommentById(comments, id), total: total - 1 });
    await deleteComment(id);
  },
  updateCommentVote: async (id: number, type: "up" | "down") => {
    const { comments } = get();
    const newComments = comments.map((comment) => {
      if (comment.id === id) {
        if (type === "up") {
          return { ...comment, up_vote: comment.up_vote + 1 };
        } else {
          return { ...comment, down_vote: comment.down_vote + 1 };
        }
      }
      if (comment.replies) {
        const newReplies = comment.replies.map((reply) => {
          if (reply.id === id) {
            if (type === "up") {
              return { ...reply, up_vote: reply.up_vote + 1 };
            } else {
              return { ...reply, down_vote: reply.down_vote + 1 };
            }
          }
          return reply;
        });
        return { ...comment, replies: newReplies };
      }
      return comment;
    });
    set({ comments: newComments });
    await vote(id, type);
  },
}));
