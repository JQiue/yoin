interface Props {
  panelClass: string;
}

export const UsersPanel = ({ panelClass }: Props) => {
  return (
    <section className={panelClass}>
      <h2 className="text-xl font-semibold">用户管理</h2>
      <p className="mt-2 text-sm text-(--yo-text-muted)">
        用户管理页先把信息架子搭好。当前后端还没有用户列表 / 角色绑定管理接口，所以这里先保留为控制台占位区。
      </p>

      <div className="mt-5 grid gap-4 md:grid-cols-2">
        <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
          <p className="text-sm font-medium">计划接入</p>
          <ul className="mt-3 space-y-2 text-sm text-(--yo-text-muted)">
            <li>用户列表与分页</li>
            <li>外部身份绑定查看</li>
            <li>角色与权限绑定</li>
            <li>站点级别授权</li>
          </ul>
        </div>
        <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
          <p className="text-sm font-medium">当前建议</p>
          <p className="mt-3 text-sm text-(--yo-text-muted)">
            先把 RBAC 管理 API 补齐，再把用户管理真正接入这个页面。这样页面结构不会推倒重来。
          </p>
        </div>
      </div>
    </section>
  );
};
