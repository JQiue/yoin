import { useState } from "preact/hooks";
import { AdminLogin } from "@/admin/components/AdminLogin";
import { AdminSidebar } from "@/admin/components/AdminSidebar";
import { CommentsPanel } from "@/admin/components/CommentsPanel";
import { ExternalProvidersPanel } from "@/admin/components/ExternalProvidersPanel";
import { ModerationProvidersPanel } from "@/admin/components/ModerationProvidersPanel";
import { OAuthProvidersPanel } from "@/admin/components/OAuthProvidersPanel";
import { PermissionsPanel } from "@/admin/components/PermissionsPanel";
import { SitesPanel } from "@/admin/components/SitesPanel";
import { UsersPanel } from "@/admin/components/UsersPanel";
import {
  COMMENT_STATUS_OPTIONS,
  COMMENT_TABS,
  useAdminComments,
} from "@/admin/hooks/useAdminComments";
import { useAdminPermissions } from "@/admin/hooks/useAdminPermissions";
import { useAdminProfile } from "@/admin/hooks/useAdminProfile";
import { useAdminSites } from "@/admin/hooks/useAdminSites";
import { useAdminUsers } from "@/admin/hooks/useAdminUsers";
import { ADMIN_TABS, type AdminTab } from "@/admin/types";
import { useAdminProviders } from "./hooks/useAdminProviders";

const panelClass =
  "rounded-xl border border-(--yo-surface-strong) bg-(--yo-surface) p-5 shadow-sm";

const App = () => {
  const { profile, isLoadingProfile, profileError, refreshProfile, logout } =
    useAdminProfile();

  if (isLoadingProfile) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-(--yo-bg) text-(--yo-text-muted)">
        正在加载管理员信息...
      </div>
    );
  }

  if (!profile) {
    return <AdminLogin error={profileError} onSuccess={refreshProfile} />;
  }

  return (
    <AdminDashboard
      profile={profile}
      isLoadingProfile={isLoadingProfile}
      profileError={profileError}
      onLogout={logout}
    />
  );
};

