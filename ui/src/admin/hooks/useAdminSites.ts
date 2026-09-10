import { useEffect, useState } from "preact/hooks";
import type { SiteFormState } from "@/admin/types";
import {
  createAdminSite,
  fetchAdminSites,
  updateAdminSite,
} from "@/shared/api";
import {
  GLOBAL_ALLOWED_REACTIONS,
  type Site,
  type SiteConfig,
} from "@/shared/api/types";
import { t } from "@/shared/i18n";

const emptyCreateSiteForm: SiteFormState = {
  name: "",
  url: "",
  allowAnonymous: true,
  allowPrivate: true,
  maxCommentLength: "5000",
  commentLimitSeconds: "30",
  allowedReactions: [...GLOBAL_ALLOWED_REACTIONS],
};

function createSiteFormState(site: Site): SiteFormState {
  return {
    name: site.name,
    url: site.url,
    allowAnonymous: site.config.allow_anonymous,
    allowPrivate: site.config.allow_private,
    maxCommentLength: String(site.config.max_comment_length),
    commentLimitSeconds: String(site.config.comment_limit_seconds),
    allowedReactions:
      site.config.allowed_reactions.length > 0
        ? site.config.allowed_reactions
        : [...GLOBAL_ALLOWED_REACTIONS],
  };
}

function toSiteConfig(form: SiteFormState): SiteConfig | string {
  const maxCommentLength = Number(form.maxCommentLength);
  const commentLimitSeconds = Number(form.commentLimitSeconds);
  if (!form.name.trim()) {
    return t("admin.sites.nameRequired");
  }
  if (!form.url.trim()) {
    return t("admin.sites.urlRequired");
  }
  if (!Number.isFinite(maxCommentLength) || maxCommentLength <= 0) {
    return t("admin.sites.maxLengthInvalid");
  }
  if (!Number.isFinite(commentLimitSeconds) || commentLimitSeconds < 0) {
    return t("admin.sites.rateLimitInvalid");
  }
  return {
    allow_anonymous: form.allowAnonymous,
    allow_private: form.allowPrivate,
    max_comment_length: maxCommentLength,
    comment_limit_seconds: commentLimitSeconds,
    allowed_reactions: form.allowedReactions,
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
        setSitesError(
          error instanceof Error ? error.message : t("admin.sites.loadFailed"),
        );
      })
      .finally(() => {
        if (!alive) return;
        setIsLoadingSites(false);
      });
    return () => {
      alive = false;
    };
  }, []);

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
    const config = toSiteConfig(siteForm);
    if (typeof config === "string") {
      setSiteFormError(config);
      return;
    }

    setIsSavingSite(true);
    setSiteFormError("");

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
      setSiteFormError(
        error instanceof Error ? error.message : t("admin.sites.saveFailed"),
      );
    } finally {
      setIsSavingSite(false);
    }
  };

  const createSite = async () => {
    const config = toSiteConfig(createSiteForm);
    if (typeof config === "string") {
      setCreateSiteError(config);
      return;
    }

    setIsSubmittingCreateSite(true);
    setCreateSiteError("");

    try {
      const res = await createAdminSite({
        name: createSiteForm.name.trim(),
        url: createSiteForm.url.trim(),
        config,
      });
      setSites((current) => [res.data, ...current]);
      setIsCreatingSite(false);
      setCreateSiteForm(emptyCreateSiteForm);
    } catch (error) {
      setCreateSiteError(
        error instanceof Error ? error.message : t("admin.sites.createFailed"),
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
