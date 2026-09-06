import { http } from "@/shared/api/client";
import type {
  AdminCapabilities,
  CommentForAdmin,
  ModerationProvider,
  ModerationProviderConfig,
  ModerationProviderKind,
  OauthProvider,
  Paged,
  PermissionForAdmin,
  RoleForAdmin,
  Site,
  SiteConfig,
  UserForAdmin,
  UserProfile,
  UserRoleBindingForAdmin,
} from "@/shared/api/types";

export const fetchAdminProfile = () => {
  return http.get<UserProfile>("/api/users/me");
};

export const fetchAdminSites = () => {
  return http.get<Site[]>("/api/sites");
};

export const createAdminSite = (payload: {
  name: string;
  url: string;
  config: SiteConfig;
}) => {
  return http.post<Site>("/api/sites", payload);
};

export const fetchAdminComments = (params: {
  site_id: number;
  page_path: string;
  page_size: number;
  page_offset: number;
  sort: string;
  status?: "pending" | "approved" | "spam" | "deleted";
}) => {
  return http.get<Paged<CommentForAdmin[]>>("/api/admin/comments", { params });
};

export const updateAdminSite = (payload: {
  id: number;
  name?: string;
  url?: string;
  config?: SiteConfig;
}) => {
  return http.patch<Site>("/api/sites", payload);
};

export const updateAdminCommentStatus = (
  id: number,
  payload: {
    status: "pending" | "approved" | "spam" | "deleted";
  },
) => {
  return http.patch<void>(`/api/admin/comments/${id}`, payload);
};

export const fetchAdminCapabilities = () => {
  return http.get<AdminCapabilities>("/api/admin/me/capabilities");
};

export const fetchAdminRoles = () => {
  return http.get<RoleForAdmin[]>("/api/admin/rbac/roles");
};

export const fetchAdminPermissions = () => {
  return http.get<PermissionForAdmin[]>("/api/admin/rbac/permissions");
};

export const fetchAdminUserRoleBindings = () => {
  return http.get<UserRoleBindingForAdmin[]>(
    "/api/admin/rbac/user-role-bindings",
  );
};

export const fetchAdminUsers = () => {
  return http.get<UserForAdmin[]>("/api/admin/users");
};

export const replaceAdminRolePermissions = (
  roleId: number,
  permission_names: string[],
) => {
  return http.patch<RoleForAdmin>(
    `/api/admin/rbac/roles/${roleId}/permissions`,
    { permission_names },
  );
};

export const createAdminUserRoleBinding = (payload: {
  user_id: number;
  role_id: number;
  scope_type: "global" | "site";
  scope_id?: string | null;
}) => {
  return http.post<UserRoleBindingForAdmin>(
    "/api/admin/rbac/user-role-bindings",
    payload,
  );
};

export const deleteAdminUserRoleBinding = (id: number) => {
  return http.delete<void>(`/api/admin/rbac/user-role-bindings/${id}`);
};

export const fetchAdminModerationProviders = () => {
  return http.get<ModerationProvider[]>("/api/admin/moderation/providers");
};

export const createAdminModerationProvider = (payload: {
  site_id: number;
  provider_kind: ModerationProviderKind;
  enabled: boolean;
  config: ModerationProviderConfig;
}) => {
  return http.post<ModerationProvider>(
    "/api/admin/moderation/providers",
    payload,
  );
};

export const updateAdminModerationProvider = (
  id: number,
  payload: {
    enabled?: boolean;
    config?: ModerationProviderConfig;
  },
) => {
  return http.patch<ModerationProvider>(
    `/api/admin/moderation/providers/${id}`,
    payload,
  );
};

export const fetchAdminOauthProviders = () => {
  return http.get<OauthProvider[]>("/api/admin/oauth/providers");
};

export const createAdminOauthProvider = (payload: {
  site_id?: number | null;
  provider_code: string;
  enabled?: boolean;
  client_id: string;
  client_secret: string;
  redirect_uri: string;
}) => {
  return http.post<OauthProvider>("/api/admin/oauth/providers", payload);
};

export const updateAdminOauthProvider = (
  id: number,
  payload: {
    enabled?: boolean;
    client_id?: string;
    client_secret?: string;
    redirect_uri?: string;
  },
) => {
  return http.patch<OauthProvider>(`/api/admin/oauth/providers/${id}`, payload);
};