const AdminDashboard = ({
  profile,
  isLoadingProfile,
  profileError,
  onLogout,
}: {
  profile: NonNullable<ReturnType<typeof useAdminProfile>["profile"]>;
  isLoadingProfile: boolean;
  profileError: string;
  onLogout: () => void;
}) => {
  const [activeTab, setActiveTab] = useState<AdminTab>("sites");
  const {
    capabilities,
    roles,
    permissions,
    roleBindings,
    isLoadingPermissions,
    permissionsError,
    savingRoleId,
    deletingBindingId,
    isCreatingBinding,
    bindingForm,
    bindingError,
    setBindingForm,
    saveRolePermissions,
    createBinding,
    deleteBinding,
  } = useAdminPermissions(activeTab);
  const { users, isLoadingUsers, usersError } = useAdminUsers(activeTab);
  const {
    sites,
    isLoadingSites,
    sitesError,
    editingSiteId,
    siteForm,
    siteFormError,
    isSavingSite,
    isCreatingSite,
    createSiteForm,
    createSiteError,
    isSubmittingCreateSite,
    setIsCreatingSite,
    setCreateSiteError,
    setCreateSiteForm,
    setSiteForm,
    beginEditSite,
    cancelEditSite,
    saveSite,
    createSite,
  } = useAdminSites(activeTab);
  const {
    oauthProviders,
    isLoadingOauthProviders,
    oauthProvidersError,
    isCreatingOAuthProvider,
    createOAuthProviderForm,
    createOAuthProviderError,
    isSubmittingOAuthProvider,
    updatingOauthProviderId,
    setIsCreatingOAuthProvider,
    setCreateOAuthProviderForm,
    setCreateOAuthProviderError,
    createOAuthProvider,
    toggleOauthProviderEnabled,
    moderationProviders,
    isLoadingModerationProviders,
    moderationProvidersError,
    isCreatingModerationProvider,
    createModerationProviderForm,
    createModerationProviderError,
    isSubmittingModerationProvider,
    editingModerationProviderId,
    moderationProviderForm,
    moderationProviderFormError,
    isSavingModerationProvider,
    setIsCreatingModerationProvider,
    setCreateModerationProviderForm,
    setCreateModerationProviderError,
    setModerationProviderForm,
    createModerationProvider,
    beginEditModerationProvider,
    cancelEditModerationProvider,
    saveModerationProvider,
    toggleModerationProviderEnabled,
  } = useAdminProviders(activeTab, sites);
  const {
    activeCommentTab,
    commentsPage,
    selectedCommentSiteId,
    commentPagePath,
    commentPageOffset,
    isLoadingComments,
    commentsError,
    updatingCommentId,
    comments,
    activeCommentTabLabel,
    setSelectedCommentSiteId,
    setCommentPagePath,
    setActiveCommentTab,
    setCommentPageOffset,
    refreshComments,
    handleUpdateCommentStatus,
  } = useAdminComments(activeTab, sites);

  return (
    <div className="min-h-screen bg-(--yo-bg) text-(--yo-text)">
      <div className="mx-auto flex min-h-screen max-w-full flex-col gap-6 px-4 py-6 lg:flex-row lg:px-6">
        <AdminSidebar
          panelClass={panelClass}
          tabs={ADMIN_TABS}
          activeTab={activeTab}
          profile={profile}
          isLoadingProfile={isLoadingProfile}
          profileError={profileError}
          onChangeTab={setActiveTab}
          onLogout={onLogout}
        />

        <main className="min-w-0 flex-1">
          {activeTab === "sites" && (
            <SitesPanel
              panelClass={panelClass}
              isCreatingSite={isCreatingSite}
              createSiteForm={createSiteForm}
              createSiteError={createSiteError}
              isSubmittingCreateSite={isSubmittingCreateSite}
              isLoadingSites={isLoadingSites}
              sitesError={sitesError}
              sites={sites}
              editingSiteId={editingSiteId}
              siteForm={siteForm}
              siteFormError={siteFormError}
              isSavingSite={isSavingSite}
              onToggleCreateSite={() => {
                setIsCreatingSite((current) => !current);
                setCreateSiteError("");
              }}
              onChangeCreateSiteForm={(patch) =>
                setCreateSiteForm((current) => ({
                  ...current,
                  ...patch,
                }))
              }
              onCancelCreateSite={() => {
                setIsCreatingSite(false);
                setCreateSiteError("");
              }}
              onCreateSite={createSite}
              onBeginEditSite={beginEditSite}
              onChangeSiteForm={(patch) =>
                setSiteForm((current) =>
                  current
                    ? {
                      ...current,
                      ...patch,
                    }
                    : current,
                )
              }
              onCancelEditSite={cancelEditSite}
              onSaveSite={saveSite}
            />
          )}

          {activeTab === "comments" && (
            <CommentsPanel
              panelClass={panelClass}
              sites={sites}
              selectedCommentSiteId={selectedCommentSiteId}
              commentPagePath={commentPagePath}
              activeCommentTab={activeCommentTab}
              activeCommentTabLabel={activeCommentTabLabel}
              commentPageOffset={commentPageOffset}
              commentsPage={commentsPage}
              comments={comments}
              isLoadingComments={isLoadingComments}
              commentsError={commentsError}
              updatingCommentId={updatingCommentId}
              commentTabs={COMMENT_TABS}
              commentStatusOptions={COMMENT_STATUS_OPTIONS}
              onRefresh={refreshComments}
              onSelectSite={setSelectedCommentSiteId}
              onChangePagePath={setCommentPagePath}
              onChangeTab={setActiveCommentTab}
              onUpdateCommentStatus={handleUpdateCommentStatus}
              onPrevPage={() =>
                setCommentPageOffset((current) => Math.max(1, current - 1))
              }
              onNextPage={() =>
                setCommentPageOffset((current) =>
                  commentsPage
                    ? Math.min(commentsPage.total_pages, current + 1)
                    : current,
                )
              }
            />
          )}

          {activeTab === "users" && (
            <UsersPanel
              panelClass={panelClass}
              users={users}
              isLoadingUsers={isLoadingUsers}
              usersError={usersError}
            />
          )}

          {activeTab === "permissions" && (
            <PermissionsPanel
              panelClass={panelClass}
              isLoadingPermissions={isLoadingPermissions}
              permissionsError={permissionsError}
              capabilities={capabilities}
              roles={roles}
              permissions={permissions}
              roleBindings={roleBindings}
              users={users}
              sites={sites}
              savingRoleId={savingRoleId}
              deletingBindingId={deletingBindingId}
              isCreatingBinding={isCreatingBinding}
              bindingForm={bindingForm}
              bindingError={bindingError}
              onToggleRolePermission={(role, permissionName) => {
                const next = role.permission_names.includes(permissionName)
                  ? role.permission_names.filter(
                    (name) => name !== permissionName,
                  )
                  : [...role.permission_names, permissionName];
                void saveRolePermissions(role.id, next);
              }}
              onChangeBindingForm={(patch) =>
                setBindingForm((current) => ({ ...current, ...patch }))
              }
              onCreateBinding={createBinding}
              onDeleteBinding={deleteBinding}
            />
          )}

          {activeTab === "oauthProviders" && (
            <OAuthProvidersPanel
              panelClass={panelClass}
              isCreatingOAuthProvider={isCreatingOAuthProvider}
              createOAuthProviderForm={createOAuthProviderForm}
              createOAuthProviderError={createOAuthProviderError}
              isSubmittingOAuthProvider={isSubmittingOAuthProvider}
              isLoadingOauthProviders={isLoadingOauthProviders}
              oauthProvidersError={oauthProvidersError}
              oauthProviders={oauthProviders}
              updatingOauthProviderId={updatingOauthProviderId}
              onToggleCreateOAuthProvider={() => {
                setIsCreatingOAuthProvider((current) => !current);
                setCreateOAuthProviderError("");
              }}
              onChangeCreateOAuthProviderForm={(patch) =>
                setCreateOAuthProviderForm((current) => ({
                  ...current,
                  ...patch,
                }))
              }
              onCancelCreateOAuthProvider={() => {
                setIsCreatingOAuthProvider(false);
                setCreateOAuthProviderError("");
              }}
              onCreateOAuthProvider={createOAuthProvider}
              onToggleOauthProviderEnabled={toggleOauthProviderEnabled}
            />
          )}

          {activeTab === "moderationProviders" && (
            <ModerationProvidersPanel
              panelClass={panelClass}
              sites={sites}
              isCreatingModerationProvider={isCreatingModerationProvider}
              createModerationProviderForm={createModerationProviderForm}
              createModerationProviderError={createModerationProviderError}
              isSubmittingModerationProvider={isSubmittingModerationProvider}
              isLoadingModerationProviders={isLoadingModerationProviders}
              moderationProvidersError={moderationProvidersError}
              moderationProviders={moderationProviders}
              editingModerationProviderId={editingModerationProviderId}
              moderationProviderForm={moderationProviderForm}
              moderationProviderFormError={moderationProviderFormError}
              isSavingModerationProvider={isSavingModerationProvider}
              onToggleCreateModerationProvider={() => {
                setIsCreatingModerationProvider((current) => !current);
                setCreateModerationProviderError("");
              }}
              onChangeCreateModerationProviderForm={(patch) =>
                setCreateModerationProviderForm((current) => ({
                  ...current,
                  ...patch,
                }))
              }
              onCancelCreateModerationProvider={() => {
                setIsCreatingModerationProvider(false);
                setCreateModerationProviderError("");
              }}
              onCreateModerationProvider={createModerationProvider}
              onBeginEditModerationProvider={beginEditModerationProvider}
              onChangeModerationProviderForm={(patch) =>
                setModerationProviderForm((current) =>
                  current ? { ...current, ...patch } : current,
                )
              }
              onCancelEditModerationProvider={cancelEditModerationProvider}
              onSaveModerationProvider={saveModerationProvider}
              onToggleModerationProviderEnabled={
                toggleModerationProviderEnabled
              }
            />
          )}

          {activeTab === "externalProviders" && (
            <ExternalProvidersPanel panelClass={panelClass} />
          )}
        </main>
      </div>
    </div>
  );
};

export default App;
