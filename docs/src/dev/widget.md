# 组件接口与嵌入细节

评论区与后台都是可嵌入的 Preact 应用，编译进同一个二进制，分别由 `/static/client.js`、`/static/admin.js` 提供。

## 初始化

```js
new YoinClient({ containerId: "yoin-comments", site_id: 1, api_base: "https://comments.example.com" });
new YoinAdmin({ containerId: "yoin-admin" }); // 后台没有 site_id
```

| 选项 | 必填 | 说明 |
| --- | --- | --- |
| `containerId` | 是 | 容器元素 id；找不到只打 `console.error`，不抛异常 |
| `site_id` | widget 必填 | 站点 id，决定配置、权限与限流边界 |
| `api_base` | 否 | 后端地址；同源留空。内部会去掉结尾斜杠，请求地址为 `${api_base}${path}` |

两个类都提供 `destroy()` 用于卸载（会 `render(null, container)`）。

## 评论归属

`page_path` 一律取 `location.pathname`，没有覆盖选项：

- 同一 URL 共享一条线程；
- 组件监听 `pushState`、`replaceState`、`popstate`（`useInitializeCommentPage` 里还统一发了 `yoin:locationchange`），SPA 切路由后自动重新拉取；
- 页面改名 → 路径变化 → 旧线程不再跟随；
- `print.html` 这类「整站合一页」需要由嵌入方自行排除，否则整站共用一条线程。

## 样式与尺寸

- `postcss-prefix-selector` 把 Tailwind 产物统一前缀成 `[data-yoin] ...`；宿主容器在挂载时被加上 `data-yoin` 属性，因此样式、CSS 变量都作用在该容器子树内；
- `base.css` 里把 Tailwind 的尺寸 token（`--spacing`、`--text-*`、`--radius-*`）固定成 px 等值，评论正文的排版也从 rem 改成 px：宿主修改根字号（例如 mdBook 的 `:root { font-size: 62.5% }`，`1rem = 10px`）不会把组件等比缩小；
- 媒体查询里的断点仍是 rem，无法用变量覆盖——远窄的视口下响应式阈值会比 16px 基准更早触发。

## 主题

`watchHostTheme(container)` 解析生效主题，并在容器上切换 `yo-dark` 类（样式见 `base.css`）：

1. 宿主在 `<html>` 上声明时以宿主为准——`data-theme="dark" | "light"`，或 class 里的 `dark` / `coal` / `navy` / `ayu`（暗）与 `light` / `rust`（浅，mdBook 的命名）；
2. 宿主没声明时回退 `prefers-color-scheme`；
3. 用 `MutationObserver` 监听 `<html>` 的 `class`/`data-theme`，并监听系统配色变化，切换实时生效、无需刷新。

## 本地化

`watchHtmlLang()` 监听 `<html lang>`，`zh*` 用中文词典，其余用英文（`ui/src/shared/i18n/`）。`t()` 缺键时回退英文，再缺则返回键名本身。

## 请求约定

- 认证：`Authorization: Bearer <token>`，token 存在 `localStorage` 的 `yoin:token`；
- 游客身份：请求头 `x-yoin-guest-id`（服务端同名回响应头，组件存入 `localStorage`），服务端同时写 `yoin_guest_id` cookie（HttpOnly、SameSite=Lax）；
- `credentials: "same-origin"`：跨源时按非凭据请求处理，服务端的宽松 CORS 即可放行；同源时 cookie 照常；
- 响应统一信封 `{ code, msg, data }`，成功 `code = "ok"`，失败抛出带 `msg` 的错误。

## 反应条的显形

评论上的反应选择器与回复/置顶/删除按钮默认隐藏，靠 `group-hover:` 显形。这类规则必须带 `!`（Tailwind 的 important 修饰符）：Tailwind 的 utilities 在 `@layer` 里，而宿主 CSS 常常是未分层且可能带 `!important`（mdBook 的 `.hidden { display: none !important }` 就会直接压掉普通声明）。

## 构建产物

- 两个入口分别配置 `output.library`（`window.YoinClient` / `window.YoinAdmin`），`chunkSplit` 全量打成一个文件，样式内联注入；
- 产物路径固定（无 hash），`index.d.ts` 一并拷到 `dist/*/` 作为公开类型；
- 二进制里的是编译期快照（`include_str!`），改前端后必须重新编译 Rust 才能生效。
