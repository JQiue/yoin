import { create } from "zustand";
import { fetchCommentsList } from "../api";
import type { CommentsState } from "./type";
import { useConfigStore } from "./useConfigStore";

export const useCommentStore = create<CommentsState>((set, get) => ({
	comments: [],
	total: 0,
	pageOffset: 1,
	pageSize: 10,
	totalPages: 0,
	sort: "created_desc",
	isLoading: false,
	setComments: (comments) => set({ comments }),
	fetchComments: async (pageOffset = 1, append = false) => {
		const { config } = useConfigStore.getState();
		const { comments: oldComments, pageSize, sort } = get();
		set({ isLoading: true });
		const {
			data: { items, total, total_pages },
		} = await fetchCommentsList(
			config.site_id,
			pageOffset,
			pageSize,
			location.pathname,
			sort,
		);
		set({
			comments: append ? [...oldComments, ...items] : items,
			total,
			pageOffset,
			totalPages: total_pages,
		});
		set({ isLoading: false });
	},
	fetchNextPage: async () => {
		const { pageOffset, fetchComments } = get();
		fetchComments(pageOffset + 1, true);
	},
	changeSort: (newSort: string) => {
		set({ sort: newSort, pageOffset: 1 });
		get().fetchComments();
	},
}));
