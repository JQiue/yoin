# 权限模型

后台访问控制基于 RBAC：**权限码 + 角色 + 用户绑定**，绑定分全局与站点两种作用域。

## 权限码

定义在 `src/constants/mod.rs` 的 `rbac::codes`：

| 权限码 | 说明 |
| --- | --- |
| `site.manage` | 管理站点（读写站点设置、置顶评论） |
| `comment.moderate` | 审核评论（改状态） |
| `comment.delete.any` | 删除任意评论 |
| `oauth_provider.manage` | 管理第三方登录提供商 |
| `moderation_provider.manage` | 管理审核提供商 |

## 系统角色

`src/constants/mod.rs` 的 `rbac::roles` 与权限码映射：

| 角色 | 权限 |
| --- | --- |
| `super_admin` | 全部 5 个 |
| `site_admin` | `site.manage`、`oauth_provider.manage`、`moderation_provider.manage` |
| `moderator` | `comment.moderate`、`comment.delete.any` |

## 绑定与判定

- 绑定的 `scope_type` 是 `global` 或 `site`；站点作用域的 `scope_id` 存站点 id；
- 判断「某人能不能在某站点做某事」时，全局权限与站点级权限是或的关系；
- `GET /api/admin/me/capabilities` 返回当前登录者的快照：

```json
{ "global_permissions": ["comment.moderate"], "site_permissions": { "1": ["site.manage"] } }
```

## 引导

启动时幂等引导（`src/rbac/`）：缺少的系统权限、系统角色、角色-权限关联会被补上，已存在的不动。第一个注册的用户由 `ensure_super_admin_binding` 授予全局 `super_admin`，并创建默认站点。**最后一个全局 `super_admin` 不能被降级或删除绑定。**

## 相关接口

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` | `/api/admin/me/capabilities` | 我的权限快照 |
| `GET` | `/api/admin/users` | 用户列表 |
| `GET` | `/api/admin/rbac/roles` | 角色列表（含权限码） |
| `GET` | `/api/admin/rbac/permissions` | 权限码列表 |
| `PATCH` | `/api/admin/rbac/roles/{id}/permissions` | 覆盖某个角色的权限集合 |
| `GET` / `POST` | `/api/admin/rbac/user-role-bindings` | 列出 / 新增用户角色绑定 |
| `DELETE` | `/api/admin/rbac/user-role-bindings/{id}` | 删除绑定 |

## 评论上的权限投影

列表接口按查看者裁剪数据：

- `can_delete`：当前查看者是不是评论作者；
- `can_pin`：查看者是否持有该站点的 `site.manage`，且评论是顶层评论；
- 私密评论：作者本人（登录 id 或 guest id 相同）或该站点管理权限持有者可见；
- 匿名评论：无站点管理权限的查看者拿不到真实身份字段。
