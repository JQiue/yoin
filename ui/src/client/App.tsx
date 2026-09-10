import CommentForm from "@/client/components/CommentForm";
import CommentList from "@/client/components/CommentList";
import CommentListFooter from "@/client/components/CommentListFooter";
import ReactionBar from "@/client/components/ReactionBar";
import Sort from "@/client/components/Sort";
import { useInfiniteCommentScroll } from "@/client/hooks/useInfiniteCommentScroll";
import { useInitializeCommentPage } from "@/client/hooks/useInitializeCommentPage";
import { useCommentStore } from "@/client/store";
import { useI18n } from "@/shared/i18n";

const App = () => {
  const comments = useCommentStore((state) => state.comments);
  const fetchNextPage = useCommentStore((state) => state.fetchNextPage);
  const total = useCommentStore((state) => state.total);
  const pageOffset = useCommentStore((state) => state.pageOffset);
  const totalPages = useCommentStore((state) => state.totalPages);
  const isLoading = useCommentStore((state) => state.isLoading);
  const pageReactions = useCommentStore((state) => state.pageReactions);
  const updatePageReaction = useCommentStore(
    (state) => state.updatePageReaction,
  );
  const { t } = useI18n();

  useInitializeCommentPage();
  const sentinelRef = useInfiniteCommentScroll({
    pageOffset,
    totalPages,
    isLoading,
    onLoadMore: fetchNextPage,
  });

  return (
    <div className="text-(--yo-text)">
      <ReactionBar
        summary={pageReactions}
        onSelect={updatePageReaction}
        spread
      />
      <div className="mt-3">
        <CommentForm />
      </div>
      <div className="my-3 flex items-center justify-between pb-2">
        <div className="flex items-center space-x-2">
          <span className="h-5 w-1 border-l-4 border-(--yo-text)"></span>
          <p className="text-lg font-bold">
            {t("client.commentCount", { count: total })}
          </p>
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
  );
};

export default App;
