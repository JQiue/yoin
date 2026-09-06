import type { SiteFormState } from "@/admin/types";
import { GLOBAL_ALLOWED_REACTIONS, type Site } from "@/shared/api/types";
import { Button } from "@/shared/components/Button";
import { useI18n } from "@/shared/i18n";

function toggleReaction(current: string[], reaction: string) {
  return current.includes(reaction)
    ? current.filter((item) => item !== reaction)
    : [...current, reaction];
}

function ReactionPicker({
  value,
  onChange,
}: {
  value: string[];
  onChange: (next: string[]) => void;
}) {
  const { t } = useI18n();
  return (
    <div className="mt-4">
      <p className="text-xs text-(--yo-text-soft)">
        {t("admin.sites.allowedReactions")}
      </p>
      <div className="mt-2 flex flex-wrap gap-2">
        {GLOBAL_ALLOWED_REACTIONS.map((reaction) => {
          const checked = value.includes(reaction);
          return (
            <label
              key={reaction}
              className="inline-flex items-center gap-1 rounded-full border border-(--yo-surface-strong) px-2 py-1 text-sm"
            >
              <input
                type="checkbox"
                checked={checked}
                onChange={() => onChange(toggleReaction(value, reaction))}
                className="h-3.5 w-3.5 rounded border border-(--yo-surface-strong)"
              />
              <span>{reaction}</span>
            </label>
          );
        })}
      </div>
    </div>
  );
}

interface Props {
  panelClass: string;
  isCreatingSite: boolean;
  createSiteForm: SiteFormState;
  createSiteError: string;
  isSubmittingCreateSite: boolean;
  isLoadingSites: boolean;
  sitesError: string;
  sites: Site[];
  editingSiteId: number | null;
  siteForm: SiteFormState | null;
  siteFormError: string;
  isSavingSite: boolean;
  onToggleCreateSite: () => void;
  onChangeCreateSiteForm: (patch: Partial<SiteFormState>) => void;
  onCancelCreateSite: () => void;
  onCreateSite: () => void;
  onBeginEditSite: (site: Site) => void;
  onChangeSiteForm: (patch: Partial<SiteFormState>) => void;
  onCancelEditSite: () => void;
  onSaveSite: () => void;
}

