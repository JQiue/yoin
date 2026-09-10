import type { CommentTab, CommentTabItem } from "@/admin/types";
import type { CommentForAdmin, Paged, Site } from "@/shared/api/types";
import { Button } from "@/shared/components/Button";
import { formatLocalDateTime } from "@/shared/helper";
import type { MessageKey } from "@/shared/i18n";
import { useI18n } from "@/shared/i18n";

type CommentStatusOption = {
  value: CommentForAdmin["status"];
  labelKey: MessageKey;
};

interface Props {
  panelClass: string;
  sites: Site[];
  selectedCommentSiteId: number | null;
  commentPagePath: string;
  activeCommentTab: CommentTab;
  activeCommentTabLabel: string;
  commentPageOffset: number;
  commentsPage: Paged<CommentForAdmin[]> | null;
  comments: CommentForAdmin[];
  isLoadingComments: boolean;
  commentsError: string;
  updatingCommentId: number | null;
  commentTabs: CommentTabItem[];
  commentStatusOptions: readonly CommentStatusOption[];
  onRefresh: () => void;
  onSelectSite: (siteId: number) => void;
  onChangePagePath: (path: string) => void;
  onChangeTab: (tab: CommentTab) => void;
  onUpdateCommentStatus: (
    commentId: number,
    status: CommentForAdmin["status"],
  ) => void;
  onPrevPage: () => void;
  onNextPage: () => void;
}

