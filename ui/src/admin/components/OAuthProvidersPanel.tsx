import { Button } from "@/shared/components/Button";

interface Props {
  panelClass: string;
}

export const OAuthProvidersPanel = ({ panelClass }: Props) => {
  return (
    <section className={panelClass}>
      <div className="mb-4 flex flex-col items-start gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-xl font-semibold">OAuth 提供者</h2>
          <p className="mt-1 text-sm text-(--yo-text-muted)">
            这里放社交登录与标准 OAuth 配置。后面可以继续接 GitHub、Google、QQ 等提供者的创建与启用流程。
          </p>
        </div>
        <Button size="sm">新增提供者</Button>
      </div>

      <div className="grid gap-4 md:grid-cols-2">
        <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
          <p className="text-sm font-medium">准备接入</p>
          <ul className="mt-3 space-y-2 text-sm text-(--yo-text-muted)">
            <li>GitHub / Google / QQ OAuth</li>
            <li>客户端 ID / Secret 管理</li>
            <li>回调地址校验</li>
            <li>启用 / 禁用状态切换</li>
          </ul>
        </div>
        <div className="rounded-lg border border-dashed border-(--yo-surface-strong) px-4 py-8 text-center text-sm text-(--yo-text-muted)">
          当前后端只有创建接口骨架，这里先把管理位置和后续信息结构预留出来。
        </div>
      </div>
    </section>
  );
};
