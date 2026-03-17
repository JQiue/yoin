import { useEffect } from "preact/compat";
import { storage } from "../shared/helper";
import { useCommentFormStore, useCommentStore } from "../store";

export const useInitializeCommentPage = () => {
	const { fetchComments } = useCommentStore();
	const { setField } = useCommentFormStore();

	useEffect(() => {
		const saveUserInfo = storage.get("yoin:user_info");
		const saveDraft = storage.get("yoin:comment_draft");

		if (saveUserInfo) {
			setField("nickname", saveUserInfo.nickname);
			setField("email", saveUserInfo.email);
			setField("website", saveUserInfo.website);
		}

		if (saveDraft) {
			setField("content", saveDraft);
		}

		fetchComments();
	}, []);
};
