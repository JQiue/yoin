# 嵌入到网页

评论组件是一个独立脚本（`/static/client.js`），把它引到任意页面、给一个容器即可。

```html
<div id="yoin-comments"></div>

<script src="https://comments.example.com/static/client.js"></script>
<script>
  new YoinClient({
    containerId: "yoin-comments",
    site_id: 1,
    api_base: "https://comments.example.com",
  });
</script>
```

## 初始化选项

| 选项 | 必填 | 说明 |
| --- | --- | --- |
| `containerId` | 是 | 容器元素的 `id`；找不到会在控制台报错，不会抛异常 |
| `site_id` | 是 | 站点 id，决定配置、限流与权限边界 |
| `api_base` | 否 | 后端地址；与页面同源时可省略，跨域时必须写成绝对地址且不带结尾斜杠 |

挂载后组件会给容器加上 `data-yoin` 属性，并实例自带 `destroy()` 方法用于卸载。

## 评论挂在哪一页

评论按 **当前页面路径**（`location.pathname`）归类，组件内部就是读 `location.pathname` 作为 `page_path`，不需要你传：

- 同一个 URL 的访客看到同一串评论；
- 文章改名 / 路径变化后，旧评论不会跟过来（没有覆盖 `page_path` 的选项）；
- 组件监听 `pushState`/`replaceState`/`popstate`，单页应用切换路由后会自动加载新页面的评论；
- 整站导出的「打印页 / 全站合一页」建议不要挂组件，否则全书共用一串评论。

## 样式与本地化

- 所有样式都限定在 `[data-yoin]` 作用域内（Tailwind 产物统一加了该前缀），不会污染宿主页面；
- 但组件根节点带 `min-h-screen`，是给「整页评论区」准备的；嵌在文档正文里时会占满一屏高度；
- 界面语言跟随 `<html lang>`：`zh*` 显示中文，其余显示英文；
- 深色模式只跟随系统的 `prefers-color-scheme`，没有强制主题的选项。

## 跨域部署要注意

组件的请求固定带凭据（`credentials: "include"`，用于游客 cookie），而服务端目前是宽松 CORS（`Access-Control-Allow-Origin: *`）。按浏览器规则，**带凭据的跨源请求会被拒绝**。

实测结论（文档站 `:3100` → 后端 `:7410`）：

| 请求 | 结果 |
| --- | --- |
| 普通 GET | 200 |
| 带自定义头（`Authorization` / `x-yoin-guest-id`） | 200 |
| 带 `credentials: "include"` | 被 CORS 拒绝 |

因此推荐**同源部署**：把网站与后端放在同一域名下（例如 `/api` 反向代理到 Yoin），`api_base` 留空即可，cookie 与登录态都正常。

## 游客身份

未登录访客靠 `guest_id` 区分：

- 请求头 `x-yoin-guest-id`，响应头同名回传，组件会存进 `localStorage`；
- 服务端同时会写 `yoin_guest_id` cookie（HttpOnly、SameSite=Lax）；
- 同一条评论的删除权限按「登录用户 id 或 guest_id 相同」判定。
