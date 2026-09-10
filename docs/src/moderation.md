# 评论审核

## 状态机

一条评论有四种状态，公开列表只返回 `approved`：

| 状态 | 含义 |
| --- | --- |
| `pending` | 待审，等待人工或审核提供商判定 |
| `approved` | 已通过，访客可见 |
| `spam` | 判定为垃圾，且不可作为回复的父评论 |
| `deleted` | 已删除（软删，保留 `deleted_at`） |

## 初始状态怎么定

评论插入时会先落库，再看该站点是否配置了**启用的审核提供商**：

- 没有提供商 → 直接 `approved`；
- 有提供商 → 先写 `pending`，随后异步调用提供商判定，回写结果。

判定映射（[src/service/comment.rs](https://github.com/JQiue/yoin/blob/dev/src/service/comment.rs)）：

| 提供商返回 | 状态 |
| --- | --- |
| `allow` | `approved` |
| `review` | `pending` |
| `reject` | `spam` |
| 调用失败 / 异常 | `pending` |

回写使用「仅当仍为 `pending` 时才更新」，所以人工已经处理过的评论不会被异步结果覆盖。

## 提供商

| 类型 | 状态 |
| --- | --- |
| `llm` | 已实现：按站点配置模型、API base、API key 与审核规则，返回 `allow/review/reject` |
| `akismet` | **尚未实现**：后台可以配置，但配置后会打 `akismet moderation provider configured but not implemented` 警告，评论一直停在 `pending` |

提供商按站点配置，接口见下方。

## 人工审核

需要 `comment.moderate` 权限：

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` | `/api/admin/comments` | 按站点、页面路径、状态筛选评论 |
| `PATCH` | `/api/admin/comments/{id}` | 修改状态（通过 / 垃圾 / 删除等） |

## 谁能看见什么

- 公开列表（`GET /api/comments`、`/api/comments/{id}/replies`）只返回 `approved`；
- **私密评论**只对作者本人（登录用户 id 或 guest_id 相同）和持有该站点 `site.manage` 的人可见，其他人看到的是「不存在」；
- **匿名评论**对没有站点管理权限的访客隐藏真实身份字段；有权限的审核者能看到原始信息；
- 删除：作者可删自己的评论；持有 `comment.delete.any` 的人可删任意评论；
- 置顶：需要该站点的 `site.manage`，且只能置顶顶层评论。
