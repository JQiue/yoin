import { useEffect, useState } from "preact/hooks";
import type { AdminTab } from "@/admin/types";
import { fetchAdminUsers } from "@/shared/api";
import type { UserForAdmin } from "@/shared/api/types";
import { t } from "@/shared/i18n";

export const useAdminUsers = (activeTab: AdminTab) => {
  const [users, setUsers] = useState<UserForAdmin[]>([]);
  const [isLoadingUsers, setIsLoadingUsers] = useState(false);
  const [usersError, setUsersError] = useState("");

  useEffect(() => {
    if (activeTab !== "users" && activeTab !== "permissions") return;

    let alive = true;
    setIsLoadingUsers(true);
    setUsersError("");
    fetchAdminUsers()
      .then((res) => {
        if (!alive) return;
        setUsers(res.data);
      })
      .catch((error) => {
        if (!alive) return;
        setUsersError(
          error instanceof Error ? error.message : t("admin.users.loadFailed"),
        );
      })
      .finally(() => {
        if (!alive) return;
        setIsLoadingUsers(false);
      });

    return () => {
      alive = false;
    };
  }, [activeTab]);

  return { users, isLoadingUsers, usersError };
};
