# 代码结构与请求链路

## 分层

| 层 | 位置 | 职责 |
| --- | --- | --- |
| handler | `src/handler/` | HTTP 载荷、提取器、`ApiResponse` / `AppError`，保持薄 |
| service | `src/service/` | 业务规则、RBAC 判断、JWT、审核编排（方法挂在 `AppService` 上） |
| repository | `src/repository/` | 只用 SeaORM，返回 `Result<_, DbErr>`，错误用 `.with_op("...")` 包装 |
| entity | `src/entity/` | SeaORM 实体（由 `sea-orm-codegen` 生成，优先重新生成而不是手改） |
| extractor | `src/extractor.rs` | `AppJson`、`RequireAuth`、`OptionnalAuth`、`RemoteIp`、`GuestId` |
| moderation | `src/moderation/` | 可插拔的审核实现（LLM、noop） |
| rbac | `src/rbac/` | 权限与角色的引导、权限码再导出 |

请求流向：`handler` → `state.service.*` → `repo.*` → entity。**handler 里不查询 SeaORM。**

路由统一挂在 `/api` 下（`src/app.rs`），静态资源是 `/static/client.js`、`/static/admin.js`。

## 一条评论的生命周期

以 `POST /api/comments` 为例：

1. **门禁**（handler）：非管理员要过三道——匿名/私密是否被站点允许、发帖频率（按「站点 + 用户 id 或来源 IP」）、正文长度上限；
2. **归属与派生**（service）：带 `parent_id` 时校验父评论同站点同页面、未被删除/判垃圾、当前访客可见，并由父评论推出 `thread_id`；
3. **初始状态**：站点存在启用的审核提供商 → `Pending`，否则 `Approved`；
4. **异步审核**：有提供商时 `tokio::spawn` 调用审核（LLM 的 allow/review/reject 分别映射为 approved/pending/spam），回写用 `update_status_if_pending`——人工已经处理过的不被覆盖；
5. **投影**：`CommentView` 会按查看者身份裁剪——匿名评论对无权限者隐藏身份字段，`can_delete` 看作者、`can_pin` 看站点 `site.manage` 且必须是顶层评论。

前台的评论列表查询固定只取 `Approved`，私密评论再按「作者本人 / 站点管理权限」过滤。

## 首次运行的引导

- 第一个注册的账号会在 `users` 里创建，同时 `ensure_super_admin_binding` 给它全局 `super_admin`，并创建默认站点；
- 权限码与系统角色在启动时幂等引导（`src/rbac/`），重复启动不会重复创建；
- 最后一个全局 `super_admin` 受保护，不能被降级或删除绑定。

## 进程内状态

- 评论限流器与站点配置缓存都是**进程内**的 mutex/hashmap（`src/app.rs`、`src/helper.rs`）。多实例部署时它们不共享，审核与限流的判断会各算各的；
- `main` 把数据库连接 `Box::leak` 成 `'static` 传给仓库层，进程生命周期内不释放；
- CORS 目前是宽松策略（`Access-Control-Allow-Origin: *`），配合前端使用非凭据请求，因此跨域嵌入可用。

## 前端

`ui/` 是独立的 npm 包，产物有两个入口（widget 与 admin），共享 `ui/src/shared/`。两条约定值得记住：

- 所有样式被 `postcss-prefix-selector` 统一加上 `[data-yoin]` 前缀，因此组件的 CSS 不会影响宿主页面；
- 尺寸 token 与评论正文样式被固定成 px（而不是 Tailwind 默认的 rem），这样宿主页面调整根字号也不会把组件缩小。

细节见[组件接口与嵌入细节](widget.md)。

## 已知的坑

- `OptionnalAuth` 是**故意**的拼写，不要"顺手修"；
- `TESTING.md` 里的目录结构与命令已经过期，以 `cargo test --all-features` 为准；
- 根 `README.md` 是占位文件，产品状态看 `CHANGELOG.md`。
