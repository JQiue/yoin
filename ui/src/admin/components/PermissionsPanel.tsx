import type {
  AdminCapabilities,
  PermissionForAdmin,
  RoleForAdmin,
  UserRoleBindingForAdmin,
} from "@/shared/api/types";
import { Button } from "@/shared/components/Button";
import { formatLocalDateTime } from "@/shared/helper";

interface Props {
  panelClass: string;
  isLoadingPermissions: boolean;
  permissionsError: string;
  capabilities: AdminCapabilities | null;
  roles: RoleForAdmin[];
  permissions: PermissionForAdmin[];
  roleBindings: UserRoleBindingForAdmin[];
}

export const PermissionsPanel = ({
  panelClass,
  isLoadingPermissions,
  permissionsError,
  capabilities,
  roles,
  permissions,
  roleBindings,
}: Props) => {
  const globalPermissions = capabilities?.global_permissions ?? [];
  const sitePermissionEntries = Object.entries(
    capabilities?.site_permissions ?? {},
  );

  return (
    <section className={panelClass}>
      <div className="mb-4 flex flex-col items-start gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-xl font-semibold">权限管理</h2>
          <p className="mt-1 text-sm text-(--yo-text-muted)">
            这里承接 RBAC
            的当前能力、角色定义和用户授权关系，先把只读信息真正接上。
          </p>
        </div>
        <Button size="sm" variant="secondary" disabled>
          编辑能力
        </Button>
      </div>

      {isLoadingPermissions ? (
        <p className="text-sm text-(--yo-text-muted)">
          正在加载权限管理数据...
        </p>
      ) : permissionsError ? (
        <div className="rounded-lg bg-(--yo-danger-bg) px-4 py-3 text-sm text-(--yo-danger)">
          {permissionsError}
        </div>
      ) : (
        <>
          <div className="grid gap-4 lg:grid-cols-[minmax(0,1.3fr)_minmax(320px,0.9fr)]">
            <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
              <div className="flex flex-col gap-1 sm:flex-row sm:items-end sm:justify-between">
                <div>
                  <p className="text-sm font-medium">我的权限快照</p>
                  <p className="mt-1 text-xs text-(--yo-text-soft)">
                    这里展示当前登录管理员自己拥有的权限，不是系统里的全部权限定义。
                  </p>
                </div>
              </div>

              <div className="mt-4 grid gap-4 xl:grid-cols-2">
                <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface) p-4">
                  <p className="text-sm font-medium">我拥有的全局权限</p>
                  {globalPermissions.length === 0 ? (
                    <p className="mt-3 text-sm text-(--yo-text-muted)">
                      当前账号没有全局权限，后面后台面板应该更多依赖站点级授权来裁剪。
                    </p>
                  ) : (
                    <div className="mt-3 flex flex-wrap gap-2">
                      {globalPermissions.map((permission) => (
                        <span
                          key={permission}
                          className="rounded-full border border-(--yo-surface-strong) px-2.5 py-1 text-xs text-(--yo-text-muted)"
                        >
                          {permission}
                        </span>
                      ))}
                    </div>
                  )}
                </div>

                <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface) p-4">
                  <p className="text-sm font-medium">我拥有的站点权限</p>
                  {sitePermissionEntries.length === 0 ? (
                    <p className="mt-3 text-sm text-(--yo-text-muted)">
                      当前账号没有站点级角色绑定。
                    </p>
                  ) : (
                    <div className="mt-3 space-y-3">
                      {sitePermissionEntries.map(
                        ([siteId, permissionNames]) => (
                          <div
                            key={siteId}
                            className="rounded-md border border-(--yo-surface-strong) bg-(--yo-surface-soft) px-3 py-3"
                          >
                            <p className="text-xs text-(--yo-text-soft)">
                              站点 #{siteId}
                            </p>
                            <div className="mt-2 flex flex-wrap gap-2">
                              {permissionNames.map((permission) => (
                                <span
                                  key={`${siteId}-${permission}`}
                                  className="rounded-full border border-(--yo-surface-strong) px-2 py-0.5 text-xs text-(--yo-text-muted)"
                                >
                                  {permission}
                                </span>
                              ))}
                            </div>
                          </div>
                        ),
                      )}
                    </div>
                  )}
                </div>
              </div>
            </div>

            <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
              <p className="text-sm font-medium">RBAC 概览</p>
              <p className="mt-1 text-xs text-(--yo-text-soft)">
                这一块看的是系统里当前有多少角色、权限和授权关系。
              </p>
              <div className="mt-4 grid gap-3 text-sm text-(--yo-text-muted)">
                <div className="rounded-md border border-(--yo-surface-strong) bg-(--yo-surface) px-3 py-3">
                  <p className="text-xs text-(--yo-text-soft)">系统角色</p>
                  <p className="mt-1 text-2xl font-semibold text-(--yo-text)">
                    {roles.length}
                  </p>
                </div>
                <div className="rounded-md border border-(--yo-surface-strong) bg-(--yo-surface) px-3 py-3">
                  <p className="text-xs text-(--yo-text-soft)">权限能力</p>
                  <p className="mt-1 text-2xl font-semibold text-(--yo-text)">
                    {permissions.length}
                  </p>
                </div>
                <div className="rounded-md border border-(--yo-surface-strong) bg-(--yo-surface) px-3 py-3">
                  <p className="text-xs text-(--yo-text-soft)">授权绑定</p>
                  <p className="mt-1 text-2xl font-semibold text-(--yo-text)">
                    {roleBindings.length}
                  </p>
                </div>
                <div className="rounded-md border border-(--yo-surface-strong) bg-(--yo-surface) px-3 py-3">
                  <p className="text-xs text-(--yo-text-soft)">站点授权覆盖</p>
                  <p className="mt-1 text-2xl font-semibold text-(--yo-text)">
                    {sitePermissionEntries.length}
                  </p>
                </div>
              </div>
            </div>
          </div>

          <div className="mt-4 grid gap-4 xl:grid-cols-[minmax(0,1.2fr)_minmax(0,1fr)]">
            <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
              <div className="flex items-center justify-between gap-3">
                <div>
                  <p className="text-sm font-medium">系统角色</p>
                  <p className="mt-1 text-xs text-(--yo-text-soft)">
                    展示角色与其当前绑定的权限能力。
                  </p>
                </div>
              </div>
              <div className="mt-3 space-y-3">
                {roles.length === 0 ? (
                  <p className="text-sm text-(--yo-text-muted)">
                    暂无角色数据。
                  </p>
                ) : (
                  roles.map((role) => (
                    <div
                      key={role.id}
                      className="rounded-md border border-(--yo-surface-strong) bg-(--yo-surface) p-3"
                    >
                      <div className="flex flex-wrap items-center gap-2">
                        <p className="font-medium">{role.name}</p>
                        <span className="rounded-full bg-(--yo-surface-soft) px-2 py-0.5 text-[11px] text-(--yo-text-muted)">
                          ID {role.id}
                        </span>
                      </div>
                      {role.description && (
                        <p className="mt-2 text-sm text-(--yo-text-muted)">
                          {role.description}
                        </p>
                      )}
                      <div className="mt-3 flex flex-wrap gap-2">
                        {role.permission_names.length === 0 ? (
                          <span className="text-xs text-(--yo-text-muted)">
                            当前未绑定权限
                          </span>
                        ) : (
                          role.permission_names.map((permission) => (
                            <span
                              key={`${role.id}-${permission}`}
                              className="rounded-full border border-(--yo-surface-strong) px-2 py-0.5 text-xs text-(--yo-text-muted)"
                            >
                              {permission}
                            </span>
                          ))
                        )}
                      </div>
                    </div>
                  ))
                )}
              </div>
            </div>

            <div className="space-y-4">
              <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
                <p className="text-sm font-medium">权限能力</p>
                <div className="mt-3 space-y-2">
                  {permissions.length === 0 ? (
                    <p className="text-sm text-(--yo-text-muted)">
                      暂无权限数据。
                    </p>
                  ) : (
                    permissions.map((permission) => (
                      <div
                        key={permission.id}
                        className="rounded-md border border-(--yo-surface-strong) bg-(--yo-surface) px-3 py-2"
                      >
                        <div className="flex flex-wrap items-center gap-2">
                          <code className="text-sm">{permission.name}</code>
                          <span className="text-[11px] text-(--yo-text-soft)">
                            ID {permission.id}
                          </span>
                        </div>
                        {permission.description && (
                          <p className="mt-1 text-sm text-(--yo-text-muted)">
                            {permission.description}
                          </p>
                        )}
                      </div>
                    ))
                  )}
                </div>
              </div>

              <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
                <p className="text-sm font-medium">用户授权绑定</p>
                <div className="mt-3 space-y-2">
                  {roleBindings.length === 0 ? (
                    <p className="text-sm text-(--yo-text-muted)">
                      暂无授权绑定。
                    </p>
                  ) : (
                    roleBindings.map((binding) => (
                      <div
                        key={binding.id}
                        className="rounded-md border border-(--yo-surface-strong) bg-(--yo-surface) px-3 py-3"
                      >
                        <div className="flex flex-wrap items-center gap-2">
                          <p className="font-medium">
                            {binding.role_name ?? `角色 #${binding.role_id}`}
                          </p>
                          <span className="rounded-full bg-(--yo-surface-soft) px-2 py-0.5 text-[11px] text-(--yo-text-muted)">
                            用户 #{binding.user_id}
                          </span>
                          <span className="rounded-full bg-(--yo-surface-soft) px-2 py-0.5 text-[11px] text-(--yo-text-muted)">
                            {binding.scope_type}
                            {binding.scope_id ? `:${binding.scope_id}` : ""}
                          </span>
                        </div>
                        <div className="mt-2 flex flex-wrap gap-x-4 gap-y-1 text-xs text-(--yo-text-muted)">
                          <span>
                            创建：{formatLocalDateTime(binding.created_at)}
                          </span>
                          <span>
                            更新：{formatLocalDateTime(binding.updated_at)}
                          </span>
                        </div>
                      </div>
                    ))
                  )}
                </div>
              </div>
            </div>
          </div>
        </>
      )}
    </section>
  );
};
