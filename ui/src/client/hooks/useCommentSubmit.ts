import type { TargetedSubmitEvent } from "preact";

import { useState } from "preact/hooks";
import type { StoredUser } from "@/client/hooks/useStoredUser";
import { useCommentStore } from "@/client/store";
import type { CommentForm } from "@/client/types";
import { getRuntimeConfig } from "@/config/runtime";
import { sendComment } from "@/shared/api/comment";
import type { Comment } from "@/shared/api/types";
import { storage } from "@/shared/helper";
import { t } from "@/shared/i18n";

type SubmitStatus = {
  type: "" | "success" | "error";
  msg: string;
};

type UseCommentSubmitOptions = {
  parentId?: number;
  currentUser: StoredUser | null;
  onCreated?: (createdComment?: Comment) => void;
  form: CommentForm;
  setField: <K extends keyof CommentForm>(
    name: K,
    value: CommentForm[K],
  ) => void;
};

export const useCommentSubmit = ({
  parentId,
  currentUser,
  onCreated,
  form,
  setField,
}: UseCommentSubmitOptions) => {
  const siteId = getRuntimeConfig().site_id;
  const fetchComments = useCommentStore((state) => state.fetchComments);
  const allowAnonymous =
    useCommentStore((state) => state.siteConfig?.allow_anonymous) ?? false;
  const allowPrivate =
    useCommentStore((state) => state.siteConfig?.allow_private) ?? true;
  const { nickname, email, website, content, is_private, is_anonymous } = form;

  const [submitting, setSubmitting] = useState(false);
  const [submitStatus, setSubmitStatus] = useState<SubmitStatus>({
    type: "",
    msg: "",
  });

  const setFormField = <K extends keyof CommentForm>(
    name: K,
    value: CommentForm[K],
  ) => {
    setField(name, value);
    if (name === "content" && typeof value === "string") {
      storage.set("yoin:comment_draft", value);
    }
  };

  const handleSubmit = async (event: TargetedSubmitEvent<HTMLFormElement>) => {
    event.preventDefault();
    setSubmitting(true);
    setSubmitStatus({ type: "", msg: "" });

    if (siteId == null) {
      setSubmitStatus({ type: "error", msg: t("client.missingSite") });
      setSubmitting(false);
      return;
    }

    try {
      const resData = await sendComment(
        siteId,
        nickname,
        email,
        website,
        content,
        location.pathname,
        parentId,
        allowPrivate && is_private,
        allowAnonymous && is_anonymous,
      );

      setField("is_private", false);
      setField("is_anonymous", false);
      setField("content", "");
      setSubmitStatus({ type: "success", msg: resData.msg });
      storage.set("yoin:user_info", {
        nickname,
        website,
        email,
        avatar: currentUser?.avatar,
      });
      storage.remove("yoin:comment_draft");
      await fetchComments();
      onCreated?.(resData.data);
    } catch (error) {
      setSubmitStatus({
        type: "error",
        msg: error instanceof Error ? error.message : String(error),
      });
    } finally {
      setSubmitting(false);
    }
  };

  return {
    content,
    nickname,
    is_anonymous,
    email,
    is_private,
    website,
    submitting,
    submitStatus,
    setFormField,
    handleSubmit,
  };
};
