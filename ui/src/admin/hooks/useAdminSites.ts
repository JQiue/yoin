import { useEffect, useState } from "preact/hooks";
import { createAdminSite, fetchAdminSites, updateAdminSite } from "@/shared/api";
import type { Site, SiteConfig } from "@/shared/api/types";
import type { SiteFormState } from "@/admin/types";

const emptyCreateSiteForm: SiteFormState = {
  name: "",
  url: "",
  allowAnonymous: true,
  maxCommentLength: "5000",
  commentLimitSeconds: "30",
};

function createSiteFormState(site: Site): SiteFormState {
  return {
    name: site.name,
    url: site.url,
    allowAnonymous: site.config.allow_anonymous,
    maxCommentLength: String(site.config.max_comment_length),
    commentLimitSeconds: String(site.config.comment_limit_seconds),
  };
}

export const useAdminSites = (activeTab: string) => {
  const [sites, setSites] = useState<Site[]>([]);
  const [isLoadingSites, setIsLoadingSites] = useState(false);
  const [sitesError, setSitesError] = useState("");
  const [editingSiteId, setEditingSiteId] = useState<number | null>(null);
  const [siteForm, setSiteForm] = useState<SiteFormState | null>(null);
  const [siteFormError, setSiteFormError] = useState("");
  const [isSavingSite, setIsSavingSite] = useState(false);
  const [isCreatingSite, setIsCreatingSite] = useState(false);
  const [createSiteForm, setCreateSiteForm] =
    useState<SiteFormState>(emptyCreateSiteForm);
  const [createSiteError, setCreateSiteError] = useState("");
  const [isSubmittingCreateSite, setIsSubmittingCreateSite] = useState(false);

  useEffect(() => {
    if (activeTab !== "sites") return;
    let alive = true;
    setIsLoadingSites(true);
    setSitesError("");
    fetchAdminSites()
      .then((res) => {
        if (!alive) return;
        setSites(res.data);
      })
      .catch((error) => {
        if (!alive) return;
        setSitesError(error instanceof Error ? error.message : "加载站点失败");
      })
      .finally(() => {
        if (!alive) return;
        setIsLoadingSites(false);
      });
    return () => {
      alive = false;
    };
  }, [activeTab]);

  useEffect(() => {
    if (activeTab !== "sites") {
      setEditingSiteId(null);
      setSiteForm(null);
      setSiteFormError("");
    }
  }, [activeTab]);

  const beginEditSite = (site: Site) => {
    setEditingSiteId(site.id);
    setSiteForm(createSiteFormState(site));
    setSiteFormError("");
  };

  const cancelEditSite = () => {
    setEditingSiteId(null);
    setSiteForm(null);
    setSiteFormError("");
  };

  const saveSite = async () => {
    if (editingSiteId == null || siteForm == null) return;
    const maxCommentLength = Number(siteForm.maxCommentLength);
    const commentLimitSeconds = Number(siteForm.commentLimitSeconds);

    if (!siteForm.name.trim()) {
      setSiteFormError("站点名称不能为空");
      return;
    }
    if (!siteForm.url.trim()) {
      setSiteFormError("站点地址不能为空");
      return;
    }
    if (!Number.isFinite(maxCommentLength) || maxCommentLength <= 0) {
      setSiteFormError("最大长度必须是大于 0 的数字");
      return;
    }
    if (!Number.isFinite(commentLimitSeconds) || commentLimitSeconds < 0) {
      setSiteFormError("限流秒数必须是大于等于 0 的数字");
      return;
    }

    setIsSavingSite(true);
    setSiteFormError("");

    const config: SiteConfig = {
      allow_anonymous: siteForm.allowAnonymous,
      max_comment_length: maxCommentLength,
      comment_limit_seconds: commentLimitSeconds,
    };

    try {
      const res = await updateAdminSite({
        id: editingSiteId,
        name: siteForm.name.trim(),
        url: siteForm.url.trim(),
        config,
      });
      setSites((current) =>
        current.map((site) => (site.id === editingSiteId ? res.data : site)),
      );
      setEditingSiteId(null);
      setSiteForm(null);
    } catch (error) {
      setSiteFormError(error instanceof Error ? error.message : "保存站点失败");
    } finally {
      setIsSavingSite(false);
    }
  };

  const createSite = async () => {
    const maxCommentLength = Number(createSiteForm.maxCommentLength);
    const commentLimitSeconds = Number(createSiteForm.commentLimitSeconds);

    if (!createSiteForm.name.trim()) {
      setCreateSiteError("站点名称不能为空");
      return;
    }
    if (!createSiteForm.url.trim()) {
      setCreateSiteError("站点地址不能为空");
      return;
    }
    if (!Number.isFinite(maxCommentLength) || maxCommentLength <= 0) {
      setCreateSiteError("最大长度必须是大于 0 的数字");
      return;
    }
    if (!Number.isFinite(commentLimitSeconds) || commentLimitSeconds < 0) {
      setCreateSiteError("限流秒数必须是大于等于 0 的数字");
      return;
    }

    setIsSubmittingCreateSite(true);
    setCreateSiteError("");

    try {
      const res = await createAdminSite({
        name: createSiteForm.name.trim(),
        url: createSiteForm.url.trim(),
        config: {
          allow_anonymous: createSiteForm.allowAnonymous,
          max_comment_length: maxCommentLength,
          comment_limit_seconds: commentLimitSeconds,
        },
      });
      setSites((current) => [res.data, ...current]);
      setIsCreatingSite(false);
      setCreateSiteForm(emptyCreateSiteForm);
    } catch (error) {
      setCreateSiteError(
        error instanceof Error ? error.message : "创建站点失败",
      );
    } finally {
      setIsSubmittingCreateSite(false);
    }
  };

  return {
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
  };
};
