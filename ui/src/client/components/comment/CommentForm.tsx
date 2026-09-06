import type { TargetedEvent } from "preact";
import { useEffect } from "preact/hooks";
import Login from "@/client/components/auth/Login";
import CommentComposer from "@/client/components/comment/CommentComposer";
import CommentIdentityBar from "@/client/components/comment/CommentIdentityBar";
import CommentSubmitStatus from "@/client/components/comment/CommentSubmitStatus";
import { useAutoResizeTextarea } from "@/client/hooks/useAutoResizeTextarea";
import { useCommentForm } from "@/client/hooks/useCommentForm";
import { useCommentLoginModal } from "@/client/hooks/useCommentLoginModal";
import { useCommentSubmit } from "@/client/hooks/useCommentSubmit";
import { useStoredUser } from "@/client/hooks/useStoredUser";
import { useCommentStore } from "@/client/store";
import type { CommentForm } from "@/client/types";
import type { Comment } from "@/shared/api/types";
import { useI18n } from "@/shared/i18n";

interface Props {
  parent_id?: number;
  placeholder?: string;
  cb?: (createdComment?: Comment) => void;
}

export default (props: Props) => {
  const { currentUser, syncUserFromStorage, clearStoredUser } = useStoredUser();
  const allowAnonymous =
    useCommentStore((state) => state.siteConfig?.allow_anonymous) ?? false;
  const allowPrivate =
    useCommentStore((state) => state.siteConfig?.allow_private) ?? true;
  const maxCommentLength =
    useCommentStore((state) => state.siteConfig?.max_comment_length) ?? 1024;
  const { isOpen, open, close } = useCommentLoginModal();
  const { form, setField } = useCommentForm({
    hydrateDraft: props.parent_id == null,
  });
  const {
    content,
    nickname,
    email,
    website,
    is_private,
    is_anonymous,
    submitting,
    submitStatus,
    setFormField,
    handleSubmit,
  } = useCommentSubmit({
    parentId: props.parent_id,
    currentUser,
    onCreated: props.cb,
    form,
    setField,
  });
  const textareaRef = useAutoResizeTextarea(content);
  const { t } = useI18n();

  const fields: {
    name: keyof Omit<CommentForm, "content" | "is_private" | "is_anonymous">;
    placeholder: string;
    type: string;
  }[] = [
    {
      name: "nickname",
      placeholder: t("client.placeholder.nickname"),
      type: "text",
    },
    {
      name: "email",
      placeholder: t("client.placeholder.email"),
      type: "email",
    },
    {
      name: "website",
      placeholder: t("client.placeholder.website"),
      type: "url",
    },
  ];

  const fieldValues = { nickname, email, website };

  const handleAuthSuccess = () => {
    syncUserFromStorage();
    close();
  };

  const handleLogout = () => {
    clearStoredUser();
    setFormField("nickname", "");
    setFormField("email", "");
    setFormField("website", "");
  };

  const handleInputChange = (
    e: TargetedEvent<HTMLTextAreaElement | HTMLInputElement>,
  ) => {
    const { name, value } = e.currentTarget;
    setFormField(
      name as keyof Omit<CommentForm, "is_private" | "is_anonymous">,
      value,
    );
  };

  const handlePrivateChange = (checked: boolean) => {
    setFormField("is_private", checked);
  };

  const handleAnonymousChange = (checked: boolean) => {
    setFormField("is_anonymous", checked);
  };

  useEffect(() => {
    syncUserFromStorage();
  }, [syncUserFromStorage]);

  return (
    <div className="text-sm transition-all">
      <form className="space-y-3" onSubmit={handleSubmit}>
        <CommentIdentityBar
          currentUser={currentUser}
          fields={fields}
          fieldValues={fieldValues}
          onInputChange={handleInputChange}
          onLoginClick={open}
          onLogout={handleLogout}
        />
        <CommentComposer
          content={content}
          currentUser={currentUser}
          submitting={submitting}
          isPrivate={is_private}
          isAnonymous={is_anonymous}
          allowAnonymous={allowAnonymous}
          allowPrivate={allowPrivate}
          maxCommentLength={maxCommentLength}
          textareaRef={textareaRef}
          onInputChange={handleInputChange}
          onPrivateChange={handlePrivateChange}
          onAnonymousChange={handleAnonymousChange}
        />
        <CommentSubmitStatus type={submitStatus.type} msg={submitStatus.msg} />
      </form>

      {isOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center">
          <button
            aria-label={t("common.closeLogin")}
            className="absolute inset-0 bg-(--yo-overlay) backdrop-blur-sm"
            onClick={close}
            type="button"
          />
          <div
            aria-modal="true"
            className="relative z-10 w-full max-w-sm bg-(--yo-surface) p-3 text-(--yo-text)"
            role="dialog"
          >
            <Login onSuccess={handleAuthSuccess}></Login>
          </div>
        </div>
      )}
    </div>
  );
};
