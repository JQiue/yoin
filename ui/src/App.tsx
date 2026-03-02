import { useEffect, useRef } from "preact/compat";
import CommentList from "./components/CommentList";
import Send from "./components/Send";
import { storage } from "./helper";

import { useCommentStore, useCommentFormStore } from "./store";
import Sort from "./components/Sort";

const App = () => {
	const {
		comments,
		fetchComments,
		fetchNextPage,
		total,
		pageOffset,
		totalPages,
		isLoading,
	} = useCommentStore();
	const { setField } = useCommentFormStore();
	const sentinelRef = useRef<HTMLDivElement>(null);

	const init = () => {
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
	};

	const renderFooter = () => {
		const containerClass =
			"flex justify-center items-center py-8 w-full text-sm text-app-muted";

		if (isLoading)
			return (
				<div className={containerClass}>
					<svg
						className="animate-spin -ml-1 mr-3 h-5 w-5 text-app-muted"
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
					>
						<title>Loading</title>
						<circle
							className="opacity-25"
							cx="12"
							cy="12"
							r="10"
							stroke="currentColor"
							strokeWidth="4"
						></circle>
						<path
							className="opacity-75"
							fill="currentColor"
							d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
						></path>
					</svg>
					<span>正在努力加载...</span>
				</div>
			);
		if (pageOffset >= totalPages)
			return (
				<div className={containerClass}>
					<div className="grow border-t border-app-border"></div>
					<span className="mx-4 text-app-muted select-none">这就到底啦 ☕️</span>
					<div className="grow border-t border-app-border"></div>
				</div>
			);
	};

	useEffect(() => {
		fetchComments();
		init();
	}, []);

	useEffect(() => {
		if (pageOffset >= totalPages || isLoading) return;

		const observer = new IntersectionObserver(
			(entries) => {
				if (entries[0].isIntersecting) {
					fetchNextPage();
				}
			},
			{ threshold: 0.1 },
		);

		if (sentinelRef.current) {
			observer.observe(sentinelRef.current);
		}

		return () => observer.disconnect();
	}, [pageOffset, totalPages, isLoading]);

	return (
		<div className="min-h-screen bg-app-bg py-6 px-4 font-sans">
			<div className="max-w-2xl mx-auto">
				<Send />
				<div className="my-3 flex items-center justify-between pb-2">
					<div className="flex items-center space-x-2">
						<span className="h-5 w-1 border-l-4 border-zinc-800"></span>
						<h2 className="text-lg font-bold">{total} 条评论</h2>
					</div>
					<Sort />
				</div>
				<CommentList comments={comments} isReply={false}></CommentList>
				<div ref={sentinelRef} className="mt-2">
					{renderFooter()}
				</div>
			</div>
		</div>
	);
};

export default App;
