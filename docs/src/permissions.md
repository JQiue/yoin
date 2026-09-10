# 权限与角色

后台的访问控制基于 RBAC：权限码 + 角色 + 用户绑定，绑定分**全局**和**站点**两种作用域。

## 权限码

定义在 [src/constants/mod.rs](https://github.com/JQiue/yoin/blob/dev/src/constants/mod.rs)：

| 权限码 | 说明 |
| --- | --- |
| `site.manage` | 管理站点（读写站点配置、置顶评论） |
| `comment.moderate` | 审核评论（改状态） |
| `comment.delete.any` | 删除任意评论 |
| `oauth_provider.manage` | 管理 OAuth 提供商 |
| `moderation_provider.manage` | 管理审核提供商 |

## 系统角色

| 角色 | 权限 |
| --- | --- |
| `super_admin` | 全部 5 个权限 |
| `site_admin` | `site.manage`、`oauth_provider.manage`、`moderation_provider.manage` |
| `moderator` | `comment.moderate`、`comment.delete.any` |

## 引导

第一个注册的用户会被自动授予**全局** `super_admin`，并创建一个默认站点。角色与权限在启动时幂等引导（见 [src/docs/bootstrap_rbac.md](https://github.com/JQiue/yoin/blob/dev/src/docs/bootstrap_rbac.md)），重复启动不会重复创建。

## 绑定与判定

- 绑定 `global` 作用域 → 权限对所有站点生效；绑定 `site` 作用域 → 只对指定站点生效。
- 站点级判断（例如「能不能置顶这条评论」）会同时看全局权限和该站点的站点级权限。
- `GET /api/admin/me/capabilities` 返回当前登录者的权限快照：`global_permissions` 与按站点分组的 `site_permissions`。

## 管理接口

均需要认证；读写角色/绑定需要相应权限：

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` | `/api/admin/me/capabilities` | 我的权限快照 |
| `GET` | `/api/admin/users` | 用户列表 |
| `GET` | `/api/admin/rbac/roles` | 角色列表（含权限码） |
| `GET` | `/api/admin/rbac/permissions` | 权限码列表 |
| `PATCH` | `/api/admin/rbac/roles/{id}/permissions` | 覆盖某个角色的权限集合 |
| `GET` / `POST` | `/api/admin/rbac/user-role-bindings` | 列出 / 新增用户角色绑定 |
| `DELETE` | `/api/admin/rbac/user-role-bindings/{id}` | 删除绑定 |

最后一个全局 `super_admin` 受保护，不能被降级或删除绑定。
