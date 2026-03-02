import { http } from "./client";
import type { Comment, Paged } from "./types";

export const fetchCommentsList = (
	site_id: number,
	page_offset: number,
	page_size: number,
	page_path: string,
	sort: string,
) => {
	return http.get<Paged<Comment[]>>("/api/comments", {
		params: {
			site_id,
			page_offset,
			page_size,
			page_path,
			sort,
		},
	});
};

export const sendComment = (
	site_id: number,
	nickname: string,
	email: string,
	website: string,
	content: string,
	page_path: string,
	parent_id?: number,
) => {
	return http.post<Comment>("/api/comments", {
		site_id,
		nickname,
		website,
		content,
		page_path,
		email,
		parent_id,
	});
};
