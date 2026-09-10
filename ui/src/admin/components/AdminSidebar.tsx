import type { AdminTab, AdminTabItem } from "@/admin/types";
import type { UserProfile } from "@/shared/api/types";
import { Button } from "@/shared/components/Button";
import { useI18n } from "@/shared/i18n";

interface Props {
  panelClass: string;
  tabs: AdminTabItem[];
  activeTab: AdminTab;
  profile: UserProfile | null;
  isLoadingProfile: boolean;
  profileError: string;
  onChangeTab: (tab: AdminTab) => void;
  onLogout: () => void;
}

export const AdminSidebar = ({
  panelClass,
  tabs,
  activeTab,
  profile,
  isLoadingProfile,
  profileError,
  onChangeTab,
  onLogout,
}: Props) => {
  const { t } = useI18n();
  return (
    <aside className="w-full shrink-0 lg:w-72">
      <div className={`${panelClass} lg:sticky lg:top-6`}>
        <div className="mb-5 rounded-lg bg-(--yo-surface-soft) p-4">
          <p className="text-xs text-(--yo-text-soft)">
            {t("admin.currentIdentity")}
          </p>
          {isLoadingProfile ? (
            <p className="mt-2 text-sm text-(--yo-text-muted)">
              {t("admin.loadingProfile")}
            </p>
          ) : profile ? (
            <div className="mt-2 space-y-3">
              <div className="space-y-1">
                <p className="font-medium">{profile.nickname}</p>
                <p className="text-sm text-(--yo-text-muted)">
                  {profile.email}
                </p>
              </div>
              <Button
                type="button"
                variant="ghost"
                size="sm"
                onClick={onLogout}
              >
                {t("admin.logout")}
              </Button>
            </div>
          ) : (
            <p className="mt-2 text-sm text-(--yo-danger)">
              {profileError || t("admin.profileMissing")}
            </p>
          )}
        </div>

        <nav className="flex gap-2 overflow-x-auto pb-1 lg:block lg:space-y-2 lg:overflow-visible lg:pb-0">
          {tabs.map((tab) => {
            const isActive = activeTab === tab.key;
            return (
              <Button
                key={tab.key}
                type="button"
                variant={isActive ? "primary" : "secondary"}
                onClick={() => onChangeTab(tab.key)}
                className={`min-w-52 justify-start rounded-lg px-4 py-3 text-left lg:w-full lg:min-w-0 ${
                  !isActive ? "hover:bg-(--yo-surface-soft)" : ""
                }`}
                fullWidth
              >
                <div className="block">
                  <p className="font-medium">{t(tab.labelKey)}</p>
                  <p
                    className={`mt-1 text-xs ${
                      isActive
                        ? "text-[color-mix(in_srgb,var(--yo-primary-contrast)_72%,transparent)]"
                        : "text-(--yo-text-muted)"
                    }`}
                  >
                    {t(tab.descriptionKey)}
                  </p>
                </div>
              </Button>
            );
          })}
        </nav>
      </div>
    </aside>
  );
};
