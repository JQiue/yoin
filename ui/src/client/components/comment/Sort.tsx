import { useCommentStore } from "@/client/store";
import { Button } from "@/shared/components/Button";

export default () => {
  const sort = useCommentStore((state) => state.sort);
  const changeSort = useCommentStore((state) => state.changeSort);
  const isLoading = useCommentStore((state) => state.isLoading);

  const tabs = [
    { label: "最新", value: "created_desc" },
    { label: "最旧", value: "created_asc" },
  ];

  return (
    <nav className="inline-flex p-1">
      {tabs.map((tab) => {
        const isActive = sort === tab.value;
        return (
          <Button
            key={tab.value}
            disabled={isLoading}
            size="sm"
            type="button"
            variant={isActive ? "secondary" : "ghost"}
            onClick={() => !isActive && changeSort(tab.value)}
            className={`relative font-bold ${isActive ? "bg-zinc-100" : "hover:bg-zinc-100 hover:text-zinc-500"} ${isLoading ? "opacity-30" : ""}`}
          >
            {tab.label}
          </Button>
        );
      })}
    </nav>
  );
};