export const CommentsPanel = ({
  panelClass,
  sites,
  selectedCommentSiteId,
  commentPagePath,
  activeCommentTab,
  activeCommentTabLabel,
  commentPageOffset,
  commentsPage,
  comments,
  isLoadingComments,
  commentsError,
  updatingCommentId,
  commentTabs,
  commentStatusOptions,
  onRefresh,
  onSelectSite,
  onChangePagePath,
  onChangeTab,
  onUpdateCommentStatus,
  onPrevPage,
  onNextPage,
}: Props) => {
  const { t } = useI18n();
  return (
    <section className={panelClass}>
      <div className="mb-4 flex flex-col items-start gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-xl font-semibold">{t("admin.comments.title")}</h2>
          <p className="mt-1 text-sm text-(--yo-text-muted)">
            {t("admin.comments.desc")}
          </p>
        </div>
        <Button size="sm" variant="secondary" onClick={onRefresh}>
          {t("admin.comments.refresh")}
        </Button>
      </div>

      <div className="mb-4 grid gap-3 md:grid-cols-[minmax(0,240px)_minmax(0,1fr)]">
        <label className="block">
          <span className="text-xs text-(--yo-text-soft)">
            {t("common.site")}
          </span>
          <select
            value={selectedCommentSiteId ?? ""}
            onChange={(event) =>
              onSelectSite(Number(event.currentTarget.value))
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
          <span className="text-xs text-(--yo-text-soft)">
            {t("admin.comments.pagePath")}
          </span>
          <input
            type="text"
            value={commentPagePath}
            onInput={(event) =>
              onChangePagePath(event.currentTarget.value || "/")
            }
            className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm outline-none transition-colors focus:border-(--yo-primary)"
            placeholder="/"
          />
        </label>
      </div>

      <div className="mb-4 flex gap-2 overflow-x-auto pb-1">
        {commentTabs.map((tab) => {
          const isActive = activeCommentTab === tab.key;
          return (
            <Button
              key={tab.key}
              onClick={() => onChangeTab(tab.key)}
              size="sm"
              type="button"
              variant={isActive ? "primary" : "secondary"}
              className={`rounded-full whitespace-nowrap ${
                !isActive
                  ? "text-(--yo-text-muted) hover:bg-(--yo-surface)"
                  : ""
              }`}
            >
              {t(tab.labelKey)}
            </Button>
          );
        })}
      </div>

      <div className="mb-4 rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) px-4 py-3 text-sm text-(--yo-text-muted)">
        {t("admin.comments.currentView", { label: activeCommentTabLabel })}
      </div>

      {isLoadingComments ? (
        <p className="text-sm text-(--yo-text-muted)">
          {t("admin.comments.loading")}
        </p>
      ) : commentsError ? (
        <div className="rounded-lg bg-(--yo-danger-bg) px-4 py-3 text-sm text-(--yo-danger)">
          {commentsError}
        </div>
      ) : selectedCommentSiteId == null ? (
        <div className="rounded-lg border border-dashed border-(--yo-surface-strong) px-4 py-8 text-center text-sm text-(--yo-text-muted)">
          {t("admin.comments.needSite")}
        </div>
      ) : comments.length === 0 ? (
        <div className="rounded-lg border border-dashed border-(--yo-surface-strong) px-4 py-8 text-center text-sm text-(--yo-text-muted)">
          {t("admin.comments.empty")}
        </div>
      ) : (
        <div className="space-y-3">
          {comments.map((comment) => (
            <article
              key={comment.id}
              className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4"
            >
              <div className="flex items-start justify-between gap-3">
                <div>
                  <div className="flex flex-wrap items-center gap-2">
                    <h3 className="font-medium">{comment.nickname}</h3>
                    <span className="rounded-full bg-(--yo-surface) px-2 py-0.5 text-[11px] text-(--yo-text-muted)">
                      ID {comment.id}
                    </span>
                    {comment.parent_id != null && (
                      <span className="rounded-full bg-(--yo-surface) px-2 py-0.5 text-[11px] text-(--yo-text-muted)">
                        {t("admin.comments.reply", { id: comment.parent_id })}
                      </span>
                    )}
                    {comment.thread_id != null && (
                      <span className="rounded-full bg-(--yo-surface) px-2 py-0.5 text-[11px] text-(--yo-text-muted)">
                        Thread #{comment.thread_id}
                      </span>
                    )}
                  </div>
                  <div className="mt-2 flex flex-wrap gap-x-4 gap-y-1 text-xs text-(--yo-text-muted)">
                    <span>
                      {t("admin.comments.created", {
                        time: formatLocalDateTime(comment.created_at),
                      })}
                    </span>
                    <span>
                      {t("admin.comments.updated", {
                        time: formatLocalDateTime(comment.updated_at),
                      })}
                    </span>
                    {comment.website && (
                      <span>
                        {t("admin.comments.website", {
                          value: comment.website,
                        })}
                      </span>
                    )}
                    {comment.device && (
                      <span>
                        {t("admin.comments.device", { value: comment.device })}
                      </span>
                    )}
                    {comment.location && (
                      <span>
                        {t("admin.comments.location", {
                          value: comment.location,
                        })}
                      </span>
                    )}
                  </div>
                </div>
                <div className="flex flex-wrap justify-end gap-1">
                  {commentStatusOptions.map((option) => {
                    const isActive = comment.status === option.value;
                    return (
                      <Button
                        key={option.value}
                        disabled={updatingCommentId === comment.id || isActive}
                        type="button"
                        variant={isActive ? "primary" : "secondary"}
                        size="sm"
                        onClick={() =>
                          onUpdateCommentStatus(comment.id, option.value)
                        }
                        className={`rounded-full ${
                          !isActive
                            ? "bg-(--yo-surface) text-(--yo-text-muted) hover:bg-(--yo-surface-soft) disabled:hover:bg-(--yo-surface)"
                            : ""
                        }`}
                      >
                        {updatingCommentId === comment.id &&
                        option.value === comment.status
                          ? t("admin.comments.updating")
                          : t(option.labelKey)}
                      </Button>
                    );
                  })}
                </div>
              </div>
              <div
                className="mt-3 rounded-lg bg-(--yo-surface) px-4 py-3 text-sm leading-6"
                dangerouslySetInnerHTML={{ __html: comment.content }}
              />
            </article>
          ))}

          {commentsPage && commentsPage.total_pages > 0 && (
            <div className="flex flex-col gap-3 rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) px-4 py-3 text-sm text-(--yo-text-muted) sm:flex-row sm:items-center sm:justify-between">
              <p>
                {t("admin.comments.pager", {
                  total: commentsPage.total,
                  page: commentsPage.page_offset,
                  pages: commentsPage.total_pages,
                })}
              </p>
              <div className="flex gap-2">
                <Button
                  size="sm"
                  variant="ghost"
                  disabled={commentPageOffset <= 1}
                  onClick={onPrevPage}
                >
                  {t("common.previous")}
                </Button>
                <Button
                  size="sm"
                  variant="secondary"
                  disabled={commentPageOffset >= commentsPage.total_pages}
                  onClick={onNextPage}
                >
                  {t("common.next")}
                </Button>
              </div>
            </div>
          )}
        </div>
      )}
    </section>
  );
};
