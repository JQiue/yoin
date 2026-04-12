import type { TargetedSubmitEvent } from "preact";

import { useState } from "preact/hooks";
import type { StoredUser } from "@/client/hooks/useStoredUser";
import { useCommentStore } from "@/client/store";
import type { CommentForm } from "@/client/types";
import { getRuntimeConfig } from "@/config/runtime";
import { sendComment } from "@/shared/api/comment";
import type { Comment } from "@/shared/api/types";
import { storage } from "@/shared/helper";

type SubmitStatus = {
  type: "" | "success" | "error";
  msg: string;
};

type UseCommentSubmitOptions = {
  parentId?: number;
  currentUser: StoredUser | null;
  onCreated?: (createdComment?: Comment) => void;
  form: CommentForm;
  setField: (name: keyof CommentForm, value: string) => void;
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
  const { nickname, email, website, content } = form;

  const [submitting, setSubmitting] = useState(false);
  const [submitStatus, setSubmitStatus] = useState<SubmitStatus>({
    type: "",
    msg: "",
  });

  const setFormField = (name: keyof CommentForm, value: string) => {
    setField(name, value);
    if (name === "content") {
      storage.set("yoin:comment_draft", value);
    }
  };

  const handleSubmit = async (event: TargetedSubmitEvent<HTMLFormElement>) => {
    event.preventDefault();
    setSubmitting(true);
    setSubmitStatus({ type: "", msg: "" });

    if (siteId == null) {
      setSubmitStatus({ type: "error", msg: "缺少站点配置，暂时无法发表评论" });
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
      );

      if (resData.code === 0) {
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
      }
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
    email,
    website,
    submitting,
    submitStatus,
    setFormField,
    handleSubmit,
  };
};
