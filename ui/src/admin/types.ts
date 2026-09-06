import type { RuntimeOptions } from "@/config/types";

export type Option = RuntimeOptions;

export type AdminTab =
  | "sites"
  | "comments"
  | "users"
  | "permissions"
  | "oauthProviders"
  | "moderationProviders"
  | "externalProviders";

export type CommentTab = "all" | "pending" | "spam" | "deleted";

export type AdminTabItem = {
  key: AdminTab;
  label: string;
  description: string;
};

export type CommentTabItem = {
  key: CommentTab;
  label: string;
};

export type SiteFormState = {
  name: string;
  url: string;
  allowAnonymous: boolean;
  allowPrivate: boolean;
  maxCommentLength: string;
  commentLimitSeconds: string;
  allowedReactions: string[];
};

export type OAuthProviderFormState = {
  providerCode: string;
  clientId: string;
  clientSecret: string;
  redirectUri: string;
  enabled: boolean;
};

export type ModerationProviderFormState = {
  siteId: string;
  providerKind: "llm" | "akismet";
  enabled: boolean;
  model: string;
  apiBase: string;
  apiKey: string;
  rule: string;
  blogUrl: string;
};

export const ADMIN_TABS: AdminTabItem[] = [
  {
    key: "sites",
    label: "站点管理",
    description: "查看站点配置与评论基础参数。",
  },
  {
    key: "comments",
    label: "评论管理",
    description: "处理待审核评论与风险内容。",
  },
  {
    key: "users",
    label: "用户管理",
    description: "查看用户、外部身份与当前角色绑定。",
  },
  {
    key: "permissions",
    label: "权限管理",
    description: "编辑角色权限，以及用户在全局或站点上的角色绑定。",
  },
  {
    key: "oauthProviders",
    label: "OAuth 提供者",
    description: "管理社交登录与 OAuth 配置。",
  },
  {
    key: "moderationProviders",
    label: "审核提供者",
    description: "管理 LLM、Akismet 等审核来源。",
  },
  {
    key: "externalProviders",
    label: "外部身份提供者",
    description: "外部 token exchange 尚未提供管理接口。",
  },
];
