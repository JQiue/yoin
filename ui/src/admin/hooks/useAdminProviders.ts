import { useEffect, useState } from "preact/hooks";
import type {
  ModerationProviderFormState,
  OAuthProviderFormState,
} from "@/admin/types";
import {
  createAdminModerationProvider,
  createAdminOauthProvider,
  fetchAdminModerationProviders,
  fetchAdminOauthProviders,
  updateAdminModerationProvider,
  updateAdminOauthProvider,
} from "@/shared/api";
import type {
  AkismetModerationConfig,
  LlmModerationConfig,
  ModerationProvider,
  OauthProvider,
  Site,
} from "@/shared/api/types";

const emptyCreateOAuthProviderForm: OAuthProviderFormState = {
  providerCode: "github",
  clientId: "",
  clientSecret: "",
  redirectUri: "",
  enabled: true,
};

const emptyCreateModerationProviderForm: ModerationProviderFormState = {
  siteId: "",
  providerKind: "llm",
  enabled: true,
  model: "",
  apiBase: "",
  apiKey: "",
  rule: "",
  blogUrl: "",
};

function isLlmConfig(
  config: ModerationProvider["config"],
): config is LlmModerationConfig {
  return "model" in config && "api_base" in config;
}

function isAkismetConfig(
  config: ModerationProvider["config"],
): config is AkismetModerationConfig {
  return "blog_url" in config;
}

function formFromProvider(
  provider: ModerationProvider,
): ModerationProviderFormState {
  const llm = isLlmConfig(provider.config) ? provider.config : null;
  const akismet = isAkismetConfig(provider.config) ? provider.config : null;
  return {
    siteId: String(provider.site_id),
    providerKind: provider.provider_kind,
    enabled: provider.enabled,
    model: llm?.model ?? "",
    apiBase: llm?.api_base ?? "",
    apiKey: llm?.api_key ?? "",
    rule: llm?.rule ?? "",
    blogUrl: akismet?.blog_url ?? "",
  };
}

