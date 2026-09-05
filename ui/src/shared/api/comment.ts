import { http } from "@/shared/api/client";
import type { Comment, Paged, ReactionSummary } from "@/shared/api/types";

export const fetchCommentsList = (
  site_id: number,
  page_offset: number,
  page_size: number,
  page_path: string,
  sort: string,
) => {
  return http.get<Paged<Comment[]>>("/api/comments", {
    params: {
      site_id,
      page_offset,
      page_size,
      page_path,
      sort,
    },
  });
};

export const sendComment = (
  site_id: number,
  nickname: string,
  email: string,
  website: string,
  content: string,
  page_path: string,
  parent_id?: number,
) => {
  return http.post<Comment>("/api/comments", {
    site_id,
    nickname,
    website,
    content,
    page_path,
    email,
    parent_id,
  });
};

export const deleteComment = (id: number) => {
  return http.delete(`/api/comments/${id}`);
};

export const upsertReaction = (
  site_id: number,
  target_type: "comment" | "page",
  page_path: string,
  reaction: string,
  comment_id?: number,
) => {
  return http.post<ReactionSummary>("/api/reactions", {
    site_id,
    target_type,
    page_path,
    reaction,
    comment_id,
  });
};

export const fetchReactions = (
  site_id: number,
  target_type: "comment" | "page",
  page_path: string,
  comment_id?: number,
) => {
  return http.get<ReactionSummary>("/api/reactions", {
    params: {
      site_id,
      target_type,
      page_path,
      comment_id,
    },
  });
};

export const fetchCommentReplies = (
  id: number,
  site_id: number,
  page_offset: number,
  page_size: number,
  page_path: string,
  sort: string,
) => {
  return http.get<Paged<Comment[]>>(`/api/comments/${id}/replies`, {
    params: {
      site_id,
      page_offset,
      page_size,
      page_path,
      sort,
    },
  });
};
