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
  maxCommentLength: string;
  commentLimitSeconds: string;
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
    description: "为后续用户列表、角色绑定预留位置。",
  },
  {
    key: "permissions",
    label: "权限管理",
    description: "管理角色、权限与站点级授权范围。",
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
    description: "为宿主系统登录态和外部 SSO 预留。",
  },
];
