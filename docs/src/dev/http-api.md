# HTTP API

## 约定

- 所有接口挂在 `/api` 下（路由定义见 `src/app.rs`）；
- 请求体是 JSON，`Content-Type: application/json`；
- 响应统一信封，成功时 HTTP 200：

```json
{ "code": "ok", "msg": "success", "data": {} }
```

- 失败时**按错误类型返回真实 HTTP 状态码**，响应体仍是同一个信封，`data` 为 `null`：

```json
{ "code": "site_not_found", "msg": "site not found", "data": null }
```

| `code` | HTTP | 语义 |
| --- | --- | --- |
| `bad_request` | 400 | 参数不合法，含「发太快」「评论过长」等业务拒绝 |
| `unauthorized` | 401 | 未登录或令牌无效 |
| `invalid_credentials` | 401 | 账号或密码错误 |
| `forbidden` | 403 | 已登录但没权限（含匿名/私密被站点关闭） |
| `not_found` / `user_not_found` / `site_not_found` / `comment_not_found` / `moderation_provider_not_found` | 404 | 目标不存在；不可见的私密评论也返回这一类，不泄露存在性 |
| `conflict` / `user_already_exists` | 409 | 唯一性冲突 |
| `unsupported_media_type` | 415 | 载荷类型不对 |
| `rate_limited` | 429 | 预留状态码（当前评论限流走的是 `bad_request`） |
| `internal_error` | 500 | 服务端错误；响应体只给通用文案，细节只进日志 |

## 认证与身份

- 登录/注册成功后拿到 JWT，放在 `Authorization: Bearer <token>`；
- 公开接口使用可选认证：带令牌时按登录用户处理（能看到自己的私密评论、拿到 `can_delete` / `can_pin`），不带也能匿名访问；
- 管理类接口必须带令牌并校验对应权限；
- **游客身份**：请求头 `x-yoin-guest-id`（响应同名回传，前端存 `localStorage`）或 cookie `yoin_guest_id`（HttpOnly、SameSite=Lax），服务端任取其一；缺失或非法时生成新的写回。

## 公开接口

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` | `/api/health` | 健康检查 |
| `POST` | `/api/auth/register` | 注册 |
| `POST` | `/api/auth/login` | 登录 |
| `POST` | `/api/auth/external/exchange` | 外部身份交换（骨架，未实现） |
| `GET` | `/api/auth/oauth/providers` | 已启用的全局第三方登录提供商 |
| `GET` | `/api/auth/oauth/{provider}/start` | 取授权地址 |
| `GET` | `/api/auth/oauth/{provider}/callback` | 第三方回调（返回 HTML） |
| `GET` | `/api/sites/{id}/config` | 站点公开配置 |
| `GET` / `POST` | `/api/comments` | 评论列表 / 发表评论 |
| `GET` | `/api/comments/{id}/replies` | 回复列表 |
| `GET` / `POST` | `/api/reactions` | 反应查询 / 提交 |

## 需要登录的接口

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` / `PATCH` | `/api/users/me` | 我的资料 |
| `DELETE` | `/api/comments/{id}` | 删除评论（作者本人或持有删除任意评论的权限） |
| `PATCH` | `/api/comments/{id}/sticky` | 置顶 / 取消置顶（需该站点管理权限，仅顶层评论） |
| `GET` / `POST` | `/api/comment-subscriptions` | 评论订阅列表 / 订阅 |
| `DELETE` | `/api/comment-subscriptions/{id}` | 取消订阅 |
| `POST` / `GET` / `PATCH` | `/api/sites` | 站点创建 / 列表 / 更新 |
| `GET` / `POST` | `/api/admin/oauth/providers` | 第三方登录提供商列表 / 创建 |
| `PATCH` | `/api/admin/oauth/providers/{id}` | 更新提供商 |
| `GET` / `POST` | `/api/admin/moderation/providers` | 审核提供商列表 / 创建 |
| `PATCH` | `/api/admin/moderation/providers/{id}` | 更新审核提供商 |
| `GET` | `/api/admin/comments` | 审核用评论列表（可按站点、页面、状态筛选） |
| `PATCH` | `/api/admin/comments/{id}` | 修改评论状态 |
| `GET` | `/api/admin/users` | 用户列表 |
| `GET` | `/api/admin/me/capabilities` | 我的权限快照 |
| `GET` | `/api/admin/rbac/roles`、`/api/admin/rbac/permissions` | 角色 / 权限码 |
| `PATCH` | `/api/admin/rbac/roles/{id}/permissions` | 覆盖某角色的权限集合 |
| `GET` / `POST` | `/api/admin/rbac/user-role-bindings` | 用户角色绑定 |
| `DELETE` | `/api/admin/rbac/user-role-bindings/{id}` | 删除绑定 |

