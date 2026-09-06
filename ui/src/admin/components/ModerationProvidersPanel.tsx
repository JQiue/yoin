import type { ModerationProviderFormState } from "@/admin/types";
import type { ModerationProvider, Site } from "@/shared/api/types";
import { Button } from "@/shared/components/Button";

interface Props {
  panelClass: string;
  sites: Site[];
  isCreatingModerationProvider: boolean;
  createModerationProviderForm: ModerationProviderFormState;
  createModerationProviderError: string;
  isSubmittingModerationProvider: boolean;
  isLoadingModerationProviders: boolean;
  moderationProvidersError: string;
  moderationProviders: ModerationProvider[];
  editingModerationProviderId: number | null;
  moderationProviderForm: ModerationProviderFormState | null;
  moderationProviderFormError: string;
  isSavingModerationProvider: boolean;
  onToggleCreateModerationProvider: () => void;
  onChangeCreateModerationProviderForm: (
    patch: Partial<ModerationProviderFormState>,
  ) => void;
  onCancelCreateModerationProvider: () => void;
  onCreateModerationProvider: () => void;
  onBeginEditModerationProvider: (provider: ModerationProvider) => void;
  onChangeModerationProviderForm: (
    patch: Partial<ModerationProviderFormState>,
  ) => void;
  onCancelEditModerationProvider: () => void;
  onSaveModerationProvider: () => void;
  onToggleModerationProviderEnabled: (provider: ModerationProvider) => void;
}

function siteName(sites: Site[], siteId: number) {
  return sites.find((site) => site.id === siteId)?.name ?? `站点 #${siteId}`;
}

function configSummary(provider: ModerationProvider) {
  if (provider.provider_kind === "llm" && "model" in provider.config) {
    return `${provider.config.model} · ${provider.config.api_base}`;
  }
  if (provider.provider_kind === "akismet" && "blog_url" in provider.config) {
    return provider.config.blog_url;
  }
  return "配置已保存";
}

