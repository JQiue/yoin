interface Props {
  panelClass: string;
}

export const ExternalProvidersPanel = ({ panelClass }: Props) => {
  return (
    <section className={panelClass}>
      <h2 className="text-xl font-semibold">外部身份提供者</h2>
      <p className="mt-2 text-sm text-(--yo-text-muted)">
        这个区域用来承接宿主系统登录态、外部 SSO、以及统一身份映射配置。它和
        OAuth 提供者不同，更偏“已有身份接入”而不是社交登录。目前只有
        `/api/auth/external/exchange` 骨架，还没有管理 CRUD。
      </p>

      <div className="mt-5 grid gap-4 md:grid-cols-2">
        <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
          <p className="text-sm font-medium">后续会放</p>
          <ul className="mt-3 space-y-2 text-sm text-(--yo-text-muted)">
            <li>External token exchange</li>
            <li>外部 provider 标识与元数据</li>
            <li>用户身份映射检查</li>
            <li>SSO / OIDC 接入入口</li>
          </ul>
        </div>
        <div className="rounded-lg border border-dashed border-(--yo-surface-strong) px-4 py-8 text-center text-sm text-(--yo-text-muted)">
          管理接口未提供，这里只保留入口，避免和社交 OAuth 配置混在一起。
        </div>
      </div>
    </section>
  );
};
