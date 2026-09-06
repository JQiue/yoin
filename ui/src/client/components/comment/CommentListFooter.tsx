import { useI18n } from "@/shared/i18n";

interface Props {
  isLoading: boolean;
  pageOffset: number;
  totalPages: number;
  total: number;
}

export default ({ isLoading, pageOffset, totalPages, total }: Props) => {
  const { t } = useI18n();
  const containerClass =
    "flex justify-center items-center py-8 w-full text-sm text-(--yo-text-muted)";

  if (isLoading) {
    return (
      <div className={containerClass}>
        <svg
          className="animate-spin -ml-1 mr-3 h-5 w-5 text-(--yo-text-muted)"
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
        <span>{t("client.loadingComments")}</span>
      </div>
    );
  }

  if (total === 0) {
    return (
      <div className={containerClass}>
        <div className="grow border-t border-(--yo-surface-strong)"></div>
        <span className="mx-4 text-(--yo-text-muted) select-none">
          {t("client.emptyComments")}
        </span>
        <div className="grow border-t border-(--yo-surface-strong)"></div>
      </div>
    );
  }

  if (pageOffset >= totalPages) {
    return (
      <div className={containerClass}>
        <div className="grow border-t border-(--yo-surface-strong)"></div>
        <span className="mx-4 text-(--yo-text-muted) select-none">
          {t("client.allCommentsLoaded")}
        </span>
        <div className="grow border-t border-(--yo-surface-strong)"></div>
      </div>
    );
  }

  return null;
};
