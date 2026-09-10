# Yoin

## 功能

- 多站点管理：一个站长多个站点
- 访客评论：无需登录即可发评，访客只是未登录身份
- 匿名评论：署名状态，访客和注册用户都可以匿名发布；库中保留真实身份，普通人看不到，该站站长可以看到
- 私密评论：即使审核通过，也仅作者与该站站长可见；普通人列表里没有这条，因此也无法回复；站点可关闭私密评论
- 邮件通知（尚未实现）
- 评论审核：启用审核源后新评论进入 `pending`；后台 `PATCH /api/admin/comments/{id}` 需要 `comment.moderate`，通过后公开列表可见
- OAuth：GitHub / QQ 凭证存在 `oauth_providers`。前台 `GET /api/auth/oauth/providers` 列出已启用提供者，widget 弹窗走 start/callback，回调建用户或绑定已有邮箱并签发 JWT
- 外部身份：宿主站点已登录用户通过 `POST /api/auth/external/exchange` 提交 `{ provider, token }`，换取 Yoin JWT；身份映射写入 `user_identities`。验票、建用户与提供者管理尚未实现
- 2fa（尚未实现）
- 评论反应：Telegram 式 emoji，每人每条评论一条；站点可从全局白名单里挑选可用反应
- 页面反应：与评论反应同一套语义，目标是页面
