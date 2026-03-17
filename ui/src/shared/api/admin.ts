import { http } from "./client";
import type {
	AdminCapabilities,
	CommentForAdmin,
	Paged,
	PermissionForAdmin,
	RoleForAdmin,
	Site,
	SiteConfig,
	UserProfile,
	UserRoleBindingForAdmin,
} from "./types";

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
	return http.get<UserRoleBindingForAdmin[]>("/api/admin/rbac/user-role-bindings");
};
