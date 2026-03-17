import { useEffect, useState } from "preact/hooks";

import { Button } from "@/shared/components/Button";
import { formatLocalDateTime } from "@/shared/helper";
import {
	createAdminSite,
	fetchAdminCapabilities,
	fetchAdminComments,
	fetchAdminPermissions,
	fetchAdminProfile,
	fetchAdminRoles,
	fetchAdminSites,
	fetchAdminUserRoleBindings,
	updateAdminCommentStatus,
	updateAdminSite,
} from "@/shared/api";
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
} from "@/shared/api/types";

type AdminTab =
	| "sites"
	| "comments"
	| "users"
	| "permissions"
	| "oauthProviders"
	| "moderationProviders"
	| "externalProviders";
type CommentTab = "all" | "pending" | "spam" | "deleted";

const TABS: { key: AdminTab; label: string; description: string }[] = [
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

const panelClass =
	"rounded-xl border border-[var(--yo-surface-strong)] bg-[var(--yo-surface)] p-5 shadow-sm";

const COMMENT_TABS: { key: CommentTab; label: string }[] = [
	{ key: "all", label: "全部评论" },
	{ key: "pending", label: "待审核" },
	{ key: "spam", label: "垃圾评论" },
	{ key: "deleted", label: "已删除" },
];

const COMMENT_PAGE_SIZE = 20;

const COMMENT_STATUS_OPTIONS = [
	{ value: "pending", label: "待审核" },
	{ value: "approved", label: "已通过" },
	{ value: "spam", label: "垃圾" },
	{ value: "deleted", label: "已删除" },
] as const;

type SiteFormState = {
	name: string;
	url: string;
	allowAnonymous: boolean;
	maxCommentLength: string;
	commentLimitSeconds: string;
};

function createSiteFormState(site: Site): SiteFormState {
	return {
		name: site.name,
		url: site.url,
		allowAnonymous: site.config.allow_anonymous,
		maxCommentLength: String(site.config.max_comment_length),
		commentLimitSeconds: String(site.config.comment_limit_seconds),
	};
}

function getCommentStatusFilter(tab: CommentTab) {
	if (tab === "all") return undefined;
	if (tab === "pending") return "pending" as const;
	if (tab === "spam") return "spam" as const;
	return "deleted" as const;
}

function matchesCommentTab(
	tab: CommentTab,
	status: CommentForAdmin["status"],
) {
	if (tab === "all") return true;
	if (tab === "pending") return status === "pending";
	if (tab === "spam") return status === "spam";
	return status === "deleted";
}

const App = () => {
	const [activeTab, setActiveTab] = useState<AdminTab>("sites");
	const [activeCommentTab, setActiveCommentTab] = useState<CommentTab>("all");
	const [profile, setProfile] = useState<UserProfile | null>(null);
	const [sites, setSites] = useState<Site[]>([]);
	const [commentsPage, setCommentsPage] = useState<Paged<CommentForAdmin[]> | null>(null);
	const [selectedCommentSiteId, setSelectedCommentSiteId] = useState<number | null>(null);
	const [commentPagePath, setCommentPagePath] = useState("/");
	const [commentPageOffset, setCommentPageOffset] = useState(1);
	const [isLoadingProfile, setIsLoadingProfile] = useState(true);
	const [isLoadingSites, setIsLoadingSites] = useState(false);
	const [isLoadingComments, setIsLoadingComments] = useState(false);
	const [profileError, setProfileError] = useState("");
	const [sitesError, setSitesError] = useState("");
	const [commentsError, setCommentsError] = useState("");
	const [editingSiteId, setEditingSiteId] = useState<number | null>(null);
	const [siteForm, setSiteForm] = useState<SiteFormState | null>(null);
	const [siteFormError, setSiteFormError] = useState("");
	const [isSavingSite, setIsSavingSite] = useState(false);
	const [isCreatingSite, setIsCreatingSite] = useState(false);
	const [createSiteForm, setCreateSiteForm] = useState<SiteFormState>({
		name: "",
		url: "",
		allowAnonymous: true,
		maxCommentLength: "5000",
		commentLimitSeconds: "30",
	});
	const [createSiteError, setCreateSiteError] = useState("");
	const [isSubmittingCreateSite, setIsSubmittingCreateSite] = useState(false);
	const [updatingCommentId, setUpdatingCommentId] = useState<number | null>(null);
	const [capabilities, setCapabilities] = useState<AdminCapabilities | null>(null);
	const [roles, setRoles] = useState<RoleForAdmin[]>([]);
	const [permissions, setPermissions] = useState<PermissionForAdmin[]>([]);
	const [roleBindings, setRoleBindings] = useState<UserRoleBindingForAdmin[]>([]);
	const [isLoadingPermissions, setIsLoadingPermissions] = useState(false);
	const [permissionsError, setPermissionsError] = useState("");

	useEffect(() => {
		let alive = true;
		setIsLoadingProfile(true);
		setProfileError("");
		fetchAdminProfile()
			.then((res) => {
				if (!alive) return;
				setProfile(res.data);
			})
			.catch((error) => {
				if (!alive) return;
				setProfileError(error instanceof Error ? error.message : "加载管理员信息失败");
			})
			.finally(() => {
				if (!alive) return;
				setIsLoadingProfile(false);
			});
		return () => {
			alive = false;
		};
	}, []);

	useEffect(() => {
		if (activeTab !== "sites") return;
		let alive = true;
		setIsLoadingSites(true);
		setSitesError("");
		fetchAdminSites()
			.then((res) => {
				if (!alive) return;
				setSites(res.data);
			})
			.catch((error) => {
				if (!alive) return;
				setSitesError(error instanceof Error ? error.message : "加载站点失败");
			})
			.finally(() => {
				if (!alive) return;
				setIsLoadingSites(false);
			});
		return () => {
			alive = false;
		};
	}, [activeTab]);

	useEffect(() => {
		if (sites.length === 0) return;
		setSelectedCommentSiteId((current) => current ?? sites[0].id);
	}, [sites]);

	useEffect(() => {
		setCommentPageOffset(1);
	}, [activeCommentTab, selectedCommentSiteId, commentPagePath]);

	useEffect(() => {
		if (activeTab !== "sites") {
			setEditingSiteId(null);
			setSiteForm(null);
			setSiteFormError("");
		}
	}, [activeTab]);

	useEffect(() => {
		if (activeTab !== "comments" || selectedCommentSiteId == null) return;
		let alive = true;
		setIsLoadingComments(true);
		setCommentsError("");
		fetchAdminComments({
			site_id: selectedCommentSiteId,
			page_path: commentPagePath,
			page_size: COMMENT_PAGE_SIZE,
			page_offset: commentPageOffset,
			sort: "created_desc",
			status: getCommentStatusFilter(activeCommentTab),
		})
			.then((res) => {
				if (!alive) return;
				setCommentsPage(res.data);
			})
			.catch((error) => {
				if (!alive) return;
				setCommentsError(error instanceof Error ? error.message : "加载评论列表失败");
			})
			.finally(() => {
				if (!alive) return;
				setIsLoadingComments(false);
			});
		return () => {
			alive = false;
		};
	}, [activeTab, activeCommentTab, selectedCommentSiteId, commentPagePath, commentPageOffset]);

	useEffect(() => {
		if (activeTab !== "permissions") return;
		let alive = true;
		setIsLoadingPermissions(true);
		setPermissionsError("");
		Promise.all([
			fetchAdminCapabilities(),
			fetchAdminRoles(),
			fetchAdminPermissions(),
			fetchAdminUserRoleBindings(),
		])
			.then(([capabilitiesRes, rolesRes, permissionsRes, bindingsRes]) => {
				if (!alive) return;
				setCapabilities(capabilitiesRes.data);
				setRoles(rolesRes.data);
				setPermissions(permissionsRes.data);
				setRoleBindings(bindingsRes.data);
			})
			.catch((error) => {
				if (!alive) return;
				setPermissionsError(
					error instanceof Error ? error.message : "加载权限管理数据失败",
				);
			})
			.finally(() => {
				if (!alive) return;
				setIsLoadingPermissions(false);
			});
		return () => {
			alive = false;
		};
	}, [activeTab]);

	const beginEditSite = (site: Site) => {
		setEditingSiteId(site.id);
		setSiteForm(createSiteFormState(site));
		setSiteFormError("");
	};

	const cancelEditSite = () => {
		setEditingSiteId(null);
		setSiteForm(null);
		setSiteFormError("");
	};

	const saveSite = async () => {
		if (editingSiteId == null || siteForm == null) return;
		const maxCommentLength = Number(siteForm.maxCommentLength);
		const commentLimitSeconds = Number(siteForm.commentLimitSeconds);

		if (!siteForm.name.trim()) {
			setSiteFormError("站点名称不能为空");
			return;
		}
		if (!siteForm.url.trim()) {
			setSiteFormError("站点地址不能为空");
			return;
		}
		if (!Number.isFinite(maxCommentLength) || maxCommentLength <= 0) {
			setSiteFormError("最大长度必须是大于 0 的数字");
			return;
		}
		if (!Number.isFinite(commentLimitSeconds) || commentLimitSeconds < 0) {
			setSiteFormError("限流秒数必须是大于等于 0 的数字");
			return;
		}

		setIsSavingSite(true);
		setSiteFormError("");

		const config: SiteConfig = {
			allow_anonymous: siteForm.allowAnonymous,
			max_comment_length: maxCommentLength,
			comment_limit_seconds: commentLimitSeconds,
		};

		try {
			const res = await updateAdminSite({
				id: editingSiteId,
				name: siteForm.name.trim(),
				url: siteForm.url.trim(),
				config,
			});
			setSites((current) =>
				current.map((site) => (site.id === editingSiteId ? res.data : site)),
			);
			setEditingSiteId(null);
			setSiteForm(null);
		} catch (error) {
			setSiteFormError(error instanceof Error ? error.message : "保存站点失败");
		} finally {
			setIsSavingSite(false);
		}
	};

	const createSite = async () => {
		const maxCommentLength = Number(createSiteForm.maxCommentLength);
		const commentLimitSeconds = Number(createSiteForm.commentLimitSeconds);

		if (!createSiteForm.name.trim()) {
			setCreateSiteError("站点名称不能为空");
			return;
		}
		if (!createSiteForm.url.trim()) {
			setCreateSiteError("站点地址不能为空");
			return;
		}
		if (!Number.isFinite(maxCommentLength) || maxCommentLength <= 0) {
			setCreateSiteError("最大长度必须是大于 0 的数字");
			return;
		}
		if (!Number.isFinite(commentLimitSeconds) || commentLimitSeconds < 0) {
			setCreateSiteError("限流秒数必须是大于等于 0 的数字");
			return;
		}

		setIsSubmittingCreateSite(true);
		setCreateSiteError("");

		try {
			const res = await createAdminSite({
				name: createSiteForm.name.trim(),
				url: createSiteForm.url.trim(),
				config: {
					allow_anonymous: createSiteForm.allowAnonymous,
					max_comment_length: maxCommentLength,
					comment_limit_seconds: commentLimitSeconds,
				},
			});
			setSites((current) => [res.data, ...current]);
			setIsCreatingSite(false);
			setCreateSiteForm({
				name: "",
				url: "",
				allowAnonymous: true,
				maxCommentLength: "5000",
				commentLimitSeconds: "30",
			});
		} catch (error) {
			setCreateSiteError(error instanceof Error ? error.message : "创建站点失败");
		} finally {
			setIsSubmittingCreateSite(false);
		}
	};

	const comments = commentsPage?.items ?? [];
	const activeCommentTabLabel =
		COMMENT_TABS.find((tab) => tab.key === activeCommentTab)?.label ?? "评论";
	const globalPermissions = capabilities?.global_permissions ?? [];
	const sitePermissionEntries = Object.entries(capabilities?.site_permissions ?? {});

	const handleUpdateCommentStatus = async (
		commentId: number,
		status: CommentForAdmin["status"],
	) => {
		setUpdatingCommentId(commentId);
		setCommentsError("");
		try {
			await updateAdminCommentStatus(commentId, { status });
			setCommentsPage((current) => {
				if (!current) return current;
				const updatedItems = current.items
					.map((comment) =>
						comment.id === commentId ? { ...comment, status } : comment,
					)
					.filter((comment) => matchesCommentTab(activeCommentTab, comment.status));
				const total =
					activeCommentTab === "all" || matchesCommentTab(activeCommentTab, status)
						? current.total
						: Math.max(0, current.total - 1);
				const totalPages = Math.max(1, Math.ceil(total / current.page_size));
				return {
					...current,
					items: updatedItems,
					total,
					total_pages: totalPages,
				};
			});
		} catch (error) {
			setCommentsError(error instanceof Error ? error.message : "更新评论状态失败");
		} finally {
			setUpdatingCommentId(null);
		}
	};

	return (
		<div className="min-h-screen bg-[var(--yo-bg)] text-[var(--yo-text)]">
			<div className="mx-auto flex min-h-screen max-w-full flex-col gap-6 px-4 py-6 lg:flex-row lg:px-6">
				<aside className="w-full shrink-0 lg:w-72">
					<div className={`${panelClass} lg:sticky lg:top-6`}>
						<div className="mb-5">
							<h1 className="mt-2 text-2xl font-semibold">Yoin Admin</h1>
							{/* <p className="mt-2 text-sm text-[var(--yo-text-muted)]">
							</p> */}
						</div>

						<div className="mb-5 rounded-lg bg-[var(--yo-surface-soft)] p-4">
							<p className="text-xs text-[var(--yo-text-soft)]">当前身份</p>
							{isLoadingProfile ? (
								<p className="mt-2 text-sm text-[var(--yo-text-muted)]">正在加载管理员信息...</p>
							) : profile ? (
								<div className="mt-2 space-y-1">
									<p className="font-medium">{profile.nickname}</p>
									<p className="text-sm text-[var(--yo-text-muted)]">{profile.email}</p>
								</div>
							) : (
								<p className="mt-2 text-sm text-[var(--yo-danger)]">
									{profileError || "未获取到管理员信息"}
								</p>
							)}
						</div>

						<nav className="flex gap-2 overflow-x-auto pb-1 lg:block lg:space-y-2 lg:overflow-visible lg:pb-0">
							{TABS.map((tab) => {
								const isActive = activeTab === tab.key;
								return (
									<button
										key={tab.key}
										type="button"
										onClick={() => setActiveTab(tab.key)}
										className={`min-w-52 rounded-lg border px-4 py-3 text-left transition-colors lg:w-full lg:min-w-0 ${isActive
											? "border-[var(--yo-primary)] bg-[var(--yo-primary)] text-[var(--yo-primary-contrast)]"
											: "border-[var(--yo-surface-strong)] bg-[var(--yo-surface)] hover:bg-[var(--yo-surface-soft)]"
											}`}
									>
										<p className="font-medium">{tab.label}</p>
										<p
											className={`mt-1 text-xs ${isActive
												? "text-[color:color-mix(in_srgb,var(--yo-primary-contrast)_72%,transparent)]"
												: "text-[var(--yo-text-muted)]"
												}`}
										>
											{tab.description}
										</p>
									</button>
								);
							})}
						</nav>
					</div>
				</aside>

				<main className="min-w-0 flex-1">
					{activeTab === "sites" && (
						<section className={panelClass}>
							<div className="mb-4 flex flex-col items-start gap-3 sm:flex-row sm:items-center sm:justify-between">
								<div>
									<h2 className="text-xl font-semibold">站点管理</h2>
									<p className="mt-1 text-sm text-[var(--yo-text-muted)]">
										站点列表已经接上，配置修改也可以直接在这里就地保存。
									</p>
								</div>
								<Button
									size="sm"
									onClick={() => {
										setIsCreatingSite((current) => !current);
										setCreateSiteError("");
									}}
								>
									{isCreatingSite ? "收起表单" : "新建站点"}
								</Button>
							</div>

							{isCreatingSite && (
								<div className="mb-4 rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] p-4">
									<h3 className="text-base font-medium">创建新站点</h3>
									<p className="mt-1 text-sm text-[var(--yo-text-muted)]">
										先把站点名称、URL 和评论基础限制填好，后面再继续补其它站点级配置。
									</p>

									<div className="mt-4 grid gap-4 md:grid-cols-2">
										<label className="block">
											<span className="text-xs text-[var(--yo-text-soft)]">站点名称</span>
											<input
												type="text"
												value={createSiteForm.name}
												onInput={(event) =>
													setCreateSiteForm((current) => ({
														...current,
														name: event.currentTarget.value,
													}))
												}
												className="mt-1 w-full rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-bg)] px-3 py-2 text-sm outline-none transition-colors focus:border-[var(--yo-primary)]"
											/>
										</label>

										<label className="block">
											<span className="text-xs text-[var(--yo-text-soft)]">站点地址</span>
											<input
												type="url"
												value={createSiteForm.url}
												onInput={(event) =>
													setCreateSiteForm((current) => ({
														...current,
														url: event.currentTarget.value,
													}))
												}
												className="mt-1 w-full rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-bg)] px-3 py-2 text-sm outline-none transition-colors focus:border-[var(--yo-primary)]"
											/>
										</label>

										<label className="block">
											<span className="text-xs text-[var(--yo-text-soft)]">最大评论长度</span>
											<input
												type="number"
												min="1"
												value={createSiteForm.maxCommentLength}
												onInput={(event) =>
													setCreateSiteForm((current) => ({
														...current,
														maxCommentLength: event.currentTarget.value,
													}))
												}
												className="mt-1 w-full rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-bg)] px-3 py-2 text-sm outline-none transition-colors focus:border-[var(--yo-primary)]"
											/>
										</label>

										<label className="block">
											<span className="text-xs text-[var(--yo-text-soft)]">评论限流秒数</span>
											<input
												type="number"
												min="0"
												value={createSiteForm.commentLimitSeconds}
												onInput={(event) =>
													setCreateSiteForm((current) => ({
														...current,
														commentLimitSeconds: event.currentTarget.value,
													}))
												}
												className="mt-1 w-full rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-bg)] px-3 py-2 text-sm outline-none transition-colors focus:border-[var(--yo-primary)]"
											/>
										</label>
									</div>

									<label className="mt-4 inline-flex items-center gap-2 text-sm text-[var(--yo-text-muted)]">
										<input
											type="checkbox"
											checked={createSiteForm.allowAnonymous}
											onChange={(event) =>
												setCreateSiteForm((current) => ({
													...current,
													allowAnonymous: event.currentTarget.checked,
												}))
											}
											className="h-4 w-4 rounded border border-[var(--yo-surface-strong)]"
										/>
										允许匿名评论
									</label>

									{createSiteError && (
										<p className="mt-3 text-sm text-[var(--yo-danger)]">{createSiteError}</p>
									)}

									<div className="mt-4 flex flex-wrap gap-2">
										<Button size="sm" loading={isSubmittingCreateSite} onClick={createSite}>
											创建站点
										</Button>
										<Button
											size="sm"
											variant="ghost"
											disabled={isSubmittingCreateSite}
											onClick={() => {
												setIsCreatingSite(false);
												setCreateSiteError("");
											}}
										>
											取消
										</Button>
									</div>
								</div>
							)}

							{isLoadingSites ? (
								<p className="text-sm text-[var(--yo-text-muted)]">正在加载站点列表...</p>
							) : sitesError ? (
								<div className="rounded-lg bg-[var(--yo-danger-bg)] px-4 py-3 text-sm text-[var(--yo-danger)]">
									{sitesError}
								</div>
							) : sites.length === 0 ? (
								<div className="rounded-lg border border-dashed border-[var(--yo-surface-strong)] px-4 py-8 text-center text-sm text-[var(--yo-text-muted)]">
									还没有站点，先从这里开始接入你的第一个评论站点。
								</div>
							) : (
								<div className="grid gap-4 xl:grid-cols-2">
									{sites.map((site) => (
										<article
											key={site.id}
											className="rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] p-4"
										>
											<div className="flex items-start justify-between gap-3">
												<div>
													<h3 className="font-medium">{site.name}</h3>
													<p className="mt-1 break-all text-sm text-[var(--yo-text-muted)]">
														{site.url}
													</p>
												</div>
												<div className="flex items-center gap-2">
													<span className="rounded-full bg-[var(--yo-surface)] px-2.5 py-1 text-xs text-[var(--yo-text-muted)]">
														ID {site.id}
													</span>
													<Button
														size="sm"
														variant="secondary"
														onClick={() => beginEditSite(site)}
													>
														编辑
													</Button>
												</div>
											</div>
											<div className="mt-4 grid grid-cols-3 gap-3 text-sm">
												<div>
													<p className="text-[var(--yo-text-soft)]">匿名评论</p>
													<p className="mt-1 font-medium">
														{site.config.allow_anonymous ? "允许" : "关闭"}
													</p>
												</div>
												<div>
													<p className="text-[var(--yo-text-soft)]">最大长度</p>
													<p className="mt-1 font-medium">{site.config.max_comment_length}</p>
												</div>
												<div>
													<p className="text-[var(--yo-text-soft)]">限流秒数</p>
													<p className="mt-1 font-medium">
														{site.config.comment_limit_seconds}
													</p>
												</div>
											</div>

											{editingSiteId === site.id && siteForm && (
												<div className="mt-4 rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface)] p-4">
													<div className="grid gap-4 md:grid-cols-2">
														<label className="block">
															<span className="text-xs text-[var(--yo-text-soft)]">站点名称</span>
															<input
																type="text"
																value={siteForm.name}
																onInput={(event) =>
																	setSiteForm((current) =>
																		current
																			? {
																				...current,
																				name: event.currentTarget.value,
																			}
																			: current,
																	)
																}
																className="mt-1 w-full rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-bg)] px-3 py-2 text-sm outline-none transition-colors focus:border-[var(--yo-primary)]"
															/>
														</label>

														<label className="block">
															<span className="text-xs text-[var(--yo-text-soft)]">站点地址</span>
															<input
																type="url"
																value={siteForm.url}
																onInput={(event) =>
																	setSiteForm((current) =>
																		current
																			? {
																				...current,
																				url: event.currentTarget.value,
																			}
																			: current,
																	)
																}
																className="mt-1 w-full rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-bg)] px-3 py-2 text-sm outline-none transition-colors focus:border-[var(--yo-primary)]"
															/>
														</label>

														<label className="block">
															<span className="text-xs text-[var(--yo-text-soft)]">最大评论长度</span>
															<input
																type="number"
																min="1"
																value={siteForm.maxCommentLength}
																onInput={(event) =>
																	setSiteForm((current) =>
																		current
																			? {
																				...current,
																				maxCommentLength: event.currentTarget.value,
																			}
																			: current,
																	)
																}
																className="mt-1 w-full rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-bg)] px-3 py-2 text-sm outline-none transition-colors focus:border-[var(--yo-primary)]"
															/>
														</label>

														<label className="block">
															<span className="text-xs text-[var(--yo-text-soft)]">评论限流秒数</span>
															<input
																type="number"
																min="0"
																value={siteForm.commentLimitSeconds}
																onInput={(event) =>
																	setSiteForm((current) =>
																		current
																			? {
																				...current,
																				commentLimitSeconds: event.currentTarget.value,
																			}
																			: current,
																	)
																}
																className="mt-1 w-full rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-bg)] px-3 py-2 text-sm outline-none transition-colors focus:border-[var(--yo-primary)]"
															/>
														</label>
													</div>

													<label className="mt-4 inline-flex items-center gap-2 text-sm text-[var(--yo-text-muted)]">
														<input
															type="checkbox"
															checked={siteForm.allowAnonymous}
															onChange={(event) =>
																setSiteForm((current) =>
																	current
																		? {
																			...current,
																			allowAnonymous: event.currentTarget.checked,
																		}
																		: current,
																)
															}
															className="h-4 w-4 rounded border border-[var(--yo-surface-strong)]"
														/>
														允许匿名评论
													</label>

													{siteFormError && (
														<p className="mt-3 text-sm text-[var(--yo-danger)]">{siteFormError}</p>
													)}

													<div className="mt-4 flex flex-wrap gap-2">
														<Button size="sm" loading={isSavingSite} onClick={saveSite}>
															保存配置
														</Button>
														<Button
															size="sm"
															variant="ghost"
															disabled={isSavingSite}
															onClick={cancelEditSite}
														>
															取消
														</Button>
													</div>
												</div>
											)}
										</article>
									))}
								</div>
							)}
						</section>
					)}

					{activeTab === "comments" && (
						<section className={panelClass}>
							<div className="mb-4 flex flex-col items-start gap-3 sm:flex-row sm:items-center sm:justify-between">
								<div>
									<h2 className="text-xl font-semibold">评论管理</h2>
									<p className="mt-1 text-sm text-[var(--yo-text-muted)]">
										现在先统一接到后台评论列表接口，按站点、页面和状态做筛选。
									</p>
								</div>
								<Button
									size="sm"
									variant="secondary"
									onClick={() => {
										if (selectedCommentSiteId == null) return;
										setIsLoadingComments(true);
										setCommentsError("");
										fetchAdminComments({
											site_id: selectedCommentSiteId,
											page_path: commentPagePath,
											page_size: COMMENT_PAGE_SIZE,
											page_offset: commentPageOffset,
											sort: "created_desc",
											status: getCommentStatusFilter(activeCommentTab),
										})
											.then((res) => setCommentsPage(res.data))
											.catch((error) =>
												setCommentsError(
													error instanceof Error ? error.message : "加载评论列表失败",
												),
											)
											.finally(() => setIsLoadingComments(false));
									}}
								>
									刷新列表
								</Button>
							</div>

							<div className="mb-4 grid gap-3 md:grid-cols-[minmax(0,240px)_minmax(0,1fr)]">
								<label className="block">
									<span className="text-xs text-[var(--yo-text-soft)]">站点</span>
									<select
										value={selectedCommentSiteId ?? ""}
										onChange={(event) => setSelectedCommentSiteId(Number(event.currentTarget.value))}
										className="mt-1 w-full rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-bg)] px-3 py-2 text-sm outline-none transition-colors focus:border-[var(--yo-primary)]"
									>
										{sites.map((site) => (
											<option key={site.id} value={site.id}>
												{site.name}
											</option>
										))}
									</select>
								</label>

								<label className="block">
									<span className="text-xs text-[var(--yo-text-soft)]">页面路径</span>
									<input
										type="text"
										value={commentPagePath}
										onInput={(event) => setCommentPagePath(event.currentTarget.value || "/")}
										className="mt-1 w-full rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-bg)] px-3 py-2 text-sm outline-none transition-colors focus:border-[var(--yo-primary)]"
										placeholder="/"
									/>
								</label>
							</div>

							<div className="mb-4 flex gap-2 overflow-x-auto pb-1">
								{COMMENT_TABS.map((tab) => {
									const isActive = activeCommentTab === tab.key;
									return (
										<button
											key={tab.key}
											type="button"
											onClick={() => setActiveCommentTab(tab.key)}
											className={`rounded-full border px-3 py-1.5 text-sm whitespace-nowrap transition-colors ${isActive
												? "border-[var(--yo-primary)] bg-[var(--yo-primary)] text-[var(--yo-primary-contrast)]"
												: "border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] text-[var(--yo-text-muted)] hover:bg-[var(--yo-surface)]"
												}`}
										>
											{tab.label}
										</button>
									);
								})}
							</div>

							<div className="mb-4 rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] px-4 py-3 text-sm text-[var(--yo-text-muted)]">
								当前视图：<span className="font-medium text-[var(--yo-text)]">{activeCommentTabLabel}</span>。
								状态接口接上后，我们再按真实状态显示对应动作，而不是所有面板都摆同一组按钮。
							</div>

							{isLoadingComments ? (
								<p className="text-sm text-[var(--yo-text-muted)]">正在加载评论列表...</p>
							) : commentsError ? (
								<div className="rounded-lg bg-[var(--yo-danger-bg)] px-4 py-3 text-sm text-[var(--yo-danger)]">
									{commentsError}
								</div>
							) : selectedCommentSiteId == null ? (
								<div className="rounded-lg border border-dashed border-[var(--yo-surface-strong)] px-4 py-8 text-center text-sm text-[var(--yo-text-muted)]">
									请先创建站点，再查看评论管理。
								</div>
							) : comments.length === 0 ? (
								<div className="rounded-lg border border-dashed border-[var(--yo-surface-strong)] px-4 py-8 text-center text-sm text-[var(--yo-text-muted)]">
									当前筛选条件下没有评论，或者后端过滤逻辑还在继续完善。
								</div>
							) : (
								<div className="space-y-3">
									{comments.map((comment) => (
										<article
											key={comment.id}
											className="rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] p-4"
										>
											<div className="flex items-start justify-between gap-3">
												<div>
													<div className="flex flex-wrap items-center gap-2">
														<h3 className="font-medium">{comment.nickname}</h3>
														<span className="rounded-full bg-[var(--yo-surface)] px-2 py-0.5 text-[11px] text-[var(--yo-text-muted)]">
															ID {comment.id}
														</span>
														{comment.parent_id != null && (
															<span className="rounded-full bg-[var(--yo-surface)] px-2 py-0.5 text-[11px] text-[var(--yo-text-muted)]">
																回复 #{comment.parent_id}
															</span>
														)}
														{comment.thread_id != null && (
															<span className="rounded-full bg-[var(--yo-surface)] px-2 py-0.5 text-[11px] text-[var(--yo-text-muted)]">
																Thread #{comment.thread_id}
															</span>
														)}
													</div>
													<div className="mt-2 flex flex-wrap gap-x-4 gap-y-1 text-xs text-[var(--yo-text-muted)]">
														<span>创建：{formatLocalDateTime(comment.created_at)}</span>
														<span>更新：{formatLocalDateTime(comment.updated_at)}</span>
														{comment.website && <span>主页：{comment.website}</span>}
														{comment.device && <span>设备：{comment.device}</span>}
														{comment.location && <span>地区：{comment.location}</span>}
														<span>赞同：{comment.up_vote}</span>
														<span>反对：{comment.down_vote}</span>
													</div>
												</div>
												<div className="flex flex-wrap justify-end gap-1">
													{COMMENT_STATUS_OPTIONS.map((option) => {
														const isActive = comment.status === option.value;
														return (
															<button
																key={option.value}
																type="button"
																disabled={updatingCommentId === comment.id || isActive}
																aria-pressed={isActive}
																onClick={() =>
																	handleUpdateCommentStatus(comment.id, option.value)
																}
																className={`rounded-full border px-2.5 py-1 text-xs transition-colors ${isActive
																	? "border-[var(--yo-primary)] bg-[var(--yo-primary)] text-[var(--yo-primary-contrast)]"
																	: "border-[var(--yo-surface-strong)] bg-[var(--yo-surface)] text-[var(--yo-text-muted)] hover:bg-[var(--yo-surface-soft)] disabled:hover:bg-[var(--yo-surface)]"
																	}`}
															>
																{updatingCommentId === comment.id && option.value === comment.status
																	? "更新中..."
																	: option.label}
															</button>
														);
													})}
												</div>
											</div>
											<div
												className="mt-3 rounded-lg bg-[var(--yo-surface)] px-4 py-3 text-sm leading-6"
												dangerouslySetInnerHTML={{ __html: comment.content }}
											/>
										</article>
									))}

									{commentsPage && commentsPage.total_pages > 0 && (
										<div className="flex flex-col gap-3 rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] px-4 py-3 text-sm text-[var(--yo-text-muted)] sm:flex-row sm:items-center sm:justify-between">
											<p>
												共 {commentsPage.total} 条，当前第 {commentsPage.page_offset} 页 / 共{" "}
												{commentsPage.total_pages} 页
											</p>
											<div className="flex gap-2">
												<Button
													size="sm"
													variant="ghost"
													disabled={commentPageOffset <= 1}
													onClick={() =>
														setCommentPageOffset((current) => Math.max(1, current - 1))
													}
												>
													上一页
												</Button>
												<Button
													size="sm"
													variant="secondary"
													disabled={commentPageOffset >= commentsPage.total_pages}
													onClick={() =>
														setCommentPageOffset((current) =>
															Math.min(commentsPage.total_pages, current + 1),
														)
													}
												>
													下一页
												</Button>
											</div>
										</div>
									)}
								</div>
							)}
						</section>
					)}

					{activeTab === "users" && (
						<section className={panelClass}>
							<h2 className="text-xl font-semibold">用户管理</h2>
							<p className="mt-2 text-sm text-[var(--yo-text-muted)]">
								用户管理页先把信息架子搭好。当前后端还没有用户列表 / 角色绑定管理接口，所以这里先保留为控制台占位区。
							</p>

							<div className="mt-5 grid gap-4 md:grid-cols-2">
								<div className="rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] p-4">
									<p className="text-sm font-medium">计划接入</p>
									<ul className="mt-3 space-y-2 text-sm text-[var(--yo-text-muted)]">
										<li>用户列表与分页</li>
										<li>外部身份绑定查看</li>
										<li>角色与权限绑定</li>
										<li>站点级别授权</li>
									</ul>
								</div>
								<div className="rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] p-4">
									<p className="text-sm font-medium">当前建议</p>
									<p className="mt-3 text-sm text-[var(--yo-text-muted)]">
										先把 RBAC 管理 API 补齐，再把用户管理真正接入这个页面。这样页面结构不会推倒重来。
									</p>
								</div>
							</div>
						</section>
					)}

					{activeTab === "permissions" && (
						<section className={panelClass}>
							<div className="mb-4 flex flex-col items-start gap-3 sm:flex-row sm:items-center sm:justify-between">
								<div>
									<h2 className="text-xl font-semibold">权限管理</h2>
									<p className="mt-1 text-sm text-[var(--yo-text-muted)]">
										这里承接 RBAC 的当前能力、角色定义和用户授权关系，先把只读信息真正接上。
									</p>
								</div>
								<Button size="sm" variant="secondary" disabled>
									编辑能力
								</Button>
							</div>

							{isLoadingPermissions ? (
								<p className="text-sm text-[var(--yo-text-muted)]">正在加载权限管理数据...</p>
							) : permissionsError ? (
								<div className="rounded-lg bg-[var(--yo-danger-bg)] px-4 py-3 text-sm text-[var(--yo-danger)]">
									{permissionsError}
								</div>
							) : (
								<>
									<div className="grid gap-4 lg:grid-cols-[minmax(0,1.3fr)_minmax(320px,0.9fr)]">
										<div className="rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] p-4">
											<div className="flex flex-col gap-1 sm:flex-row sm:items-end sm:justify-between">
												<div>
													<p className="text-sm font-medium">我的权限快照</p>
													<p className="mt-1 text-xs text-[var(--yo-text-soft)]">
														这里展示当前登录管理员自己拥有的权限，不是系统里的全部权限定义。
													</p>
												</div>
											</div>

											<div className="mt-4 grid gap-4 xl:grid-cols-2">
												<div className="rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface)] p-4">
													<p className="text-sm font-medium">我拥有的全局权限</p>
													{globalPermissions.length === 0 ? (
														<p className="mt-3 text-sm text-[var(--yo-text-muted)]">
															当前账号没有全局权限，后面后台面板应该更多依赖站点级授权来裁剪。
														</p>
													) : (
														<div className="mt-3 flex flex-wrap gap-2">
															{globalPermissions.map((permission) => (
																<span
																	key={permission}
																	className="rounded-full border border-[var(--yo-surface-strong)] px-2.5 py-1 text-xs text-[var(--yo-text-muted)]"
																>
																	{permission}
																</span>
															))}
														</div>
													)}
												</div>

												<div className="rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface)] p-4">
													<p className="text-sm font-medium">我拥有的站点权限</p>
													{sitePermissionEntries.length === 0 ? (
														<p className="mt-3 text-sm text-[var(--yo-text-muted)]">
															当前账号没有站点级角色绑定。
														</p>
													) : (
														<div className="mt-3 space-y-3">
															{sitePermissionEntries.map(([siteId, permissionNames]) => (
																<div
																	key={siteId}
																	className="rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] px-3 py-3"
																>
																	<p className="text-xs text-[var(--yo-text-soft)]">站点 #{siteId}</p>
																	<div className="mt-2 flex flex-wrap gap-2">
																		{permissionNames.map((permission) => (
																			<span
																				key={`${siteId}-${permission}`}
																				className="rounded-full border border-[var(--yo-surface-strong)] px-2 py-0.5 text-xs text-[var(--yo-text-muted)]"
																			>
																				{permission}
																			</span>
																		))}
																	</div>
																</div>
															))}
														</div>
													)}
												</div>
											</div>
										</div>

										<div className="rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] p-4">
											<p className="text-sm font-medium">RBAC 概览</p>
											<p className="mt-1 text-xs text-[var(--yo-text-soft)]">
												这一块看的是系统里当前有多少角色、权限和授权关系。
											</p>
											<div className="mt-4 grid gap-3 text-sm text-[var(--yo-text-muted)]">
												<div className="rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-surface)] px-3 py-3">
													<p className="text-xs text-[var(--yo-text-soft)]">系统角色</p>
													<p className="mt-1 text-2xl font-semibold text-[var(--yo-text)]">{roles.length}</p>
												</div>
												<div className="rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-surface)] px-3 py-3">
													<p className="text-xs text-[var(--yo-text-soft)]">权限能力</p>
													<p className="mt-1 text-2xl font-semibold text-[var(--yo-text)]">{permissions.length}</p>
												</div>
												<div className="rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-surface)] px-3 py-3">
													<p className="text-xs text-[var(--yo-text-soft)]">授权绑定</p>
													<p className="mt-1 text-2xl font-semibold text-[var(--yo-text)]">{roleBindings.length}</p>
												</div>
												<div className="rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-surface)] px-3 py-3">
													<p className="text-xs text-[var(--yo-text-soft)]">站点授权覆盖</p>
													<p className="mt-1 text-2xl font-semibold text-[var(--yo-text)]">{sitePermissionEntries.length}</p>
												</div>
											</div>
										</div>
									</div>

									<div className="mt-4 grid gap-4 xl:grid-cols-[minmax(0,1.2fr)_minmax(0,1fr)]">
										<div className="rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] p-4">
											<div className="flex items-center justify-between gap-3">
												<div>
													<p className="text-sm font-medium">系统角色</p>
													<p className="mt-1 text-xs text-[var(--yo-text-soft)]">
														展示角色与其当前绑定的权限能力。
													</p>
												</div>
											</div>
											<div className="mt-3 space-y-3">
												{roles.length === 0 ? (
													<p className="text-sm text-[var(--yo-text-muted)]">暂无角色数据。</p>
												) : (
													roles.map((role) => (
														<div
															key={role.id}
															className="rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-surface)] p-3"
														>
															<div className="flex flex-wrap items-center gap-2">
																<p className="font-medium">{role.name}</p>
																<span className="rounded-full bg-[var(--yo-surface-soft)] px-2 py-0.5 text-[11px] text-[var(--yo-text-muted)]">
																	ID {role.id}
																</span>
															</div>
															{role.description && (
																<p className="mt-2 text-sm text-[var(--yo-text-muted)]">
																	{role.description}
																</p>
															)}
															<div className="mt-3 flex flex-wrap gap-2">
																{role.permission_names.length === 0 ? (
																	<span className="text-xs text-[var(--yo-text-muted)]">
																		当前未绑定权限
																	</span>
																) : (
																	role.permission_names.map((permission) => (
																		<span
																			key={`${role.id}-${permission}`}
																			className="rounded-full border border-[var(--yo-surface-strong)] px-2 py-0.5 text-xs text-[var(--yo-text-muted)]"
																		>
																			{permission}
																		</span>
																	))
																)}
															</div>
														</div>
													))
												)}
											</div>
										</div>

										<div className="space-y-4">
											<div className="rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] p-4">
												<p className="text-sm font-medium">权限能力</p>
												<div className="mt-3 space-y-2">
													{permissions.length === 0 ? (
														<p className="text-sm text-[var(--yo-text-muted)]">暂无权限数据。</p>
													) : (
														permissions.map((permission) => (
															<div
																key={permission.id}
																className="rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-surface)] px-3 py-2"
															>
																<div className="flex flex-wrap items-center gap-2">
																	<code className="text-sm">{permission.name}</code>
																	<span className="text-[11px] text-[var(--yo-text-soft)]">
																		ID {permission.id}
																	</span>
																</div>
																{permission.description && (
																	<p className="mt-1 text-sm text-[var(--yo-text-muted)]">
																		{permission.description}
																	</p>
																)}
															</div>
														))
													)}
												</div>
											</div>

											<div className="rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] p-4">
												<p className="text-sm font-medium">用户授权绑定</p>
												<div className="mt-3 space-y-2">
													{roleBindings.length === 0 ? (
														<p className="text-sm text-[var(--yo-text-muted)]">暂无授权绑定。</p>
													) : (
														roleBindings.map((binding) => (
															<div
																key={binding.id}
																className="rounded-md border border-[var(--yo-surface-strong)] bg-[var(--yo-surface)] px-3 py-3"
															>
																<div className="flex flex-wrap items-center gap-2">
																	<p className="font-medium">
																		{binding.role_name ?? `角色 #${binding.role_id}`}
																	</p>
																	<span className="rounded-full bg-[var(--yo-surface-soft)] px-2 py-0.5 text-[11px] text-[var(--yo-text-muted)]">
																		用户 #{binding.user_id}
																	</span>
																	<span className="rounded-full bg-[var(--yo-surface-soft)] px-2 py-0.5 text-[11px] text-[var(--yo-text-muted)]">
																		{binding.scope_type}
																		{binding.scope_id ? `:${binding.scope_id}` : ""}
																	</span>
																</div>
																<div className="mt-2 flex flex-wrap gap-x-4 gap-y-1 text-xs text-[var(--yo-text-muted)]">
																	<span>创建：{formatLocalDateTime(binding.created_at)}</span>
																	<span>更新：{formatLocalDateTime(binding.updated_at)}</span>
																</div>
															</div>
														))
													)}
												</div>
											</div>
										</div>
									</div>
								</>
							)}
						</section>
					)}

					{activeTab === "oauthProviders" && (
						<section className={panelClass}>
							<div className="mb-4 flex flex-col items-start gap-3 sm:flex-row sm:items-center sm:justify-between">
								<div>
									<h2 className="text-xl font-semibold">OAuth 提供者</h2>
									<p className="mt-1 text-sm text-[var(--yo-text-muted)]">
										这里放社交登录与标准 OAuth 配置。后面可以继续接 GitHub、Google、QQ 等提供者的创建与启用流程。
									</p>
								</div>
								<Button size="sm">新增提供者</Button>
							</div>

							<div className="grid gap-4 md:grid-cols-2">
								<div className="rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] p-4">
									<p className="text-sm font-medium">准备接入</p>
									<ul className="mt-3 space-y-2 text-sm text-[var(--yo-text-muted)]">
										<li>GitHub / Google / QQ OAuth</li>
										<li>客户端 ID / Secret 管理</li>
										<li>回调地址校验</li>
										<li>启用 / 禁用状态切换</li>
									</ul>
								</div>
								<div className="rounded-lg border border-dashed border-[var(--yo-surface-strong)] px-4 py-8 text-center text-sm text-[var(--yo-text-muted)]">
									当前后端只有创建接口骨架，这里先把管理位置和后续信息结构预留出来。
								</div>
							</div>
						</section>
					)}

					{activeTab === "moderationProviders" && (
						<section className={panelClass}>
							<div className="mb-4 flex flex-col items-start gap-3 sm:flex-row sm:items-center sm:justify-between">
								<div>
									<h2 className="text-xl font-semibold">审核提供者</h2>
									<p className="mt-1 text-sm text-[var(--yo-text-muted)]">
										这里管理评论审核来源，比如 LLM、Akismet，以及后续可能接入的自定义审核器。
									</p>
								</div>
								<Button size="sm">新增审核器</Button>
							</div>

							<div className="grid gap-4 lg:grid-cols-2">
								<div className="rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] p-4">
									<p className="text-sm font-medium">当前规划</p>
									<ul className="mt-3 space-y-2 text-sm text-[var(--yo-text-muted)]">
										<li>LLM 审核配置</li>
										<li>Akismet 审核配置</li>
										<li>按站点启用 / 禁用</li>
										<li>Prompt / 模型 / API Base 管理</li>
									</ul>
								</div>
								<div className="rounded-lg border border-dashed border-[var(--yo-surface-strong)] px-4 py-8 text-center text-sm text-[var(--yo-text-muted)]">
									后端管理 API 已经有基础骨架，这里后面可以优先接成第一批真正可用的后台配置页。
								</div>
							</div>
						</section>
					)}

					{activeTab === "externalProviders" && (
						<section className={panelClass}>
							<h2 className="text-xl font-semibold">外部身份提供者</h2>
							<p className="mt-2 text-sm text-[var(--yo-text-muted)]">
								这个区域用来承接宿主系统登录态、外部 SSO、以及统一身份映射配置。它和 OAuth 提供者不同，更偏“已有身份接入”而不是社交登录。
							</p>

							<div className="mt-5 grid gap-4 md:grid-cols-2">
								<div className="rounded-lg border border-[var(--yo-surface-strong)] bg-[var(--yo-surface-soft)] p-4">
									<p className="text-sm font-medium">后续会放</p>
									<ul className="mt-3 space-y-2 text-sm text-[var(--yo-text-muted)]">
										<li>External token exchange</li>
										<li>外部 provider 标识与元数据</li>
										<li>用户身份映射检查</li>
										<li>SSO / OIDC 接入入口</li>
									</ul>
								</div>
								<div className="rounded-lg border border-dashed border-[var(--yo-surface-strong)] px-4 py-8 text-center text-sm text-[var(--yo-text-muted)]">
									这块目前以后端统一身份映射模型为基础，UI 先留入口，避免后面再重做信息架构。
								</div>
							</div>
						</section>
					)}
				</main>
			</div>
		</div>
	);
};

export default App;
