import type { Comment } from "@/shared/api/types";
import { fetchCommentReplies } from "@/shared/api/comment";
import CommentForm from "./CommentForm";
import { formatDate } from "../../shared/helper";
import { useCommentStore, useConfigStore } from "../../store";
import { Button } from "../../shared/components/Button";
import CommentList from "./CommentList";
import { useEffect, useState } from "preact/hooks";

interface Props {
	comment: Comment;
	onDeleteComment?: (deletedComment: Comment) => void;
	onReplyCreated?: (createdComment: Comment) => void;
}

const toSafeHttpUrl = (raw: string) => {
	try {
		if (!raw) return null;
		const parsed = new URL(raw);
		if (parsed.protocol === "http:" || parsed.protocol === "https:") {
			return parsed.toString();
		}
		return null;
	} catch (_error) {
		return null;
	}
};

const findCommentById = (comments: Comment[], id: number): Comment | undefined => {
	for (const comment of comments) {
		if (comment.id === id) return comment;
		if (comment.replies?.length) {
			const matched = findCommentById(comment.replies, id);
			if (matched) return matched;
		}
	}
};

const REPLY_PAGE_SIZE = 3;

export default ({ comment, onDeleteComment, onReplyCreated }: Props) => {
	const isRootComment = comment.parent_id == null;
	const {
		comments,
		deleteComment,
		sort,
	} = useCommentStore();
	const { config } = useConfigStore();
	const [isReply, setIsReply] = useState(false);
	const [replies, setReplies] = useState(comment.replies ?? []);
	const [hasMoreReplies, setHasMoreReplies] = useState(Boolean(comment.has_more));
	const [replyPage, setReplyPage] = useState(comment.replies?.length ? 1 : 0);
	const [isLoadingReplies, setIsLoadingReplies] = useState(false);
	const safeWebsite = toSafeHttpUrl(comment.website);

	useEffect(() => {
		if (!isRootComment) return;
		setReplies(comment.replies ?? []);
		setHasMoreReplies(Boolean(comment.has_more));
		setReplyPage(comment.replies?.length ? 1 : 0);
	}, [comment.id, isRootComment]);

	const replyNickname = (id: number | null) => {
		if (id == null) return undefined;
		return findCommentById(comments, id)?.nickname;
	};

	const handleClickReply = () => {
		setIsReply(!isReply);
	};

	const handleClickDelete = () => {
		deleteComment(comment.id);
		onDeleteComment?.(comment);
	};

	const handleReplyCreated = (createdReply?: Comment) => {
		setIsReply(false);
		if (!createdReply) return;
		if (!isRootComment && onReplyCreated) {
			onReplyCreated(createdReply);
			return;
		}
		onReplyCreated?.(createdReply);
		setReplies((prev) => [...prev, createdReply]);
	};

	const handleDeleteReply = (deletedReply: Comment) => {
		setReplies((prev) => prev.filter((reply) => reply.id !== deletedReply.id));
	};

	const handleLoadMoreReplies = async () => {
		if (isLoadingReplies) return;
		if (config.site_id == null) return;
		const nextPage = replyPage + 1;
		setIsLoadingReplies(true);
		try {
			const {
				data: { items, total_pages, page_offset },
			} = await fetchCommentReplies(
				comment.id,
				config.site_id,
				nextPage,
				REPLY_PAGE_SIZE,
				location.pathname,
				sort,
			);
			setReplies((prev) => [...prev, ...items]);
			setReplyPage(page_offset);
			setHasMoreReplies(page_offset < total_pages);
		} finally {
			setIsLoadingReplies(false);
		}
	};

	return (
		<div className={`-mx-4 px-4 py-3 rounded-md transition-all hover:bg-zinc-100 ${comment.parent_id ? "ml-8 sm:ml-12" : ""}`}>
			<div className="flex items-start gap-2.5">
				<div
					className={`w-9 h-9 sm:w-11 sm:h-11 rounded-full`}
				>
					<img className="w-full h-full rounded-full" src={comment.avatar} alt="avatar" />
				</div>
				<div className="flex-1 min-w-0">
					<div className="flex items-center gap-x-2 text-[12px]">
						<span className="text-[14px] font-bold tracking-tight">
							{safeWebsite ? (
								<a
									href={safeWebsite}
									target="_blank"
									rel="noopener noreferrer"
									className="hover:underline"
								>
									{comment.nickname}
								</a>
							) : (
								<span>{comment.nickname}</span>
							)}
							{comment.parent_id && (<span>&gt;{replyNickname(comment.parent_id)}</span>)}
						</span>
						<span>
							{formatDate(comment.created_at)}
						</span>
						<span>{comment.location || "LOCAL"}</span>
					</div>
					<div
						className="w-full mt-2 comment-content "
						dangerouslySetInnerHTML={{ __html: comment.content }}
					>
					</div>
					<div className="mt-2 flex gap-2 text-xs group">
						<Button variant="ghost" size="sm">
							赞同
							{comment.up_vote || 0}
						</Button>
						<Button variant="ghost" size="sm">
							反对
							{comment.down_vote || 0}
						</Button>
						<Button className="invisible group-hover:visible" variant="ghost" size="sm" aria-label="回复评论" onClick={handleClickReply} >回复</Button>
						<Button className="invisible group-hover:visible" variant="ghost" size="sm" onClick={handleClickDelete} >删除</Button>
					</div>
				</div>
			</div>
			{isReply ? (
				<div class="mt-2">
					<CommentForm parent_id={comment.id} cb={handleReplyCreated}></CommentForm>
				</div>
			) : null}
			{isRootComment ? (
				<div>
					{replies.length > 0 && (
						<CommentList
							comments={replies}
							isReply={true}
							onDeleteComment={handleDeleteReply}
							onReplyCreated={handleReplyCreated}
						></CommentList>
					)}
					{hasMoreReplies ? (
						<div className="mt-2 ml-11 sm:ml-14">
							<button
								type="button"
								className="text-xs text-zinc-500 hover:text-zinc-900"
								onClick={handleLoadMoreReplies}
								disabled={isLoadingReplies}
							>
								{isLoadingReplies ? "加载中..." : "查看更多回复"}
							</button>
						</div>
					) : null}
				</div>
			) : null}
		</div>
	);
};
