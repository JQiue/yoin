import type {
  AdminCapabilities,
  PermissionForAdmin,
  RoleForAdmin,
  Site,
  UserForAdmin,
  UserRoleBindingForAdmin,
} from "@/shared/api/types";
import { Button } from "@/shared/components/Button";
import { formatLocalDateTime } from "@/shared/helper";
import { useI18n } from "@/shared/i18n";

interface BindingFormState {
  userId: string;
  roleId: string;
  scopeType: "global" | "site";
  scopeId: string;
}

interface Props {
  panelClass: string;
  isLoadingPermissions: boolean;
  permissionsError: string;
  capabilities: AdminCapabilities | null;
  roles: RoleForAdmin[];
  permissions: PermissionForAdmin[];
  roleBindings: UserRoleBindingForAdmin[];
  users: UserForAdmin[];
  sites: Site[];
  savingRoleId: number | null;
  deletingBindingId: number | null;
  isCreatingBinding: boolean;
  bindingForm: BindingFormState;
  bindingError: string;
  onToggleRolePermission: (role: RoleForAdmin, permissionName: string) => void;
  onChangeBindingForm: (patch: Partial<BindingFormState>) => void;
  onCreateBinding: () => void;
  onDeleteBinding: (id: number) => void;
}

export const PermissionsPanel = ({
  panelClass,
  isLoadingPermissions,
  permissionsError,
  capabilities,
  roles,
  permissions,
  roleBindings,
  users,
  sites,
  savingRoleId,
  deletingBindingId,
  isCreatingBinding,
  bindingForm,
  bindingError,
  onToggleRolePermission,
  onChangeBindingForm,
  onCreateBinding,
  onDeleteBinding,
}: Props) => {
  const { t } = useI18n();
  const globalPermissions = capabilities?.global_permissions ?? [];
  const sitePermissionEntries = Object.entries(
    capabilities?.site_permissions ?? {},
  );

  return (
    <section className={panelClass}>
      <div className="mb-4 flex flex-col items-start gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-xl font-semibold">
            {t("admin.permissions.title")}
          </h2>
          <p className="mt-1 text-sm text-(--yo-text-muted)">
            {t("admin.permissions.desc")}
          </p>
        </div>
      </div>

      {isLoadingPermissions ? (
        <p className="text-sm text-(--yo-text-muted)">
          {t("admin.permissions.loading")}
        </p>
      ) : permissionsError ? (
        <div className="rounded-lg bg-(--yo-danger-bg) px-4 py-3 text-sm text-(--yo-danger)">
          {permissionsError}
        </div>
      ) : (
        <>
          <div className="grid gap-4 lg:grid-cols-[minmax(0,1.3fr)_minmax(320px,0.9fr)]">
            <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
              <p className="text-sm font-medium">
                {t("admin.permissions.snapshot")}
              </p>
              <p className="mt-1 text-xs text-(--yo-text-soft)">
                {t("admin.permissions.snapshotHint")}
              </p>
              <div className="mt-4 grid gap-4 xl:grid-cols-2">
                <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface) p-4">
                  <p className="text-sm font-medium">
                    {t("admin.permissions.myGlobal")}
                  </p>
                  {globalPermissions.length === 0 ? (
                    <p className="mt-3 text-sm text-(--yo-text-muted)">
                      {t("admin.permissions.noGlobal")}
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
                  <p className="text-sm font-medium">
                    {t("admin.permissions.mySite")}
                  </p>
                  {sitePermissionEntries.length === 0 ? (
                    <p className="mt-3 text-sm text-(--yo-text-muted)">
                      {t("admin.permissions.noSite")}
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
                              {t("admin.permissions.siteN", { id: siteId })}
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
              <p className="text-sm font-medium">
                {t("admin.permissions.overview")}
              </p>
              <div className="mt-4 grid gap-3 text-sm text-(--yo-text-muted)">
                <div className="rounded-md border border-(--yo-surface-strong) bg-(--yo-surface) px-3 py-3">
                  <p className="text-xs text-(--yo-text-soft)">
                    {t("admin.permissions.systemRoles")}
                  </p>
                  <p className="mt-1 text-2xl font-semibold text-(--yo-text)">
                    {roles.length}
                  </p>
                </div>
                <div className="rounded-md border border-(--yo-surface-strong) bg-(--yo-surface) px-3 py-3">
                  <p className="text-xs text-(--yo-text-soft)">
                    {t("admin.permissions.capabilities")}
                  </p>
                  <p className="mt-1 text-2xl font-semibold text-(--yo-text)">
                    {permissions.length}
                  </p>
                </div>
                <div className="rounded-md border border-(--yo-surface-strong) bg-(--yo-surface) px-3 py-3">
                  <p className="text-xs text-(--yo-text-soft)">
                    {t("admin.permissions.bindings")}
                  </p>
                  <p className="mt-1 text-2xl font-semibold text-(--yo-text)">
                    {roleBindings.length}
                  </p>
                </div>
              </div>
            </div>
          </div>

          <div className="mt-4 grid gap-4 xl:grid-cols-[minmax(0,1.2fr)_minmax(0,1fr)]">
            <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
              <p className="text-sm font-medium">
                {t("admin.permissions.systemRoles")}
              </p>
              <p className="mt-1 text-xs text-(--yo-text-soft)">
                {t("admin.permissions.rolesHint")}
              </p>
              <div className="mt-3 space-y-3">
                {roles.map((role) => (
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
                    {role.description ? (
                      <p className="mt-2 text-sm text-(--yo-text-muted)">
                        {role.description}
                      </p>
                    ) : null}
                    <div className="mt-3 flex flex-wrap gap-2">
                      {permissions.map((permission) => {
                        const checked = role.permission_names.includes(
                          permission.name,
                        );
                        return (
                          <label
                            key={`${role.id}-${permission.id}`}
                            className="inline-flex items-center gap-1 rounded-full border border-(--yo-surface-strong) px-2 py-0.5 text-xs text-(--yo-text-muted)"
                          >
                            <input
                              type="checkbox"
                              checked={checked}
                              disabled={savingRoleId === role.id}
                              onChange={() =>
                                onToggleRolePermission(role, permission.name)
                              }
                            />
                            {permission.name}
                          </label>
                        );
                      })}
                    </div>
                  </div>
                ))}
              </div>
            </div>

            <div className="space-y-4">
              <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
                <p className="text-sm font-medium">
                  {t("admin.permissions.capabilities")}
                </p>
                <div className="mt-3 space-y-2">
                  {permissions.map((permission) => (
                    <div
                      key={permission.id}
                      className="rounded-md border border-(--yo-surface-strong) bg-(--yo-surface) px-3 py-2"
                    >
                      <code className="text-sm">{permission.name}</code>
                      {permission.description ? (
                        <p className="mt-1 text-sm text-(--yo-text-muted)">
                          {permission.description}
                        </p>
                      ) : null}
                    </div>
                  ))}
                </div>
              </div>

              <div className="rounded-lg border border-(--yo-surface-strong) bg-(--yo-surface-soft) p-4">
                <p className="text-sm font-medium">
                  {t("admin.permissions.userBindings")}
                </p>
                <div className="mt-3 grid gap-3 md:grid-cols-2">
                  <label className="block text-xs text-(--yo-text-soft)">
                    {t("common.user")}
                    <select
                      value={bindingForm.userId}
                      onChange={(event) =>
                        onChangeBindingForm({
                          userId: event.currentTarget.value,
                        })
                      }
                      className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm"
                    >
                      <option value="">
                        {t("admin.permissions.selectUser")}
                      </option>
                      {users.map((user) => (
                        <option key={user.id} value={user.id}>
                          {user.nickname} ({user.email})
                        </option>
                      ))}
                    </select>
                  </label>
                  <label className="block text-xs text-(--yo-text-soft)">
                    {t("common.role")}
                    <select
                      value={bindingForm.roleId}
                      onChange={(event) =>
                        onChangeBindingForm({
                          roleId: event.currentTarget.value,
                        })
                      }
                      className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm"
                    >
                      <option value="">
                        {t("admin.permissions.selectRole")}
                      </option>
                      {roles.map((role) => (
                        <option key={role.id} value={role.id}>
                          {role.name}
                        </option>
                      ))}
                    </select>
                  </label>
                  <label className="block text-xs text-(--yo-text-soft)">
                    {t("admin.permissions.scope")}
                    <select
                      value={bindingForm.scopeType}
                      onChange={(event) =>
                        onChangeBindingForm({
                          scopeType: event.currentTarget.value as
                            | "global"
                            | "site",
                        })
                      }
                      className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm"
                    >
                      <option value="global">{t("common.global")}</option>
                      <option value="site">{t("common.site")}</option>
                    </select>
                  </label>
                  {bindingForm.scopeType === "site" ? (
                    <label className="block text-xs text-(--yo-text-soft)">
                      {t("common.site")}
                      <select
                        value={bindingForm.scopeId}
                        onChange={(event) =>
                          onChangeBindingForm({
                            scopeId: event.currentTarget.value,
                          })
                        }
                        className="mt-1 w-full rounded-md border border-(--yo-surface-strong) bg-(--yo-bg) px-3 py-2 text-sm"
                      >
                        <option value="">
                          {t("admin.permissions.selectSite")}
                        </option>
                        {sites.map((site) => (
                          <option key={site.id} value={site.id}>
                            {site.name}
                          </option>
                        ))}
                      </select>
                    </label>
                  ) : null}
                </div>
                {bindingError ? (
                  <p className="mt-3 text-sm text-(--yo-danger)">
                    {bindingError}
                  </p>
                ) : null}
                <div className="mt-3">
                  <Button
                    size="sm"
                    loading={isCreatingBinding}
                    onClick={onCreateBinding}
                  >
                    {t("admin.permissions.addBinding")}
                  </Button>
                </div>
                <div className="mt-3 space-y-2">
                  {roleBindings.length === 0 ? (
                    <p className="text-sm text-(--yo-text-muted)">
                      {t("admin.permissions.noBindings")}
                    </p>
                  ) : (
                    roleBindings.map((binding) => (
                      <div
                        key={binding.id}
                        className="rounded-md border border-(--yo-surface-strong) bg-(--yo-surface) px-3 py-3"
                      >
                        <div className="flex flex-wrap items-center justify-between gap-2">
                          <div className="flex flex-wrap items-center gap-2">
                            <p className="font-medium">
                              {binding.role_name ??
                                t("admin.users.roleN", { id: binding.role_id })}
                            </p>
                            <span className="rounded-full bg-(--yo-surface-soft) px-2 py-0.5 text-[11px] text-(--yo-text-muted)">
                              {t("admin.permissions.userN", {
                                id: binding.user_id,
                              })}
                            </span>
                            <span className="rounded-full bg-(--yo-surface-soft) px-2 py-0.5 text-[11px] text-(--yo-text-muted)">
                              {binding.scope_type}
                              {binding.scope_id ? `:${binding.scope_id}` : ""}
                            </span>
                          </div>
                          <Button
                            size="sm"
                            variant="ghost"
                            loading={deletingBindingId === binding.id}
                            onClick={() => onDeleteBinding(binding.id)}
                          >
                            {t("common.delete")}
                          </Button>
                        </div>
                        <div className="mt-2 flex flex-wrap gap-x-4 gap-y-1 text-xs text-(--yo-text-muted)">
                          <span>
                            {t("admin.permissions.created", {
                              time: formatLocalDateTime(binding.created_at),
                            })}
                          </span>
                          <span>
                            {t("admin.permissions.updated", {
                              time: formatLocalDateTime(binding.updated_at),
                            })}
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