export const ModerationProvidersPanel = ({
  panelClass,
  sites,
  isCreatingModerationProvider,
  createModerationProviderForm,
  createModerationProviderError,
  isSubmittingModerationProvider,
  isLoadingModerationProviders,
  moderationProvidersError,
  moderationProviders,
  editingModerationProviderId,
  moderationProviderForm,
  moderationProviderFormError,
  isSavingModerationProvider,
  onToggleCreateModerationProvider,
  onChangeCreateModerationProviderForm,
  onCancelCreateModerationProvider,
  onCreateModerationProvider,
  onBeginEditModerationProvider,
  onChangeModerationProviderForm,
  onCancelEditModerationProvider,
  onSaveModerationProvider,
  onToggleModerationProviderEnabled,
}: Props) => {
  return (
    <section className={panelClass}>
      <div className="mb-4 flex flex-col items-start gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-xl font-semibold">审核提供者</h2>
          <p className="mt-1 text-sm text-(--yo-text-muted)">
            按站点配置 LLM 或 Akismet。API Key 会回显，请只在可信环境使用。
          </p>
        </div>
        <Button
          size="sm"
          disabled={sites.length === 0}
          onClick={onToggleCreateModerationProvider}
        >
          {isCreatingModerationProvider ? "收起表单" : "新增审核器"}
        </Button>
      </div>

      {sites.length === 0 && (
        <div className="mb-4 rounded-lg border border-dashed border-(--yo-surface-strong) px-4 py-8 text-center text-sm text-(--yo-text-muted)">
          请先创建站点，再配置审核提供者。
        </div>
      )}

      {isCreatingModerationProvider && sites.length > 0 && (
        <div className="mb-4 rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
          <h3 className="text-base font-medium">创建审核提供者</h3>
          <ModerationProviderFields
            form={createModerationProviderForm}
            sites={sites}
            allowKindChange
            onChange={onChangeCreateModerationProviderForm}
          />
          {createModerationProviderError && (
            <p className="mt-3 text-sm text-(--yo-danger)">
              {createModerationProviderError}
            </p>
          )}
          <div className="mt-4 flex flex-wrap gap-2">
            <Button
              size="sm"
              loading={isSubmittingModerationProvider}
              onClick={onCreateModerationProvider}
            >
              创建
            </Button>
            <Button
              size="sm"
              variant="ghost"
              disabled={isSubmittingModerationProvider}
              onClick={onCancelCreateModerationProvider}
            >
              取消
            </Button>
          </div>
        </div>
      )}

      {isLoadingModerationProviders ? (
        <p className="text-sm text-(--yo-text-muted)">正在加载审核提供者...</p>
      ) : moderationProvidersError ? (
        <div className="rounded-lg bg-(--yo-danger-bg) px-4 py-3 text-sm text-(--yo-danger)">
          {moderationProvidersError}
        </div>
      ) : moderationProviders.length === 0 ? (
        <div className="rounded-lg border border-dashed border-(--yo-surface-strong) px-4 py-8 text-center text-sm text-(--yo-text-muted)">
          还没有审核提供者。
        </div>
      ) : (
        <div className="grid gap-4 xl:grid-cols-2">
          {moderationProviders.map((provider) => (
            <article
              key={provider.id}
              className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4"
            >
              <div className="flex items-start justify-between gap-3">
                <div>
                  <h3 className="font-medium">
                    {provider.provider_kind.toUpperCase()}
                  </h3>
                  <p className="mt-1 text-sm text-(--yo-text-muted)">
                    {siteName(sites, provider.site_id)}
                  </p>
                  <p className="mt-1 break-all text-sm text-(--yo-text-muted)">
                    {configSummary(provider)}
                  </p>
                </div>
                <div className="flex items-center gap-2">
                  <Button
                    size="sm"
                    variant="secondary"
                    onClick={() => onBeginEditModerationProvider(provider)}
                  >
                    编辑
                  </Button>
                  <Button
                    size="sm"
                    variant="ghost"
                    loading={isSavingModerationProvider}
                    onClick={() => onToggleModerationProviderEnabled(provider)}
                  >
                    {provider.enabled ? "停用" : "启用"}
                  </Button>
                </div>
              </div>

              {editingModerationProviderId === provider.id &&
                moderationProviderForm && (
                  <div className="mt-4 rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface) p-4">
                    <ModerationProviderFields
                      form={moderationProviderForm}
                      sites={sites}
                      allowKindChange={false}
                      onChange={onChangeModerationProviderForm}
                    />
                    {moderationProviderFormError && (
                      <p className="mt-3 text-sm text-(--yo-danger)">
                        {moderationProviderFormError}
                      </p>
                    )}
                    <div className="mt-4 flex flex-wrap gap-2">
                      <Button
                        size="sm"
                        loading={isSavingModerationProvider}
                        onClick={onSaveModerationProvider}
                      >
                        保存
                      </Button>
                      <Button
                        size="sm"
                        variant="ghost"
                        disabled={isSavingModerationProvider}
                        onClick={onCancelEditModerationProvider}
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
  );
};

const ModerationProviderFields = ({
  form,
  sites,
  allowKindChange,
  onChange,
}: {
  form: ModerationProviderFormState;
  sites: Site[];
  allowKindChange: boolean;
  onChange: (patch: Partial<ModerationProviderFormState>) => void;
}) => {
  return (
    <div className="mt-4 grid gap-4 md:grid-cols-2">
      {allowKindChange && (
        <>
          <label className="block">
            <span className="text-xs text-(--yo-text-soft)">站点</span>
            <select
              value={form.siteId}
              onChange={(event) =>
                onChange({ siteId: event.currentTarget.value })
              }
              className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
            >
              {sites.map((site) => (
                <option key={site.id} value={site.id}>
                  {site.name}
                </option>
              ))}
            </select>
          </label>
          <label className="block">
            <span className="text-xs text-(--yo-text-soft)">类型</span>
            <select
              value={form.providerKind}
              onChange={(event) =>
                onChange({
                  providerKind: event.currentTarget.value as "llm" | "akismet",
                })
              }
              className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
            >
              <option value="llm">LLM</option>
              <option value="akismet">Akismet</option>
            </select>
          </label>
        </>
      )}

      {form.providerKind === "llm" ? (
        <>
          <label className="block">
            <span className="text-xs text-(--yo-text-soft)">模型</span>
            <input
              type="text"
              value={form.model}
              onInput={(event) =>
                onChange({ model: event.currentTarget.value })
              }
              className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
            />
          </label>
          <label className="block">
            <span className="text-xs text-(--yo-text-soft)">API Base</span>
            <input
              type="url"
              value={form.apiBase}
              onInput={(event) =>
                onChange({ apiBase: event.currentTarget.value })
              }
              className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
            />
          </label>
          <label className="block">
            <span className="text-xs text-(--yo-text-soft)">API Key</span>
            <input
              type="password"
              value={form.apiKey}
              onInput={(event) =>
                onChange({ apiKey: event.currentTarget.value })
              }
              className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
            />
          </label>
          <label className="block md:col-span-2">
            <span className="text-xs text-(--yo-text-soft)">审核规则</span>
            <textarea
              value={form.rule}
              onInput={(event) => onChange({ rule: event.currentTarget.value })}
              rows={3}
              className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
            />
          </label>
        </>
      ) : (
        <>
          <label className="block">
            <span className="text-xs text-(--yo-text-soft)">API Key</span>
            <input
              type="password"
              value={form.apiKey}
              onInput={(event) =>
                onChange({ apiKey: event.currentTarget.value })
              }
              className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
            />
          </label>
          <label className="block">
            <span className="text-xs text-(--yo-text-soft)">Blog URL</span>
            <input
              type="url"
              value={form.blogUrl}
              onInput={(event) =>
                onChange({ blogUrl: event.currentTarget.value })
              }
              className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
            />
          </label>
        </>
      )}

      <label className="inline-flex items-center gap-2 text-sm text-(--yo-text-muted)">
        <input
          type="checkbox"
          checked={form.enabled}
          onChange={(event) =>
            onChange({ enabled: event.currentTarget.checked })
          }
          className="h-4 w-4 rounded border border-(--yo-surface-strong)"
        />
        启用
      </label>
    </div>
  );
};
