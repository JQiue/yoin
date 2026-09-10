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
  allow_private: boolean;
  max_comment_length: number;
  comment_limit_seconds: number;
  allowed_reactions: string[];
};

export type PublicSiteConfig = SiteConfig;

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
  is_sticky: boolean;
  can_delete: boolean;
  can_pin: boolean;
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

export type UserIdentityForAdmin = {
  id: number;
  provider: string;
  provider_user_id: string;
  email: string | null;
};

export type UserBindingSummaryForAdmin = {
  id: number;
  role_id: number;
  role_name: string | null;
  scope_type: "global" | "site";
  scope_id: string | null;
};

export type UserForAdmin = {
  id: number;
  nickname: string;
  email: string;
  website: string;
  avatar: string;
  created_at: string;
  identities: UserIdentityForAdmin[];
  role_bindings: UserBindingSummaryForAdmin[];
};

export const GLOBAL_ALLOWED_REACTIONS = ["👍", "❤️", "😄", "🎉", "👎"];

export type ModerationProviderKind = "llm" | "akismet";

export type LlmModerationConfig = {
  model: string;
  api_base: string;
  api_key: string;
  rule?: string;
};

export type AkismetModerationConfig = {
  api_key: string;
  blog_url: string;
};

export type ModerationProviderConfig =
  | LlmModerationConfig
  | AkismetModerationConfig;

export type ModerationProvider = {
  id: number;
  site_id: number;
  provider_kind: ModerationProviderKind;
  enabled: boolean;
  config: ModerationProviderConfig;
};

export type OauthProvider = {
  id: number;
  site_id: number | null;
  enabled: boolean;
  provider_code: string;
  client_id: string;
  redirect_uri: string;
};
