import { create } from "zustand";
import type { CommentsState } from "@/client/types";
import { getRuntimeConfig } from "@/config/runtime";
import {
  deleteComment,
  fetchCommentsList,
  fetchPublicSiteConfig,
  fetchReactions,
  upsertReaction,
} from "@/shared/api";
import type { Comment, ReactionSummary } from "@/shared/api/types";

const emptyReactions = (): ReactionSummary => ({
  counts: {},
  my_reaction: null,
});

const mapCommentById = (
  comments: Comment[],
  id: number,
  updater: (comment: Comment) => Comment,
): Comment[] => {
  return comments.map((comment) => {
    if (comment.id === id) {
      return updater(comment);
    }
    if (comment.replies?.length) {
      return {
        ...comment,
        replies: mapCommentById(comment.replies, id, updater),
      };
    }
    return comment;
  });
};

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
  pageReactions: emptyReactions(),
  siteConfig: null,
  setComments: (comments) => set({ comments }),
  fetchSiteConfig: async () => {
    const config = getRuntimeConfig();
    if (config.site_id == null) return;
    const { data } = await fetchPublicSiteConfig(config.site_id);
    set({ siteConfig: data });
  },
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
  updateCommentReaction: async (id: number, reaction: string) => {
    const config = getRuntimeConfig();
    if (config.site_id == null) return;
    const { data } = await upsertReaction(
      config.site_id,
      "comment",
      location.pathname,
      reaction,
      id,
    );
    set({
      comments: mapCommentById(get().comments, id, (comment) => ({
        ...comment,
        reactions: data,
      })),
    });
  },
  fetchPageReactions: async () => {
    const config = getRuntimeConfig();
    if (config.site_id == null) return;
    const { data } = await fetchReactions(
      config.site_id,
      "page",
      location.pathname,
    );
    set({ pageReactions: data });
  },
  updatePageReaction: async (reaction: string) => {
    const config = getRuntimeConfig();
    if (config.site_id == null) return;
    const { data } = await upsertReaction(
      config.site_id,
      "page",
      location.pathname,
      reaction,
    );
    set({ pageReactions: data });
  },
}));
