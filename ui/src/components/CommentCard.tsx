import type { Comment } from "../api/types";
import { formatDate } from "../helper";
import Icon from "./Icon";
import CommentList from "./CommentList";
import Send from "./Send";
import { useState } from "preact/hooks";

interface Props {
	comment: Comment;
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

export default ({ comment }: Props) => {
	const [isReply, setIsReply] = useState(false);
	const safeWebsite = toSafeHttpUrl(comment.website);
	const handleClickReply = () => {
		setIsReply(!isReply);
	};

	return (
		<div className="p-4 my-3 rounded-sm border transition-all shadow-sm">
			<div className="flex items-start gap-2.5">
				<div
					className={`flex items-center justify-center shrink-0 w-8 h-8 sm:w-10 sm:h-10 rounded-full shadow-md font-bold text-xs`}
				>
					<img className="w-full h-full rounded-full" src={comment.avatar} alt="avatar" />
				</div>
				<div className="flex-1 min-w-0">
					<div className="flex items-center gap-x-2 mb-0.5 text-xs">
						<span className="font-bold text-brand-black tracking-tight">
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
						</span>
						<span className="text-[11px] text-app-muted flex items-center gap-1 ml-auto">
							<Icon name="clock" />
							{formatDate(comment.created_at)}
						</span>
					</div>
					<p className="text-[13px] text-brand-black leading-snug whitespace-pre-wrap">
						{comment.content}
					</p>
					<div className="mt-1.5 flex items-center justify-between text-[12px] text-app-muted leading-none">
						<div className="flex gap-2">
							<button
								className="flex items-center gap-1 hover:text-brand-black transition-colors"
								type="button"
							>
								<Icon name="thumbsUp" />
								{comment.up_vote || 0}
							</button>
							<button
								className="flex items-center gap-1 hover:text-brand-black transition-colors"
								type="button"
							>
								<Icon name="thumbsDown" />
								{comment.down_vote || 0}
							</button>
							<button type="button" aria-label="回复评论" onClick={handleClickReply}>
								<Icon name="reply" />
							</button>
						</div>
						<div className="text-[10px] text-app-muted">
							<span>{comment.location || "LOCAL"}</span>
						</div>
					</div>
				</div>
			</div>
			{isReply ? (
				<div class="mt-2">
					<Send parent_id={comment.id} cb={() => setIsReply(false)}></Send>
				</div>
			) : null}
			<div>
				{comment.replies && (
					<CommentList comments={comment.replies} isReply={true}></CommentList>
				)}
			</div>
		</div>
	);
};
