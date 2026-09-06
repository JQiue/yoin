import type { OAuthProviderFormState } from "@/admin/types";
import type { OauthProvider } from "@/shared/api/types";
import { Button } from "@/shared/components/Button";

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
  return (
    <section className={panelClass}>
      <div className="mb-4 flex flex-col items-start gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-xl font-semibold">OAuth 提供者</h2>
          <p className="mt-1 text-sm text-(--yo-text-muted)">
            目前登录流程只支持全局启用的 GitHub / QQ。Secret
            不会回显，创建后只能重新填写覆盖。
          </p>
        </div>
        <Button size="sm" onClick={onToggleCreateOAuthProvider}>
          {isCreatingOAuthProvider ? "收起表单" : "新增提供者"}
        </Button>
      </div>

      {isCreatingOAuthProvider && (
        <div className="mb-4 rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
          <h3 className="text-base font-medium">创建新 OAuth 提供者</h3>
          <div className="mt-4 grid gap-4 md:grid-cols-2">
            <label className="block">
              <span className="text-xs text-(--yo-text-soft)">提供者代码</span>
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
              <span className="text-xs text-(--yo-text-soft)">Client ID</span>
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
                Client Secret
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
              <span className="text-xs text-(--yo-text-soft)">回调地址</span>
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
            立即启用
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
              创建
            </Button>
            <Button
              size="sm"
              variant="ghost"
              disabled={isSubmittingOAuthProvider}
              onClick={onCancelCreateOAuthProvider}
            >
              取消
            </Button>
          </div>
        </div>
      )}

      {isLoadingOauthProviders ? (
        <p className="text-sm text-(--yo-text-muted)">
          正在加载 OAuth 提供者...
        </p>
      ) : oauthProvidersError ? (
        <div className="rounded-lg bg-(--yo-danger-bg) px-4 py-3 text-sm text-(--yo-danger)">
          {oauthProvidersError}
        </div>
      ) : oauthProviders.length === 0 ? (
        <div className="rounded-lg border border-dashed border-(--yo-surface-strong) px-4 py-8 text-center text-sm text-(--yo-text-muted)">
          还没有 OAuth 提供者。登录开始接口会读取这里启用的全局配置。
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
                  {provider.enabled ? "停用" : "启用"}
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
                  <p className="text-(--yo-text-soft)">范围</p>
                  <p className="mt-1 font-medium">
                    {provider.site_id == null
                      ? "全局"
                      : `站点 #${provider.site_id}`}
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
