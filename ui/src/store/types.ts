import type { Comment } from "@/shared/api/types";

export interface AppConfig {
	containerId: string;
	api_base?: string;
	site_id?: number;
}

export interface ConfigState {
	config: AppConfig;
	setConfig: (options: AppConfig) => void;
}

export interface CommentForm {
	nickname: string;
	email: string;
	website: string;
	content: string;
}

export interface CommentFormState {
	nickname: string;
	email: string;
	website: string;
	content: string;
	setField: (name: keyof CommentForm, value: string) => void;
	reset: () => void;
}

export interface CommentsState {
	comments: Comment[];
	total: number;
	pageOffset: number;
	pageSize: number;
	totalPages: number;
	sort: string;
	isLoading: boolean;
	setComments: (comments: Comment[]) => void;
	fetchComments: (pageOffset?: number, append?: boolean) => void;
	fetchNextPage: () => void;
	changeSort: (newSort: string) => void;
	deleteComment: (id: number) => void;
}
