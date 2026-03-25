import { useEffect, useState } from "preact/hooks";
import { storage } from "@/shared/helper";
import { useCommentFormStore, useCommentStore, useConfigStore } from "@/store";

const LOCATION_CHANGE_EVENT = "yoin:locationchange";

const emitLocationChange = () => {
	window.dispatchEvent(new Event(LOCATION_CHANGE_EVENT));
};

const patchHistoryMethods = () => {
	const historyState = window.history as History & {
		__yoinLocationPatched__?: boolean;
	};

	if (historyState.__yoinLocationPatched__) {
		return;
	}

	const originalPushState = window.history.pushState.bind(window.history);
	const originalReplaceState = window.history.replaceState.bind(window.history);

	window.history.pushState = (...args) => {
		originalPushState(...args);
		emitLocationChange();
	};

	window.history.replaceState = (...args) => {
		originalReplaceState(...args);
		emitLocationChange();
	};

	historyState.__yoinLocationPatched__ = true;
};

const useCommentPagePath = () => {
	const [pathname, setPathname] = useState(() => location.pathname);

	useEffect(() => {
		patchHistoryMethods();

		const syncPathname = () => {
			setPathname(location.pathname);
		};

		window.addEventListener("popstate", syncPathname);
		window.addEventListener(LOCATION_CHANGE_EVENT, syncPathname);

		return () => {
			window.removeEventListener("popstate", syncPathname);
			window.removeEventListener(LOCATION_CHANGE_EVENT, syncPathname);
		};
	}, []);

	return pathname;
};

const useHydrateCommentDraft = () => {
	const setField = useCommentFormStore((state) => state.setField);

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
	}, [setField]);
};

const useInitializeComments = (siteId?: number) => {
	const fetchComments = useCommentStore((state) => state.fetchComments);
	const pathname = useCommentPagePath();

	useEffect(() => {
		if (siteId == null) {
			return;
		}

		fetchComments(1, false);
	}, [fetchComments, pathname, siteId]);
};

export const useInitializeCommentPage = () => {
	const siteId = useConfigStore((state) => state.config.site_id);

	useHydrateCommentDraft();
	useInitializeComments(siteId);
};
