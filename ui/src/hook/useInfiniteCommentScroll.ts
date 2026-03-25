import { useEffect, useRef } from "preact/compat";

interface Props {
  pageOffset: number;
  totalPages: number;
  isLoading: boolean;
  onLoadMore: () => void;
}

export const useInfiniteCommentScroll = ({
  pageOffset,
  totalPages,
  isLoading,
  onLoadMore,
}: Props) => {
  const sentinelRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (pageOffset >= totalPages || isLoading) return;

    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting) {
          onLoadMore();
        }
      },
      { threshold: 0.1 },
    );

    if (sentinelRef.current) {
      observer.observe(sentinelRef.current);
    }

    return () => observer.disconnect();
  }, [pageOffset, totalPages, isLoading, onLoadMore]);

  return sentinelRef;
};
