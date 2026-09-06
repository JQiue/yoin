import type { RuntimeOptions } from "@/config/types";
import type {
  Comment,
  PublicSiteConfig,
  ReactionSummary,
} from "@/shared/api/types";

export type Option = RuntimeOptions & {
  site_id: number;
};

export interface CommentForm {
  nickname: string;
  email: string;
  website: string;
  content: string;
  is_private: boolean;
  is_anonymous: boolean;
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
  setCommentSticky: (id: number, isSticky: boolean) => Promise<void>;
  updateCommentReaction: (id: number, reaction: string) => Promise<void>;
  pageReactions: ReactionSummary;
  fetchPageReactions: () => void;
  updatePageReaction: (reaction: string) => Promise<void>;
  siteConfig: PublicSiteConfig | null;
  fetchSiteConfig: () => void;
}
