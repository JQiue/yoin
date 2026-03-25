import type { RuntimeOptions } from "@/config/types";
import type { Comment } from "@/shared/api/types";

export type Option = RuntimeOptions & {
  site_id: number;
};

export interface CommentForm {
  nickname: string;
  email: string;
  website: string;
  content: string;
}

export interface CommentsState {
  comments: Comment[];
  total: number;
  pageOffset: number;
  pageSize: number;
  totalPages: number;
  sort: string;
  isLoading: boolean;
  setComments: (comments: Comment[]) => void;
  fetchComments: (pageOffset?: number, append?: boolean) => void;
  fetchNextPage: () => void;
  changeSort: (newSort: string) => void;
  deleteComment: (id: number) => void;
}
