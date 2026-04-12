import { useEffect, useState } from "preact/hooks";
import type { AdminTab } from "@/admin/types";
import {
  fetchAdminCapabilities,
  fetchAdminPermissions,
  fetchAdminRoles,
  fetchAdminUserRoleBindings,
} from "@/shared/api";
import type {
  AdminCapabilities,
  PermissionForAdmin,
  RoleForAdmin,
  UserRoleBindingForAdmin,
} from "@/shared/api/types";

export const useAdminPermissions = (activeTab: AdminTab) => {
  const [capabilities, setCapabilities] = useState<AdminCapabilities | null>(
    null,
  );
  const [roles, setRoles] = useState<RoleForAdmin[]>([]);
  const [permissions, setPermissions] = useState<PermissionForAdmin[]>([]);
  const [roleBindings, setRoleBindings] = useState<UserRoleBindingForAdmin[]>(
    [],
  );
  const [isLoadingPermissions, setIsLoadingPermissions] = useState(false);
  const [permissionsError, setPermissionsError] = useState("");

  useEffect(() => {
    if (activeTab !== "permissions") return;

    let alive = true;
    setIsLoadingPermissions(true);
    setPermissionsError("");

    Promise.all([
      fetchAdminCapabilities(),
      fetchAdminRoles(),
      fetchAdminPermissions(),
      fetchAdminUserRoleBindings(),
    ])
      .then(([capabilitiesRes, rolesRes, permissionsRes, bindingsRes]) => {
        if (!alive) return;
        setCapabilities(capabilitiesRes.data);
        setRoles(rolesRes.data);
        setPermissions(permissionsRes.data);
        setRoleBindings(bindingsRes.data);
      })
      .catch((error) => {
        if (!alive) return;
        setPermissionsError(
          error instanceof Error ? error.message : "加载权限管理数据失败",
        );
      })
      .finally(() => {
        if (!alive) return;
        setIsLoadingPermissions(false);
      });

    return () => {
      alive = false;
    };
  }, [activeTab]);

  return {
    capabilities,
    roles,
    permissions,
    roleBindings,
    isLoadingPermissions,
    permissionsError,
  };
};
