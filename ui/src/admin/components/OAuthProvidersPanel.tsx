import type { OAuthProviderFormState } from "@/admin/types";
import type { OauthProvider } from "@/shared/api/types";
import { Button } from "@/shared/components/Button";
import { useI18n } from "@/shared/i18n";

interface Props {
  panelClass: string;
  isCreatingOAuthProvider: boolean;
  createOAuthProviderForm: OAuthProviderFormState;
  createOAuthProviderError: string;
  isSubmittingOAuthProvider: boolean;
  isLoadingOauthProviders: boolean;
  oauthProvidersError: string;
  oauthProviders: OauthProvider[];
  updatingOauthProviderId: number | null;
  onToggleCreateOAuthProvider: () => void;
  onChangeCreateOAuthProviderForm: (
    patch: Partial<OAuthProviderFormState>,
  ) => void;
  onCancelCreateOAuthProvider: () => void;
  onCreateOAuthProvider: () => void;
  onToggleOauthProviderEnabled: (provider: OauthProvider) => void;
}

export const OAuthProvidersPanel = ({
  panelClass,
  isCreatingOAuthProvider,
  createOAuthProviderForm,
  createOAuthProviderError,
  isSubmittingOAuthProvider,
  isLoadingOauthProviders,
  oauthProvidersError,
  oauthProviders,
  updatingOauthProviderId,
  onToggleCreateOAuthProvider,
  onChangeCreateOAuthProviderForm,
  onCancelCreateOAuthProvider,
  onCreateOAuthProvider,
  onToggleOauthProviderEnabled,
}: Props) => {
  const { t } = useI18n();
  return (
    <section className={panelClass}>
      <div className="mb-4 flex flex-col items-start gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-xl font-semibold">{t("admin.oauth.title")}</h2>
          <p className="mt-1 text-sm text-(--yo-text-muted)">
            {t("admin.oauth.desc")}
          </p>
        </div>
        <Button size="sm" onClick={onToggleCreateOAuthProvider}>
          {isCreatingOAuthProvider
            ? t("common.collapseForm")
            : t("admin.oauth.add")}
        </Button>
      </div>

      {isCreatingOAuthProvider && (
        <div className="mb-4 rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
          <h3 className="text-base font-medium">
            {t("admin.oauth.createTitle")}
          </h3>
          <div className="mt-4 grid gap-4 md:grid-cols-2">
            <label className="block">
              <span className="text-xs text-(--yo-text-soft)">
                {t("admin.oauth.providerCode")}
              </span>
              <select
                value={createOAuthProviderForm.providerCode}
                onChange={(event) =>
                  onChangeCreateOAuthProviderForm({
                    providerCode: event.currentTarget.value,
                  })
                }
                className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
              >
                <option value="github">github</option>
                <option value="qq">qq</option>
              </select>
            </label>
            <label className="block">
              <span className="text-xs text-(--yo-text-soft)">
                {t("admin.oauth.clientId")}
              </span>
              <input
                type="text"
                value={createOAuthProviderForm.clientId}
                onInput={(event) =>
                  onChangeCreateOAuthProviderForm({
                    clientId: event.currentTarget.value,
                  })
                }
                className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
              />
            </label>
            <label className="block">
              <span className="text-xs text-(--yo-text-soft)">
                {t("admin.oauth.clientSecret")}
              </span>
              <input
                type="password"
                value={createOAuthProviderForm.clientSecret}
                onInput={(event) =>
                  onChangeCreateOAuthProviderForm({
                    clientSecret: event.currentTarget.value,
                  })
                }
                className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
              />
            </label>
            <label className="block">
              <span className="text-xs text-(--yo-text-soft)">
                {t("admin.oauth.redirectUri")}
              </span>
              <input
                type="url"
                value={createOAuthProviderForm.redirectUri}
                onInput={(event) =>
                  onChangeCreateOAuthProviderForm({
                    redirectUri: event.currentTarget.value,
                  })
                }
                className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
              />
            </label>
          </div>
          <label className="mt-4 inline-flex items-center gap-2 text-sm text-(--yo-text-muted)">
            <input
              type="checkbox"
              checked={createOAuthProviderForm.enabled}
              onChange={(event) =>
                onChangeCreateOAuthProviderForm({
                  enabled: event.currentTarget.checked,
                })
              }
              className="h-4 w-4 rounded border border-(--yo-surface-strong)"
            />
            {t("admin.oauth.enableNow")}
          </label>
          {createOAuthProviderError && (
            <p className="mt-3 text-sm text-(--yo-danger)">
              {createOAuthProviderError}
            </p>
          )}
          <div className="mt-4 flex flex-wrap gap-2">
            <Button
              size="sm"
              loading={isSubmittingOAuthProvider}
              onClick={onCreateOAuthProvider}
            >
              {t("common.create")}
            </Button>
            <Button
              size="sm"
              variant="ghost"
              disabled={isSubmittingOAuthProvider}
              onClick={onCancelCreateOAuthProvider}
            >
              {t("common.cancel")}
            </Button>
          </div>
        </div>
      )}

      {isLoadingOauthProviders ? (
        <p className="text-sm text-(--yo-text-muted)">
          {t("admin.oauth.loading")}
        </p>
      ) : oauthProvidersError ? (
        <div className="rounded-lg bg-(--yo-danger-bg) px-4 py-3 text-sm text-(--yo-danger)">
          {oauthProvidersError}
        </div>
      ) : oauthProviders.length === 0 ? (
        <div className="rounded-lg border border-dashed border-(--yo-surface-strong) px-4 py-8 text-center text-sm text-(--yo-text-muted)">
          {t("admin.oauth.empty")}
        </div>
      ) : (
        <div className="grid gap-4 xl:grid-cols-2">
          {oauthProviders.map((provider) => (
            <article
              key={provider.id}
              className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4"
            >
              <div className="flex items-start justify-between gap-3">
                <div>
                  <h3 className="font-medium">{provider.provider_code}</h3>
                  <p className="mt-1 break-all text-sm text-(--yo-text-muted)">
                    {provider.redirect_uri}
                  </p>
                </div>
                <Button
                  size="sm"
                  variant="secondary"
                  loading={updatingOauthProviderId === provider.id}
                  onClick={() => onToggleOauthProviderEnabled(provider)}
                >
                  {provider.enabled ? t("common.disable") : t("common.enable")}
                </Button>
              </div>
              <div className="mt-4 grid grid-cols-2 gap-3 text-sm">
                <div>
                  <p className="text-(--yo-text-soft)">Client ID</p>
                  <p className="mt-1 break-all font-medium">
                    {provider.client_id}
                  </p>
                </div>
                <div>
                  <p className="text-(--yo-text-soft)">
                    {t("admin.oauth.scope")}
                  </p>
                  <p className="mt-1 font-medium">
                    {provider.site_id == null
                      ? t("common.global")
                      : t("admin.oauth.siteN", { id: provider.site_id })}
                  </p>
                </div>
              </div>
            </article>
          ))}
        </div>
      )}
    </section>
  );
};
