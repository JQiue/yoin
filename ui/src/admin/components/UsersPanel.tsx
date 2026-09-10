import type { UserForAdmin } from "@/shared/api/types";
import { formatLocalDateTime } from "@/shared/helper";
import { useI18n } from "@/shared/i18n";

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
  const { t } = useI18n();
  return (
    <section className={panelClass}>
      <div className="mb-4">
        <h2 className="text-xl font-semibold">{t("admin.users.title")}</h2>
        <p className="mt-1 text-sm text-(--yo-text-muted)">
          {t("admin.users.desc")}
        </p>
      </div>

      {isLoadingUsers ? (
        <p className="text-sm text-(--yo-text-muted)">
          {t("admin.users.loading")}
        </p>
      ) : usersError ? (
        <div className="rounded-lg bg-(--yo-danger-bg) px-4 py-3 text-sm text-(--yo-danger)">
          {usersError}
        </div>
      ) : users.length === 0 ? (
        <p className="text-sm text-(--yo-text-muted)">
          {t("admin.users.empty")}
        </p>
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
                  {t("admin.users.registeredAt", {
                    time: formatLocalDateTime(user.created_at),
                  })}
                </p>
              </div>

              <div className="mt-3 grid gap-3 md:grid-cols-2">
                <div>
                  <p className="text-xs text-(--yo-text-soft)">
                    {t("admin.users.roleBindings")}
                  </p>
                  {user.role_bindings.length === 0 ? (
                    <p className="mt-2 text-sm text-(--yo-text-muted)">
                      {t("common.none")}
                    </p>
                  ) : (
                    <div className="mt-2 flex flex-wrap gap-2">
                      {user.role_bindings.map((binding) => (
                        <span
                          key={binding.id}
                          className="rounded-full border border-(--yo-surface-strong) px-2 py-0.5 text-xs text-(--yo-text-muted)"
                        >
                          {binding.role_name ??
                            t("admin.users.roleN", {
                              id: binding.role_id,
                            })}{" "}
                          · {binding.scope_type}
                          {binding.scope_id ? `:${binding.scope_id}` : ""}
                        </span>
                      ))}
                    </div>
                  )}
                </div>
                <div>
                  <p className="text-xs text-(--yo-text-soft)">
                    {t("admin.users.identities")}
                  </p>
                  {user.identities.length === 0 ? (
                    <p className="mt-2 text-sm text-(--yo-text-muted)">
                      {t("common.none")}
                    </p>
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
