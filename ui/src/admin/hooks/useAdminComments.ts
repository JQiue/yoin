import { useEffect, useState } from "preact/hooks";
import type { CommentTab, CommentTabItem } from "@/admin/types";
import { fetchAdminComments, updateAdminCommentStatus } from "@/shared/api";
import type { CommentForAdmin, Paged, Site } from "@/shared/api/types";
import type { MessageKey } from "@/shared/i18n";
import { t } from "@/shared/i18n";

const COMMENT_PAGE_SIZE = 20;

export const COMMENT_TABS: CommentTabItem[] = [
  { key: "all", labelKey: "admin.comments.tabAll" },
  { key: "pending", labelKey: "admin.comments.tabPending" },
  { key: "spam", labelKey: "admin.comments.tabSpam" },
  { key: "deleted", labelKey: "admin.comments.tabDeleted" },
];

export const COMMENT_STATUS_OPTIONS: {
  value: CommentForAdmin["status"];
  labelKey: MessageKey;
}[] = [
  { value: "pending", labelKey: "admin.comments.statusPending" },
  { value: "approved", labelKey: "admin.comments.statusApproved" },
  { value: "spam", labelKey: "admin.comments.statusSpam" },
  { value: "deleted", labelKey: "admin.comments.statusDeleted" },
];

function getCommentStatusFilter(tab: CommentTab) {
  if (tab === "all") return undefined;
  if (tab === "pending") return "pending" as const;
  if (tab === "spam") return "spam" as const;
  return "deleted" as const;
}

function matchesCommentTab(tab: CommentTab, status: CommentForAdmin["status"]) {
  if (tab === "all") return true;
  if (tab === "pending") return status === "pending";
  if (tab === "spam") return status === "spam";
  return status === "deleted";
}

export const useAdminComments = (activeTab: string, sites: Site[]) => {
  const [activeCommentTab, setActiveCommentTab] = useState<CommentTab>("all");
  const [commentsPage, setCommentsPage] = useState<Paged<
    CommentForAdmin[]
  > | null>(null);
  const [selectedCommentSiteId, setSelectedCommentSiteId] = useState<
    number | null
  >(null);
  const [commentPagePath, setCommentPagePath] = useState("/");
  const [commentPageOffset, setCommentPageOffset] = useState(1);
  const [isLoadingComments, setIsLoadingComments] = useState(false);
  const [commentsError, setCommentsError] = useState("");
  const [updatingCommentId, setUpdatingCommentId] = useState<number | null>(
    null,
  );

  useEffect(() => {
    if (sites.length === 0) return;
    setSelectedCommentSiteId((current) => current ?? sites[0].id);
  }, [sites]);

  useEffect(() => {
    setCommentPageOffset(1);
  }, [activeCommentTab, selectedCommentSiteId, commentPagePath]);

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
        setCommentsError(
          error instanceof Error
            ? error.message
            : t("admin.comments.loadFailed"),
        );
      })
      .finally(() => {
        if (!alive) return;
        setIsLoadingComments(false);
      });
    return () => {
      alive = false;
    };
  }, [
    activeTab,
    activeCommentTab,
    selectedCommentSiteId,
    commentPagePath,
    commentPageOffset,
  ]);

  const refreshComments = () => {
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
          error instanceof Error
            ? error.message
            : t("admin.comments.loadFailed"),
        ),
      )
      .finally(() => setIsLoadingComments(false));
  };

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
          .filter((comment) =>
            matchesCommentTab(activeCommentTab, comment.status),
          );
        const total =
          activeCommentTab === "all" ||
          matchesCommentTab(activeCommentTab, status)
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
      setCommentsError(
        error instanceof Error
          ? error.message
          : t("admin.comments.updateFailed"),
      );
    } finally {
      setUpdatingCommentId(null);
    }
  };

  const comments = commentsPage?.items ?? [];
  const activeCommentTabLabel = t(
    COMMENT_TABS.find((tab) => tab.key === activeCommentTab)?.labelKey ??
      "admin.comments.fallback",
  );

  return {
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
  };
};
