import { useState } from "preact/hooks";
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
import { ADMIN_TABS, type AdminTab } from "@/admin/types";

const panelClass =
  "rounded-xl border border-(--yo-surface-strong) bg-(--yo-surface) p-5 shadow-sm";

const App = () => {
  const [activeTab, setActiveTab] = useState<AdminTab>("sites");
  const { profile, isLoadingProfile, profileError } = useAdminProfile();
  const {
    capabilities,
    roles,
    permissions,
    roleBindings,
    isLoadingPermissions,
    permissionsError,
  } = useAdminPermissions(activeTab);
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

          {activeTab === "users" && <UsersPanel panelClass={panelClass} />}

          {activeTab === "permissions" && (
            <PermissionsPanel
              panelClass={panelClass}
              isLoadingPermissions={isLoadingPermissions}
              permissionsError={permissionsError}
              capabilities={capabilities}
              roles={roles}
              permissions={permissions}
              roleBindings={roleBindings}
            />
          )}

          {activeTab === "oauthProviders" && (
            <OAuthProvidersPanel panelClass={panelClass} />
          )}

          {activeTab === "moderationProviders" && (
            <ModerationProvidersPanel panelClass={panelClass} />
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
