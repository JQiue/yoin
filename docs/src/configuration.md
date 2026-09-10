# 站点配置

每个站点一份配置，公开接口 `GET /api/sites/{id}/config` 会把它下发给前端组件，组件据此决定输入框长度、匿名/私密开关是否可用、显示哪些表情。

| 字段 | 默认值 | 说明 |
| --- | --- | --- |
| `allow_anonymous` | `true` | 是否允许匿名评论（对外隐藏身份） |
| `allow_private` | `true` | 是否允许私密评论（仅作者与站点管理员可见） |
| `max_comment_length` | `1024` | 评论正文字符数上限（按 Unicode 字符计） |
| `comment_limit_seconds` | `60` | 同一身份在同一站点的发帖间隔 |
| `allowed_reactions` | `👍 ❤️ 😄 🎉 👎` | 该站点允许的表情，必须是全局允许集合的子集 |

默认值定义在 [src/entity/sites.rs](https://github.com/JQiue/yoin/blob/dev/src/entity/sites.rs)，全局允许的表情在 [src/constants/mod.rs](https://github.com/JQiue/yoin/blob/dev/src/constants/mod.rs)。

## 约束是怎么生效的

- **匿名 / 私密**：只有站点关闭允许时才会被拒（`forbidden`）；站点管理员不受限。
- **发帖频率**：按「站点 + 身份」计算，已登录用户按用户 id，未登录按来源 IP；触发时返回客户端错误，提示 `The comment is too fast.`
- **长度**：超出返回客户端错误 `comment too long`。
- 站点管理员（持有该站点 `site.manage`）绕过上述三项限制。

## 管理站点

站点读写都在需要认证的接口上，并要求 `site.manage` 权限：

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` | `/api/sites` | 列出可见站点 |
| `POST` | `/api/sites` | 创建站点（返回 `id`，嵌入时用它） |
| `PATCH` | `/api/sites` | 更新站点名称 / 地址 / 配置 |

## 表情反应

- 页面级反应与评论级反应分开：页面反应按 `page_path` 汇总，评论反应按评论 id 汇总；
- 反应记录跟随身份（登录用户 id 或 guest_id），同一个人对同一目标只保留一个反应，重复提交即切换；
- 站点可以通过 `allowed_reactions` 缩小范围，全局集合之外的取值会被拒。
