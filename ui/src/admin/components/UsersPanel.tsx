import type { UserForAdmin } from "@/shared/api/types";
import { formatLocalDateTime } from "@/shared/helper";

interface Props {
  panelClass: string;
  users: UserForAdmin[];
  isLoadingUsers: boolean;
  usersError: string;
}

export const UsersPanel = ({
  panelClass,
  users,
  isLoadingUsers,
  usersError,
}: Props) => {
  return (
    <section className={panelClass}>
      <div className="mb-4">
        <h2 className="text-xl font-semibold">用户管理</h2>
        <p className="mt-1 text-sm text-(--yo-text-muted)">
          查看注册用户、外部身份映射和当前角色绑定。角色编辑在权限页。
        </p>
      </div>

      {isLoadingUsers ? (
        <p className="text-sm text-(--yo-text-muted)">正在加载用户...</p>
      ) : usersError ? (
        <div className="rounded-lg bg-(--yo-danger-bg) px-4 py-3 text-sm text-(--yo-danger)">
          {usersError}
        </div>
      ) : users.length === 0 ? (
        <p className="text-sm text-(--yo-text-muted)">暂无用户。</p>
      ) : (
        <div className="space-y-3">
          {users.map((user) => (
            <div
              key={user.id}
              className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4"
            >
              <div className="flex flex-wrap items-start justify-between gap-3">
                <div>
                  <div className="flex flex-wrap items-center gap-2">
                    <p className="font-medium">{user.nickname}</p>
                    <span className="rounded-full bg-(--yo-surface) px-2 py-0.5 text-[11px] text-(--yo-text-muted)">
                      ID {user.id}
                    </span>
                  </div>
                  <p className="mt-1 text-sm text-(--yo-text-muted)">
                    {user.email}
                  </p>
                  {user.website ? (
                    <p className="mt-1 break-all text-xs text-(--yo-text-soft)">
                      {user.website}
                    </p>
                  ) : null}
                </div>
                <p className="text-xs text-(--yo-text-soft)">
                  注册于 {formatLocalDateTime(user.created_at)}
                </p>
              </div>

              <div className="mt-3 grid gap-3 md:grid-cols-2">
                <div>
                  <p className="text-xs text-(--yo-text-soft)">角色绑定</p>
                  {user.role_bindings.length === 0 ? (
                    <p className="mt-2 text-sm text-(--yo-text-muted)">无</p>
                  ) : (
                    <div className="mt-2 flex flex-wrap gap-2">
                      {user.role_bindings.map((binding) => (
                        <span
                          key={binding.id}
                          className="rounded-full border border-(--yo-surface-strong) px-2 py-0.5 text-xs text-(--yo-text-muted)"
                        >
                          {binding.role_name ?? `角色 #${binding.role_id}`} ·{" "}
                          {binding.scope_type}
                          {binding.scope_id ? `:${binding.scope_id}` : ""}
                        </span>
                      ))}
                    </div>
                  )}
                </div>
                <div>
                  <p className="text-xs text-(--yo-text-soft)">外部身份</p>
                  {user.identities.length === 0 ? (
                    <p className="mt-2 text-sm text-(--yo-text-muted)">无</p>
                  ) : (
                    <div className="mt-2 space-y-1 text-sm text-(--yo-text-muted)">
                      {user.identities.map((identity) => (
                        <p key={identity.id}>
                          {identity.provider} / {identity.provider_user_id}
                          {identity.email ? ` · ${identity.email}` : ""}
                        </p>
                      ))}
                    </div>
                  )}
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </section>
  );
};
