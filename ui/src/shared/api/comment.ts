import { http } from "@/shared/api/client";
import type { Comment, Paged } from "@/shared/api/types";

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

export const vote = (id: number, type: "up" | "down") => {
  return http.patch(`/api/comments/${id}/vote/${type}`);
};
