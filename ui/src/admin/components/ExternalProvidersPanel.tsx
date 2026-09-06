import { useI18n } from "@/shared/i18n";

interface Props {
  panelClass: string;
}

export const ExternalProvidersPanel = ({ panelClass }: Props) => {
  const { t } = useI18n();
  return (
    <section className={panelClass}>
      <h2 className="text-xl font-semibold">{t("admin.external.title")}</h2>
      <p className="mt-2 text-sm text-(--yo-text-muted)">
        {t("admin.external.desc")}
      </p>

      <div className="mt-5 grid gap-4 md:grid-cols-2">
        <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
          <p className="text-sm font-medium">{t("admin.external.later")}</p>
          <ul className="mt-3 space-y-2 text-sm text-(--yo-text-muted)">
            <li>{t("admin.external.itemExchange")}</li>
            <li>{t("admin.external.itemMeta")}</li>
            <li>{t("admin.external.itemMapping")}</li>
            <li>{t("admin.external.itemSso")}</li>
          </ul>
        </div>
        <div className="rounded-lg border border-dashed border-(--yo-surface-strong) px-4 py-8 text-center text-sm text-(--yo-text-muted)">
          {t("admin.external.noApi")}
        </div>
      </div>
    </section>
  );
};
