# HTTP API

## 约定

- 所有接口挂在 `/api` 下（[src/app.rs](https://github.com/JQiue/yoin/blob/dev/src/app.rs)）；
- 请求体是 JSON，`Content-Type: application/json`；
- 响应统一信封，**HTTP 状态码恒为 200**，业务结果看 `code`：

```json
{ "code": "ok", "msg": "success", "data": {} }
```

失败时：

```json
{ "code": "site_not_found", "msg": "…", "data": null }
```

`code` 为 snake_case 字符串，取值见 [src/constants/error_codes.rs](https://github.com/JQiue/yoin/blob/dev/src/constants/error_codes.rs)：`bad_request`、`unauthorized`、`forbidden`、`not_found`、`conflict`、`unsupported_media_type`、`rate_limited`、`internal_error`、`invalid_credentials`、`user_not_found`、`user_already_exists`、`site_not_found`、`comment_not_found`、`moderation_provider_not_found`、`invalid_oauth_provider`。

> 注意：业务错误在响应体里，但部分错误（例如限流提示）仍然是 `bad_request`，不要只按 `code` 猜语义。

## 认证

- 登录/注册成功后拿到 JWT，放在 `Authorization: Bearer <token>`；
- 公开接口使用可选认证：带令牌时按登录用户处理（能看到自己的私密评论、拿到 `can_delete`/`can_pin`），不带也能匿名进行；
- 管理类接口必须带令牌，并且校验对应权限。

## 游客身份

未登录访客的身份通过两种方式传递，服务端任取其一：

- 请求头 `x-yoin-guest-id`（响应头同名回传，用于前端落 `localStorage`）；
- cookie `yoin_guest_id`（HttpOnly、SameSite=Lax）。

## 公开接口

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` | `/api/health` | 健康检查 |
| `POST` | `/api/auth/register` | 注册 |
| `POST` | `/api/auth/login` | 登录 |
| `POST` | `/api/auth/external/exchange` | 外部身份交换（骨架） |
| `GET` | `/api/auth/oauth/providers` | 可用的第三方登录提供商 |
| `GET` | `/api/auth/oauth/{provider}/start` | 发起第三方登录（GitHub / QQ） |
| `GET` | `/api/auth/oauth/{provider}/callback` | 回调 |
| `GET` | `/api/sites/{id}/config` | 站点公开配置 |
| `GET` / `POST` | `/api/comments` | 评论列表 / 发表评论 |
| `GET` | `/api/comments/{id}/replies` | 回复列表 |
| `GET` / `POST` | `/api/reactions` | 反应查询 / 提交 |

## 需要登录的接口

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` / `PATCH` | `/api/users/me` | 我的资料 |
| `DELETE` | `/api/comments/{id}` | 删除评论（作者或 `comment.delete.any`） |
| `PATCH` | `/api/comments/{id}/sticky` | 置顶 / 取消置顶（需站点 `site.manage`） |
| `GET` / `POST` | `/api/comment-subscriptions` | 评论订阅列表 / 订阅 |
| `DELETE` | `/api/comment-subscriptions/{id}` | 取消订阅 |
| `POST` / `GET` / `PATCH` | `/api/sites` | 站点创建 / 列表 / 更新 |
| `GET` / `POST` | `/api/admin/oauth/providers` | OAuth 提供商 |
| `PATCH` | `/api/admin/oauth/providers/{id}` | 更新 OAuth 提供商 |
| `GET` / `POST` | `/api/admin/moderation/providers` | 审核提供商 |
| `PATCH` | `/api/admin/moderation/providers/{id}` | 更新审核提供商 |
| `GET` | `/api/admin/comments` | 评论审核列表 |
| `PATCH` | `/api/admin/comments/{id}` | 修改评论状态 |
| `GET` | `/api/admin/users` | 用户列表 |
| `GET` | `/api/admin/me/capabilities` | 我的权限快照 |
| `GET` | `/api/admin/rbac/roles`、`/permissions` | 角色 / 权限码 |
| `PATCH` | `/api/admin/rbac/roles/{id}/permissions` | 覆盖角色权限 |
| `GET` / `POST` | `/api/admin/rbac/user-role-bindings` | 用户角色绑定 |
| `DELETE` | `/api/admin/rbac/user-role-bindings/{id}` | 删除绑定 |

## 分页

列表接口返回统一分页结构：

```json
{ "items": [], "page_size": 10, "page_offset": 1, "total": 0, "total_pages": 0 }
```

## 静态资源

| 路径 | 内容 |
| --- | --- |
| `/static/client.js` | 评论组件（`window.YoinClient`） |
| `/static/admin.js` | 管理后台（`window.YoinAdmin`） |
