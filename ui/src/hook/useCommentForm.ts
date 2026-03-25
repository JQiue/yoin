import { useEffect, useState } from "preact/hooks";
import { storage } from "@/shared/helper";
import type { CommentForm } from "@/store/types";

const emptyForm: CommentForm = {
  nickname: "",
  email: "",
  website: "",
  content: "",
};

export const useCommentForm = (options?: { hydrateDraft?: boolean }) => {
  const [form, setForm] = useState<CommentForm>(emptyForm);

  useEffect(() => {
    const savedUser = storage.get("yoin:user_info");
    const savedDraft = storage.get("yoin:comment_draft");

    setForm((current) => ({
      ...current,
      nickname: savedUser?.nickname ?? current.nickname,
      email: savedUser?.email ?? current.email,
      website: savedUser?.website ?? current.website,
      content: options?.hydrateDraft
        ? (savedDraft ?? current.content)
        : current.content,
    }));
  }, [options?.hydrateDraft]);

  const setField = (name: keyof CommentForm, value: string) => {
    setForm((current) => ({
      ...current,
      [name]: value,
    }));
  };

  return {
    form,
    setField,
  };
};
