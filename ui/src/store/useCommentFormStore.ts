import { create } from "zustand";
import type { CommentFormState } from "./type";

export const useCommentFormStore = create<CommentFormState>((set) => ({
	nickname: "",
	email: "",
		website: "",
		content: "",
		setField: (name, value) =>
			set((_state) => ({
				[name]: value,
			})),
	reset: () =>
		set({
				nickname: "",
				email: "",
				website: "",
				content: "",
		}),
}));