export const useAdminProviders = (activeTab: string, sites: Site[]) => {
  const [oauthProviders, setOauthProviders] = useState<OauthProvider[]>([]);
  const [isLoadingOauthProviders, setIsLoadingOauthProviders] = useState(false);
  const [oauthProvidersError, setOauthProvidersError] = useState("");
  const [isCreatingOAuthProvider, setIsCreatingOAuthProvider] = useState(false);
  const [createOAuthProviderForm, setCreateOAuthProviderForm] =
    useState<OAuthProviderFormState>(emptyCreateOAuthProviderForm);
  const [createOAuthProviderError, setCreateOAuthProviderError] = useState("");
  const [isSubmittingOAuthProvider, setIsSubmittingOAuthProvider] =
    useState(false);
  const [updatingOauthProviderId, setUpdatingOauthProviderId] = useState<
    number | null
  >(null);

  const [moderationProviders, setModerationProviders] = useState<
    ModerationProvider[]
  >([]);
  const [isLoadingModerationProviders, setIsLoadingModerationProviders] =
    useState(false);
  const [moderationProvidersError, setModerationProvidersError] = useState("");
  const [isCreatingModerationProvider, setIsCreatingModerationProvider] =
    useState(false);
  const [createModerationProviderForm, setCreateModerationProviderForm] =
    useState<ModerationProviderFormState>(emptyCreateModerationProviderForm);
  const [createModerationProviderError, setCreateModerationProviderError] =
    useState("");
  const [isSubmittingModerationProvider, setIsSubmittingModerationProvider] =
    useState(false);
  const [editingModerationProviderId, setEditingModerationProviderId] =
    useState<number | null>(null);
  const [moderationProviderForm, setModerationProviderForm] =
    useState<ModerationProviderFormState | null>(null);
  const [moderationProviderFormError, setModerationProviderFormError] =
    useState("");
  const [isSavingModerationProvider, setIsSavingModerationProvider] =
    useState(false);

  useEffect(() => {
    if (activeTab !== "oauthProviders") return;
    let alive = true;
    setIsLoadingOauthProviders(true);
    setOauthProvidersError("");
    fetchAdminOauthProviders()
      .then((res) => {
        if (!alive) return;
        setOauthProviders(res.data);
      })
      .catch((error) => {
        if (!alive) return;
        setOauthProvidersError(
          error instanceof Error ? error.message : "加载 OAuth 提供者失败",
        );
      })
      .finally(() => {
        if (!alive) return;
        setIsLoadingOauthProviders(false);
      });
    return () => {
      alive = false;
    };
  }, [activeTab]);

  useEffect(() => {
    if (activeTab !== "moderationProviders") return;
    let alive = true;
    setIsLoadingModerationProviders(true);
    setModerationProvidersError("");
    fetchAdminModerationProviders()
      .then((res) => {
        if (!alive) return;
        setModerationProviders(res.data);
      })
      .catch((error) => {
        if (!alive) return;
        setModerationProvidersError(
          error instanceof Error ? error.message : "加载审核提供者失败",
        );
      })
      .finally(() => {
        if (!alive) return;
        setIsLoadingModerationProviders(false);
      });
    return () => {
      alive = false;
    };
  }, [activeTab]);

  useEffect(() => {
    if (sites.length === 0) return;
    setCreateModerationProviderForm((current) =>
      current.siteId ? current : { ...current, siteId: String(sites[0].id) },
    );
  }, [sites]);

  const createOAuthProvider = async () => {
    if (!createOAuthProviderForm.providerCode.trim()) {
      setCreateOAuthProviderError("提供者代码不能为空");
      return;
    }
    if (!createOAuthProviderForm.clientId.trim()) {
      setCreateOAuthProviderError("Client ID 不能为空");
      return;
    }
    if (!createOAuthProviderForm.clientSecret.trim()) {
      setCreateOAuthProviderError("Client Secret 不能为空");
      return;
    }
    if (!createOAuthProviderForm.redirectUri.trim()) {
      setCreateOAuthProviderError("回调地址不能为空");
      return;
    }

    setIsSubmittingOAuthProvider(true);
    setCreateOAuthProviderError("");
    try {
      const res = await createAdminOauthProvider({
        provider_code: createOAuthProviderForm.providerCode.trim(),
        client_id: createOAuthProviderForm.clientId.trim(),
        client_secret: createOAuthProviderForm.clientSecret.trim(),
        redirect_uri: createOAuthProviderForm.redirectUri.trim(),
        enabled: createOAuthProviderForm.enabled,
      });
      setOauthProviders((current) => [res.data, ...current]);
      setIsCreatingOAuthProvider(false);
      setCreateOAuthProviderForm(emptyCreateOAuthProviderForm);
    } catch (error) {
      setCreateOAuthProviderError(
        error instanceof Error ? error.message : "创建 OAuth 提供者失败",
      );
    } finally {
      setIsSubmittingOAuthProvider(false);
    }
  };

  const toggleOauthProviderEnabled = async (provider: OauthProvider) => {
    setUpdatingOauthProviderId(provider.id);
    setOauthProvidersError("");
    try {
      const res = await updateAdminOauthProvider(provider.id, {
        enabled: !provider.enabled,
      });
      setOauthProviders((current) =>
        current.map((item) => (item.id === provider.id ? res.data : item)),
      );
    } catch (error) {
      setOauthProvidersError(
        error instanceof Error ? error.message : "更新 OAuth 提供者失败",
      );
    } finally {
      setUpdatingOauthProviderId(null);
    }
  };

  const buildModerationConfig = (
    form: ModerationProviderFormState,
  ):
    | { error: string; config?: never }
    | { error?: never; config: ModerationProvider["config"] } => {
    if (form.providerKind === "llm") {
      if (!form.model.trim() || !form.apiBase.trim() || !form.apiKey.trim()) {
        return { error: "LLM 审核需要填写模型、API Base 和 API Key" };
      }
      return {
        config: {
          model: form.model.trim(),
          api_base: form.apiBase.trim(),
          api_key: form.apiKey.trim(),
          rule: form.rule.trim(),
        },
      };
    }
    if (!form.apiKey.trim() || !form.blogUrl.trim()) {
      return { error: "Akismet 审核需要填写 API Key 和站点地址" };
    }
    return {
      config: {
        api_key: form.apiKey.trim(),
        blog_url: form.blogUrl.trim(),
      },
    };
  };

  const createModerationProvider = async () => {
    const siteId = Number(createModerationProviderForm.siteId);
    if (!Number.isFinite(siteId) || siteId <= 0) {
      setCreateModerationProviderError("请选择站点");
      return;
    }
    const built = buildModerationConfig(createModerationProviderForm);
    if (built.error) {
      setCreateModerationProviderError(built.error);
      return;
    }

    setIsSubmittingModerationProvider(true);
    setCreateModerationProviderError("");
    try {
      const res = await createAdminModerationProvider({
        site_id: siteId,
        provider_kind: createModerationProviderForm.providerKind,
        enabled: createModerationProviderForm.enabled,
        config: built.config,
      });
      setModerationProviders((current) => [res.data, ...current]);
      setIsCreatingModerationProvider(false);
      setCreateModerationProviderForm({
        ...emptyCreateModerationProviderForm,
        siteId: String(sites[0]?.id ?? ""),
      });
    } catch (error) {
      setCreateModerationProviderError(
        error instanceof Error ? error.message : "创建审核提供者失败",
      );
    } finally {
      setIsSubmittingModerationProvider(false);
    }
  };

  const beginEditModerationProvider = (provider: ModerationProvider) => {
    setEditingModerationProviderId(provider.id);
    setModerationProviderForm(formFromProvider(provider));
    setModerationProviderFormError("");
  };

  const cancelEditModerationProvider = () => {
    setEditingModerationProviderId(null);
    setModerationProviderForm(null);
    setModerationProviderFormError("");
  };

  const saveModerationProvider = async () => {
    if (editingModerationProviderId == null || moderationProviderForm == null) {
      return;
    }
    const built = buildModerationConfig(moderationProviderForm);
    if (built.error) {
      setModerationProviderFormError(built.error);
      return;
    }

    setIsSavingModerationProvider(true);
    setModerationProviderFormError("");
    try {
      const res = await updateAdminModerationProvider(
        editingModerationProviderId,
        {
          enabled: moderationProviderForm.enabled,
          config: built.config,
        },
      );
      setModerationProviders((current) =>
        current.map((item) =>
          item.id === editingModerationProviderId ? res.data : item,
        ),
      );
      cancelEditModerationProvider();
    } catch (error) {
      setModerationProviderFormError(
        error instanceof Error ? error.message : "保存审核提供者失败",
      );
    } finally {
      setIsSavingModerationProvider(false);
    }
  };

  const toggleModerationProviderEnabled = async (
    provider: ModerationProvider,
  ) => {
    setIsSavingModerationProvider(true);
    setModerationProvidersError("");
    try {
      const res = await updateAdminModerationProvider(provider.id, {
        enabled: !provider.enabled,
      });
      setModerationProviders((current) =>
        current.map((item) => (item.id === provider.id ? res.data : item)),
      );
    } catch (error) {
      setModerationProvidersError(
        error instanceof Error ? error.message : "更新审核提供者失败",
      );
    } finally {
      setIsSavingModerationProvider(false);
    }
  };

  return {
    oauthProviders,
    isLoadingOauthProviders,
    oauthProvidersError,
    isCreatingOAuthProvider,
    createOAuthProviderForm,
    createOAuthProviderError,
    isSubmittingOAuthProvider,
    updatingOauthProviderId,
    setIsCreatingOAuthProvider,
    setCreateOAuthProviderForm,
    setCreateOAuthProviderError,
    createOAuthProvider,
    toggleOauthProviderEnabled,
    moderationProviders,
    isLoadingModerationProviders,
    moderationProvidersError,
    isCreatingModerationProvider,
    createModerationProviderForm,
    createModerationProviderError,
    isSubmittingModerationProvider,
    editingModerationProviderId,
    moderationProviderForm,
    moderationProviderFormError,
    isSavingModerationProvider,
    setIsCreatingModerationProvider,
    setCreateModerationProviderForm,
    setCreateModerationProviderError,
    setModerationProviderForm,
    createModerationProvider,
    beginEditModerationProvider,
    cancelEditModerationProvider,
    saveModerationProvider,
    toggleModerationProviderEnabled,
  };
};
