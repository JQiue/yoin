import { useEffect, useState } from "preact/hooks";
import type { AdminTab } from "@/admin/types";
import {
  createAdminUserRoleBinding,
  deleteAdminUserRoleBinding,
  fetchAdminCapabilities,
  fetchAdminPermissions,
  fetchAdminRoles,
  fetchAdminUserRoleBindings,
  replaceAdminRolePermissions,
} from "@/shared/api";
import type {
  AdminCapabilities,
  PermissionForAdmin,
  RoleForAdmin,
  UserRoleBindingForAdmin,
} from "@/shared/api/types";
import { t } from "@/shared/i18n";

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
  const [savingRoleId, setSavingRoleId] = useState<number | null>(null);
  const [deletingBindingId, setDeletingBindingId] = useState<number | null>(
    null,
  );
  const [isCreatingBinding, setIsCreatingBinding] = useState(false);
  const [bindingForm, setBindingForm] = useState({
    userId: "",
    roleId: "",
    scopeType: "global" as "global" | "site",
    scopeId: "",
  });
  const [bindingError, setBindingError] = useState("");

  const reload = async () => {
    const [capabilitiesRes, rolesRes, permissionsRes, bindingsRes] =
      await Promise.all([
        fetchAdminCapabilities(),
        fetchAdminRoles(),
        fetchAdminPermissions(),
        fetchAdminUserRoleBindings(),
      ]);
    setCapabilities(capabilitiesRes.data);
    setRoles(rolesRes.data);
    setPermissions(permissionsRes.data);
    setRoleBindings(bindingsRes.data);
  };

  useEffect(() => {
    if (activeTab !== "permissions") return;

    let alive = true;
    setIsLoadingPermissions(true);
    setPermissionsError("");

    reload()
      .catch((error) => {
        if (!alive) return;
        setPermissionsError(
          error instanceof Error
            ? error.message
            : t("admin.permissions.loadFailed"),
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

  const saveRolePermissions = async (
    roleId: number,
    permissionNames: string[],
  ) => {
    setSavingRoleId(roleId);
    setPermissionsError("");
    try {
      const res = await replaceAdminRolePermissions(roleId, permissionNames);
      setRoles((current) =>
        current.map((role) => (role.id === roleId ? res.data : role)),
      );
    } catch (error) {
      setPermissionsError(
        error instanceof Error
          ? error.message
          : t("admin.permissions.saveFailed"),
      );
    } finally {
      setSavingRoleId(null);
    }
  };

  const createBinding = async () => {
    const userId = Number(bindingForm.userId);
    const roleId = Number(bindingForm.roleId);
    if (!Number.isInteger(userId) || userId <= 0) {
      setBindingError(t("admin.permissions.needUser"));
      return;
    }
    if (!Number.isInteger(roleId) || roleId <= 0) {
      setBindingError(t("admin.permissions.needRole"));
      return;
    }
    if (bindingForm.scopeType === "site" && !bindingForm.scopeId) {
      setBindingError(t("admin.permissions.needSite"));
      return;
    }

    setIsCreatingBinding(true);
    setBindingError("");
    try {
      const res = await createAdminUserRoleBinding({
        user_id: userId,
        role_id: roleId,
        scope_type: bindingForm.scopeType,
        scope_id: bindingForm.scopeType === "site" ? bindingForm.scopeId : null,
      });
      setRoleBindings((current) => [res.data, ...current]);
      setBindingForm({
        userId: "",
        roleId: "",
        scopeType: "global",
        scopeId: "",
      });
    } catch (error) {
      setBindingError(
        error instanceof Error
          ? error.message
          : t("admin.permissions.createFailed"),
      );
    } finally {
      setIsCreatingBinding(false);
    }
  };

  const deleteBinding = async (id: number) => {
    setDeletingBindingId(id);
    setBindingError("");
    try {
      await deleteAdminUserRoleBinding(id);
      setRoleBindings((current) => current.filter((item) => item.id !== id));
    } catch (error) {
      setBindingError(
        error instanceof Error
          ? error.message
          : t("admin.permissions.deleteFailed"),
      );
    } finally {
      setDeletingBindingId(null);
    }
  };

  return {
    capabilities,
    roles,
    permissions,
    roleBindings,
    isLoadingPermissions,
    permissionsError,
    savingRoleId,
    deletingBindingId,
    isCreatingBinding,
    bindingForm,
    bindingError,
    setBindingForm,
    saveRolePermissions,
    createBinding,
    deleteBinding,
  };
};
