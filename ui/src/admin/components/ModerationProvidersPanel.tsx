import { Button } from "@/shared/components/Button";

interface Props {
  panelClass: string;
}

export const ModerationProvidersPanel = ({ panelClass }: Props) => {
  return (
    <section className={panelClass}>
      <div className="mb-4 flex flex-col items-start gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-xl font-semibold">审核提供者</h2>
          <p className="mt-1 text-sm text-(--yo-text-muted)">
            这里管理评论审核来源，比如 LLM、Akismet，以及后续可能接入的自定义审核器。
          </p>
        </div>
        <Button size="sm">新增审核器</Button>
      </div>

      <div className="grid gap-4 lg:grid-cols-2">
        <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
          <p className="text-sm font-medium">当前规划</p>
          <ul className="mt-3 space-y-2 text-sm text-(--yo-text-muted)">
            <li>LLM 审核配置</li>
            <li>Akismet 审核配置</li>
            <li>按站点启用 / 禁用</li>
            <li>Prompt / 模型 / API Base 管理</li>
          </ul>
        </div>
        <div className="rounded-lg border border-dashed border-(--yo-surface-strong) px-4 py-8 text-center text-sm text-(--yo-text-muted)">
          后端管理 API 已经有基础骨架，这里后面可以优先接成第一批真正可用的后台配置页。
        </div>
      </div>
    </section>
  );
};
