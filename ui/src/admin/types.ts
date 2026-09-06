import type { RuntimeOptions } from "@/config/types";
import type { MessageKey } from "@/shared/i18n";

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
  labelKey: MessageKey;
  descriptionKey: MessageKey;
};

export type CommentTabItem = {
  key: CommentTab;
  labelKey: MessageKey;
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
    labelKey: "admin.tab.sites",
    descriptionKey: "admin.tab.sitesDesc",
  },
  {
    key: "comments",
    labelKey: "admin.tab.comments",
    descriptionKey: "admin.tab.commentsDesc",
  },
  {
    key: "users",
    labelKey: "admin.tab.users",
    descriptionKey: "admin.tab.usersDesc",
  },
  {
    key: "permissions",
    labelKey: "admin.tab.permissions",
    descriptionKey: "admin.tab.permissionsDesc",
  },
  {
    key: "oauthProviders",
    labelKey: "admin.tab.oauth",
    descriptionKey: "admin.tab.oauthDesc",
  },
  {
    key: "moderationProviders",
    labelKey: "admin.tab.moderation",
    descriptionKey: "admin.tab.moderationDesc",
  },
  {
    key: "externalProviders",
    labelKey: "admin.tab.external",
    descriptionKey: "admin.tab.externalDesc",
  },
];
