import { useCommentStore } from "@/client/store";
import { Button } from "@/shared/components/Button";
import { useI18n } from "@/shared/i18n";

export default () => {
  const sort = useCommentStore((state) => state.sort);
  const changeSort = useCommentStore((state) => state.changeSort);
  const isLoading = useCommentStore((state) => state.isLoading);
  const { t } = useI18n();

  const tabs = [
    { label: t("client.sortNewest"), value: "created_desc" },
    { label: t("client.sortOldest"), value: "created_asc" },
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
            className={`relative font-bold ${isActive ? "bg-(--yo-surface-soft)" : "hover:bg-(--yo-surface-soft) hover:text-(--yo-text-muted)"} ${isLoading ? "opacity-30" : ""}`}
          >
            {tab.label}
          </Button>
        );
      })}
    </nav>
  );
};