export const SitesPanel = ({
  panelClass,
  isCreatingSite,
  createSiteForm,
  createSiteError,
  isSubmittingCreateSite,
  isLoadingSites,
  sitesError,
  sites,
  editingSiteId,
  siteForm,
  siteFormError,
  isSavingSite,
  onToggleCreateSite,
  onChangeCreateSiteForm,
  onCancelCreateSite,
  onCreateSite,
  onBeginEditSite,
  onChangeSiteForm,
  onCancelEditSite,
  onSaveSite,
}: Props) => {
  const { t } = useI18n();
  return (
    <section className={panelClass}>
      <div className="mb-4 flex flex-col items-start gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-xl font-semibold">{t("admin.sites.title")}</h2>
        </div>
        <Button size="sm" onClick={onToggleCreateSite}>
          {isCreatingSite ? t("common.collapseForm") : t("admin.sites.create")}
        </Button>
      </div>

      {isCreatingSite && (
        <div className="mb-4 rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
          <h3 className="text-base font-medium">
            {t("admin.sites.createTitle")}
          </h3>
          <div className="mt-4 grid gap-4 md:grid-cols-2">
            <label className="block">
              <span className="text-xs text-(--yo-text-soft)">
                {t("admin.sites.name")}
              </span>
              <input
                type="text"
                value={createSiteForm.name}
                onInput={(event) =>
                  onChangeCreateSiteForm({ name: event.currentTarget.value })
                }
                className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
              />
            </label>

            <label className="block">
              <span className="text-xs text-(--yo-text-soft)">
                {t("admin.sites.url")}
              </span>
              <input
                type="url"
                value={createSiteForm.url}
                onInput={(event) =>
                  onChangeCreateSiteForm({ url: event.currentTarget.value })
                }
                className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
              />
            </label>

            <label className="block">
              <span className="text-xs text-(--yo-text-soft)">
                {t("admin.sites.maxLength")}
              </span>
              <input
                type="number"
                min="1"
                value={createSiteForm.maxCommentLength}
                onInput={(event) =>
                  onChangeCreateSiteForm({
                    maxCommentLength: event.currentTarget.value,
                  })
                }
                className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
              />
            </label>

            <label className="block">
              <span className="text-xs text-(--yo-text-soft)">
                {t("admin.sites.rateLimit")}
              </span>
              <input
                type="number"
                min="0"
                value={createSiteForm.commentLimitSeconds}
                onInput={(event) =>
                  onChangeCreateSiteForm({
                    commentLimitSeconds: event.currentTarget.value,
                  })
                }
                className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
              />
            </label>
          </div>

          <div className="mt-4 flex flex-wrap gap-4">
            <label className="inline-flex items-center gap-2 text-sm text-(--yo-text-muted)">
              <input
                type="checkbox"
                checked={createSiteForm.allowAnonymous}
                onChange={(event) =>
                  onChangeCreateSiteForm({
                    allowAnonymous: event.currentTarget.checked,
                  })
                }
                className="h-4 w-4 rounded border border-(--yo-surface-strong)"
              />
              {t("admin.sites.allowAnonymous")}
            </label>
            <label className="inline-flex items-center gap-2 text-sm text-(--yo-text-muted)">
              <input
                type="checkbox"
                checked={createSiteForm.allowPrivate}
                onChange={(event) =>
                  onChangeCreateSiteForm({
                    allowPrivate: event.currentTarget.checked,
                  })
                }
                className="h-4 w-4 rounded border border-(--yo-surface-strong)"
              />
              {t("admin.sites.allowPrivate")}
            </label>
          </div>
          <ReactionPicker
            value={createSiteForm.allowedReactions}
            onChange={(allowedReactions) =>
              onChangeCreateSiteForm({ allowedReactions })
            }
          />

          {createSiteError && (
            <p className="mt-3 text-sm text-(--yo-danger)">{createSiteError}</p>
          )}

          <div className="mt-4 flex flex-wrap gap-2">
            <Button
              size="sm"
              loading={isSubmittingCreateSite}
              onClick={onCreateSite}
            >
              {t("admin.sites.createAction")}
            </Button>
            <Button
              size="sm"
              variant="ghost"
              disabled={isSubmittingCreateSite}
              onClick={onCancelCreateSite}
            >
              {t("common.cancel")}
            </Button>
          </div>
        </div>
      )}

      {isLoadingSites ? (
        <p className="text-sm text-(--yo-text-muted)">
          {t("admin.sites.loading")}
        </p>
      ) : sitesError ? (
        <div className="rounded-lg bg-(--yo-danger-bg) px-4 py-3 text-sm text-(--yo-danger)">
          {sitesError}
        </div>
      ) : sites.length === 0 ? (
        <div className="rounded-lg border border-dashed border-(--yo-surface-strong) px-4 py-8 text-center text-sm text-(--yo-text-muted)">
          {t("admin.sites.empty")}
        </div>
      ) : (
        <div className="grid gap-4 xl:grid-cols-2">
          {sites.map((site) => (
            <article
              key={site.id}
              className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4"
            >
              <div className="flex items-start justify-between gap-3">
                <div>
                  <h3 className="font-medium">{site.name}</h3>
                  <p className="mt-1 break-all text-sm text-(--yo-text-muted)">
                    {site.url}
                  </p>
                </div>
                <div className="flex items-center gap-2">
                  <span className="rounded-full bg-(--yo-surface) px-2.5 py-1 text-xs text-(--yo-text-muted)">
                    ID {site.id}
                  </span>
                  <Button
                    size="sm"
                    variant="secondary"
                    onClick={() => onBeginEditSite(site)}
                  >
                    {t("common.edit")}
                  </Button>
                </div>
              </div>
              <div className="mt-4 grid grid-cols-2 gap-3 text-sm md:grid-cols-5">
                <div>
                  <p className="text-(--yo-text-soft)">
                    {t("admin.sites.anonymous")}
                  </p>
                  <p className="mt-1 font-medium">
                    {site.config.allow_anonymous
                      ? t("admin.sites.allowed")
                      : t("admin.sites.closed")}
                  </p>
                </div>
                <div>
                  <p className="text-(--yo-text-soft)">
                    {t("admin.sites.private")}
                  </p>
                  <p className="mt-1 font-medium">
                    {site.config.allow_private
                      ? t("admin.sites.allowed")
                      : t("admin.sites.closed")}
                  </p>
                </div>
                <div>
                  <p className="text-(--yo-text-soft)">
                    {t("admin.sites.maxLengthShort")}
                  </p>
                  <p className="mt-1 font-medium">
                    {site.config.max_comment_length}
                  </p>
                </div>
                <div>
                  <p className="text-(--yo-text-soft)">
                    {t("admin.sites.rateLimitShort")}
                  </p>
                  <p className="mt-1 font-medium">
                    {site.config.comment_limit_seconds}
                  </p>
                </div>
                <div>
                  <p className="text-(--yo-text-soft)">
                    {t("admin.sites.reactions")}
                  </p>
                  <p className="mt-1 font-medium">
                    {(site.config.allowed_reactions ?? []).join(" ") ||
                      t("common.none")}
                  </p>
                </div>
              </div>

              {editingSiteId === site.id && siteForm && (
                <div className="mt-4 rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface) p-4">
                  <div className="grid gap-4 md:grid-cols-2">
                    <label className="block">
                      <span className="text-xs text-(--yo-text-soft)">
                        {t("admin.sites.name")}
                      </span>
                      <input
                        type="text"
                        value={siteForm.name}
                        onInput={(event) =>
                          onChangeSiteForm({ name: event.currentTarget.value })
                        }
                        className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
                      />
                    </label>

                    <label className="block">
                      <span className="text-xs text-(--yo-text-soft)">
                        {t("admin.sites.url")}
                      </span>
                      <input
                        type="url"
                        value={siteForm.url}
                        onInput={(event) =>
                          onChangeSiteForm({ url: event.currentTarget.value })
                        }
                        className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
                      />
                    </label>

                    <label className="block">
                      <span className="text-xs text-(--yo-text-soft)">
                        {t("admin.sites.maxLength")}
                      </span>
                      <input
                        type="number"
                        min="1"
                        value={siteForm.maxCommentLength}
                        onInput={(event) =>
                          onChangeSiteForm({
                            maxCommentLength: event.currentTarget.value,
                          })
                        }
                        className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
                      />
                    </label>

                    <label className="block">
                      <span className="text-xs text-(--yo-text-soft)">
                        {t("admin.sites.rateLimit")}
                      </span>
                      <input
                        type="number"
                        min="0"
                        value={siteForm.commentLimitSeconds}
                        onInput={(event) =>
                          onChangeSiteForm({
                            commentLimitSeconds: event.currentTarget.value,
                          })
                        }
                        className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
                      />
                    </label>
                  </div>

                  <div className="mt-4 flex flex-wrap gap-4">
                    <label className="inline-flex items-center gap-2 text-sm text-(--yo-text-muted)">
                      <input
                        type="checkbox"
                        checked={siteForm.allowAnonymous}
                        onChange={(event) =>
                          onChangeSiteForm({
                            allowAnonymous: event.currentTarget.checked,
                          })
                        }
                        className="h-4 w-4 rounded border border-(--yo-surface-strong)"
                      />
                      {t("admin.sites.allowAnonymous")}
                    </label>
                    <label className="inline-flex items-center gap-2 text-sm text-(--yo-text-muted)">
                      <input
                        type="checkbox"
                        checked={siteForm.allowPrivate}
                        onChange={(event) =>
                          onChangeSiteForm({
                            allowPrivate: event.currentTarget.checked,
                          })
                        }
                        className="h-4 w-4 rounded border border-(--yo-surface-strong)"
                      />
                      {t("admin.sites.allowPrivate")}
                    </label>
                  </div>
                  <ReactionPicker
                    value={siteForm.allowedReactions}
                    onChange={(allowedReactions) =>
                      onChangeSiteForm({ allowedReactions })
                    }
                  />

                  {siteFormError && (
                    <p className="mt-3 text-sm text-(--yo-danger)">
                      {siteFormError}
                    </p>
                  )}

                  <div className="mt-4 flex flex-wrap gap-2">
                    <Button
                      size="sm"
                      loading={isSavingSite}
                      onClick={onSaveSite}
                    >
                      {t("admin.sites.save")}
                    </Button>
                    <Button
                      size="sm"
                      variant="ghost"
                      disabled={isSavingSite}
                      onClick={onCancelEditSite}
                    >
                      {t("common.cancel")}
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
