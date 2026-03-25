import type { TargetedSubmitEvent } from "preact";

import { useState } from "preact/hooks";

import { sendComment } from "@/shared/api/comment";
import type { Comment } from "@/shared/api/types";
import { storage } from "@/shared/helper";
import { useCommentFormStore, useCommentStore, useConfigStore } from "@/store";
import type { CommentForm } from "@/store/types";

import type { StoredUser } from "@/hook/useStoredUser";

type SubmitStatus = {
	type: "" | "success" | "error";
	msg: string;
};

type UseCommentSubmitOptions = {
	parentId?: number;
	currentUser: StoredUser | null;
	onCreated?: (createdComment?: Comment) => void;
};

export const useCommentSubmit = ({
	parentId,
	currentUser,
	onCreated,
}: UseCommentSubmitOptions) => {
	const siteId = useConfigStore((state) => state.config.site_id);
	const fetchComments = useCommentStore((state) => state.fetchComments);
	const nickname = useCommentFormStore((state) => state.nickname);
	const email = useCommentFormStore((state) => state.email);
	const website = useCommentFormStore((state) => state.website);
	const content = useCommentFormStore((state) => state.content);
	const setField = useCommentFormStore((state) => state.setField);

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
