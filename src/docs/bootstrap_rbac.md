# bootstrap_rbac

RBAC bootstrap utilities.

This module provides helper functions to initialise the built‑in RBAC
permissions and roles, and to ensure required role‑permission and user‑role
bindings exist.

* `bootstrap_rbac` – Creates the system permissions and roles defined in
  `SYSTEM_PERMISSIONS` and `SYSTEM_ROLES` if they are missing, then ensures
  each role has the appropriate permission associations. The operation is
  idempotent and safe to run multiple times.
* `ensure_super_admin_binding` – Guarantees that a given user has a global
  super‑admin role binding. If the binding already exists the function does
  nothing; otherwise it creates a new `UserRoleBinding` linking the user to
  the `SUPER_ADMIN` role with a global scope. Safe to call repeatedly.
* `ensure_role_permissions` – Ensures a role is associated with a list of
  permission codes. Missing `RolePermission` records are created, existing
  ones are left untouched. Used during bootstrap to set up default
  role‑permission relationships.

All functions return `Result<(), AppError>` and propagate repository
errors.