## 分页

列表接口返回统一结构：

```json
{ "items": [], "page_size": 10, "page_offset": 1, "total": 0, "total_pages": 0 }
```

## 站点配置

`GET /api/sites/{id}/config` 与站点读写的载荷都用这几个字段（默认值见 `src/entity/sites.rs`）：

| 字段 | 类型 | 默认 | 说明 |
| --- | --- | --- | --- |
| `allow_anonymous` | bool | `true` | 是否允许匿名评论 |
| `allow_private` | bool | `true` | 是否允许私密评论 |
| `max_comment_length` | number | `1024` | 正文字符数上限（按 Unicode 字符计） |
| `comment_limit_seconds` | number | `60` | 同一身份的发帖间隔窗口 |
| `allowed_reactions` | string[] | `👍 ❤️ 😄 🎉 👎` | 该站点允许的反应，必须是全局集合的子集 |

发帖时的约束在 handler 层执行（仅对非管理员）：

- 匿名/私密被站点关闭时 → `forbidden`；
- 限流键是 `(site_id, 用户 id 或来源 IP)`，窗口为 `comment_limit_seconds`，触发时返回 `bad_request`，文案 `The comment is too fast.`；
- 正文长度按 `chars().count()` 计，超出返回 `bad_request`，文案 `comment too long`。

## 第三方登录接口

流程：

1. `GET /api/auth/oauth/providers` → 前端渲染登录按钮（**只返回已启用且作用域为全局的**提供商）；
2. `GET /api/auth/oauth/{provider}/start` → 返回 `{ auth_url }`，前端在弹窗里打开；
3. 用户在第三方平台授权后跳回 `GET /api/auth/oauth/{provider}/callback?code=...&state=...`；
4. 服务端校验 `state`（签名 JWT，含 `provider` 声明，**有效期 180 秒**，provider 不匹配即拒绝），用 `code` 换令牌、拉取用户资料、登录或注册本地用户，最后返回一个 HTML 页面，通过 `window.opener.postMessage({ type: "yoin-oauth", payload })` 把令牌交回开窗页面并自动关闭。

`provider` 目前支持 `github` 与 `qq`，未知取值返回 `invalid_oauth_provider`。

提供商配置字段（`/api/admin/oauth/providers`）：

| 字段 | 说明 |
| --- | --- |
| `site_id` | 作用域；`null` = 全局。**登录流程只查全局配置**（`find_enabled_public` / `find_enabled_by_site_and_code(None, ..)`），站点级配置目前不参与登录 |
| `provider_code` | `github` 或 `qq`；同一作用域内唯一 |
| `client_id` / `client_secret` | 第三方平台分配；读取接口不回显 `client_secret` |
| `redirect_uri` | 必须与第三方平台登记的完全一致，授权与换令牌时都会带上 |
| `enabled` | 是否启用，创建时默认 `true` |

用户映射：`(provider, provider_user_id)` 记在 `user_identities` 里；已存在则直接登录，否则邮箱与现有账号相同时关联到该账号，都不满足则新建账号（随机密码）。第一个用户同样会被授予超级管理员并创建默认站点。GitHub 目前不申请邮箱权限，取不到邮箱时账号使用 `<provider>+<id>@oauth.yoin.local` 占位地址。

## 静态资源

| 路径 | 内容 |
| --- | --- |
| `/static/client.js` | 评论组件（`window.YoinClient`） |
| `/static/admin.js` | 管理后台（`window.YoinAdmin`） |

两者都是编译期打进二进制的快照，改前端后需要重新编译 Rust 才会生效。
