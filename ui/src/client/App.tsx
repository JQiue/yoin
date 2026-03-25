import CommentForm from "@/features/comment/CommentForm";
import CommentList from "@/features/comment/CommentList";
import CommentListFooter from "@/features/comment/CommentListFooter";
import Sort from "@/features/comment/Sort";
import { useInfiniteCommentScroll } from "@/hook/useInfiniteCommentScroll";
import { useInitializeCommentPage } from "@/hook/useInitializeCommentPage";
import { useCommentStore } from "@/store";

const App = () => {
	const comments = useCommentStore((state) => state.comments);
	const fetchNextPage = useCommentStore((state) => state.fetchNextPage);
	const total = useCommentStore((state) => state.total);
	const pageOffset = useCommentStore((state) => state.pageOffset);
	const totalPages = useCommentStore((state) => state.totalPages);
	const isLoading = useCommentStore((state) => state.isLoading);

	useInitializeCommentPage();
	const sentinelRef = useInfiniteCommentScroll({
		pageOffset,
		totalPages,
		isLoading,
		onLoadMore: fetchNextPage,
	});

	return (
		<div className="min-h-screen py-6 px-4 bg-app-bg ">
			<div className="max-w-2xl mx-auto">
				<CommentForm />
				<div className="my-3 flex items-center justify-between pb-2">
					<div className="flex items-center space-x-2">
						<span className="h-5 w-1 border-l-4 border-zinc-800"></span>
						<h2 className="text-lg font-bold">{total} 评论数</h2>
					</div>
					<Sort />
				</div>
				<CommentList comments={comments}></CommentList>
				<div ref={sentinelRef} className="mt-2">
					<CommentListFooter
						isLoading={isLoading}
						pageOffset={pageOffset}
						totalPages={totalPages}
						total={total}
					/>
				</div>
			</div>
		</div>
	);
};

export default App;
