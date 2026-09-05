export type Method = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";

export interface RequestConfig extends Omit<RequestInit, "method"> {
  params?: Record<string, string | number | boolean | null | undefined>;
  data?: unknown;
}

export const SUCCESS_CODE = "ok";

export interface ResData<T> {
  code: string;
  msg: string;
  data: T;
}

export type Paged<T> = {
  items: T;
  page_offset: number;
  page_size: number;
  total: number;
  total_pages: number;
};

export type SiteConfig = {
  allow_anonymous: boolean;
  max_comment_length: number;
  comment_limit_seconds: number;
};

export type Site = {
  id: number;
  name: string;
  url: string;
  config: SiteConfig;
};

export type UserProfile = {
  avatar: string;
  nickname: string;
  website: string;
  email: string;
};

export type Comment = {
  id: number;
  thread_id: number | null;
  parent_id: number | null;
  nickname: string;
  website: string;
  content: string;
  device: string;
  location: string;
  avatar: string;
  created_at: string;
  is_anonymous: boolean;
  is_private: boolean;
  reactions: ReactionSummary;
  replies?: Comment[];
  has_more?: boolean;
};

export type ReactionSummary = {
  counts: Record<string, number>;
  my_reaction: string | null;
};

export type Login = {
  token: string;
  avatar: string;
  nickname: string;
  website: string;
  email: string;
};

export type CommentForAdmin = {
  id: number;
  thread_id: number | null;
  parent_id: number | null;
  nickname: string;
  website: string;
  content: string;
  device: string;
  location: string;
  avatar: string;
  created_at: string;
  updated_at: string;
  is_anonymous: boolean;
  is_private: boolean;
  status: "pending" | "approved" | "spam" | "deleted";
  replies?: CommentForAdmin[];
  has_more?: boolean;
};

export type AdminCapabilities = {
  global_permissions: string[];
  site_permissions: Record<string, string[]>;
};

export type RoleForAdmin = {
  id: number;
  name: string;
  description: string | null;
  permission_names: string[];
};

export type PermissionForAdmin = {
  id: number;
  name: string;
  description: string | null;
};

export type UserRoleBindingForAdmin = {
  id: number;
  user_id: number;
  role_id: number;
  role_name: string | null;
  scope_type: "global" | "site";
  scope_id: string | null;
  created_at: string;
  updated_at: string;
};
