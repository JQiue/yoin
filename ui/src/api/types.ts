export type Method = "GET" | "POST" | "PUT" | "DELETE";

export interface RequestConfig extends Omit<RequestInit, "method"> {
	params?: Record<string, string | number | boolean>;
	data?: unknown;
}

export interface ResData<T> {
	code: number;
	msg: string;
	data: T;
}

export type Paged<T> = {
	items: T;
	page_offset: number;
	page_size: number;
	total: number;
	total_pages: number;
};

export type Comment = {
	id: number;
	thread_id: number | null;
	parent_id: number | null;
	nickname: string;
	website: string;
	content: string;
	up_vote: number;
	down_vote: number;
	device: string;
	location: string;
	avatar: string;
	created_at: string;
	replies?: Comment[];
	has_more?: boolean;
};

export type Login = {
	token: string;
	avatar: string;
	nickname: string;
  website: string;
	email: string;
}
