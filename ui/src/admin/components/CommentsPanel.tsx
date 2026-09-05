import type { CommentTab, CommentTabItem } from "@/admin/types";
import type { CommentForAdmin, Paged, Site } from "@/shared/api/types";
import { Button } from "@/shared/components/Button";
import { formatLocalDateTime } from "@/shared/helper";

type CommentStatusOption = {
  value: CommentForAdmin["status"];
  label: string;
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
  return (
    <section className={panelClass}>
      <div className="mb-4 flex flex-col items-start gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-xl font-semibold">评论管理</h2>
          <p className="mt-1 text-sm text-(--yo-text-muted)">
            现在先统一接到后台评论列表接口，按站点、页面和状态做筛选。
          </p>
        </div>
        <Button size="sm" variant="secondary" onClick={onRefresh}>
          刷新列表
        </Button>
      </div>

      <div className="mb-4 grid gap-3 md:grid-cols-[minmax(0,240px)_minmax(0,1fr)]">
        <label className="block">
          <span className="text-xs text-(--yo-text-soft)">站点</span>
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
          <span className="text-xs text-(--yo-text-soft)">页面路径</span>
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
              {tab.label}
            </Button>
          );
        })}
      </div>

      <div className="mb-4 rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) px-4 py-3 text-sm text-(--yo-text-muted)">
        当前视图：
        <span className="font-medium text-(--yo-text)">
          {activeCommentTabLabel}
        </span>
        。状态接口接上后，我们再按真实状态显示对应动作，而不是所有面板都摆同一组按钮。
      </div>

      {isLoadingComments ? (
        <p className="text-sm text-(--yo-text-muted)">正在加载评论列表...</p>
      ) : commentsError ? (
        <div className="rounded-lg bg-(--yo-danger-bg) px-4 py-3 text-sm text-(--yo-danger)">
          {commentsError}
        </div>
      ) : selectedCommentSiteId == null ? (
        <div className="rounded-lg border border-dashed border-(--yo-surface-strong) px-4 py-8 text-center text-sm text-(--yo-text-muted)">
          请先创建站点，再查看评论管理。
        </div>
      ) : comments.length === 0 ? (
        <div className="rounded-lg border border-dashed border-(--yo-surface-strong) px-4 py-8 text-center text-sm text-(--yo-text-muted)">
          当前筛选条件下没有评论，或者后端过滤逻辑还在继续完善。
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
                        回复 #{comment.parent_id}
                      </span>
                    )}
                    {comment.thread_id != null && (
                      <span className="rounded-full bg-(--yo-surface) px-2 py-0.5 text-[11px] text-(--yo-text-muted)">
                        Thread #{comment.thread_id}
                      </span>
                    )}
                  </div>
                  <div className="mt-2 flex flex-wrap gap-x-4 gap-y-1 text-xs text-(--yo-text-muted)">
                    <span>创建：{formatLocalDateTime(comment.created_at)}</span>
                    <span>更新：{formatLocalDateTime(comment.updated_at)}</span>
                    {comment.website && <span>主页：{comment.website}</span>}
                    {comment.device && <span>设备：{comment.device}</span>}
                    {comment.location && <span>地区：{comment.location}</span>}
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
                          ? "更新中..."
                          : option.label}
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
                共 {commentsPage.total} 条，当前第 {commentsPage.page_offset} 页
                / 共 {commentsPage.total_pages} 页
              </p>
              <div className="flex gap-2">
                <Button
                  size="sm"
                  variant="ghost"
                  disabled={commentPageOffset <= 1}
                  onClick={onPrevPage}
                >
                  上一页
                </Button>
                <Button
                  size="sm"
                  variant="secondary"
                  disabled={commentPageOffset >= commentsPage.total_pages}
                  onClick={onNextPage}
                >
                  下一页
                </Button>
              </div>
            </div>
          )}
        </div>
      )}
    </section>
  );
};
