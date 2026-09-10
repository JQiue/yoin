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

## 样式、主题与本地化

- 所有样式都限定在 `[data-yoin]` 作用域内（Tailwind 产物统一加了该前缀），不会污染宿主页面；
- 组件尺寸按 16px 基准固定成 px，宿主改根字号（mdBook 把 `1rem` 设成 10px）不会把它整体缩小；
- 界面语言跟随 `<html lang>`：`zh*` 显示中文，其余显示英文；
- 深色模式**优先跟随宿主主题**：宿主在 `<html>` 上用 `class` 或 `data-theme` 声明时以宿主为准（mdBook 的 `light`/`rust` 是浅色，`coal`/`navy`/`ayu` 是暗色），读者切换主题时组件实时跟随，无需刷新；
- 宿主没有声明主题时，回退到系统的 `prefers-color-scheme`。

## 跨域部署

组件请求用 `credentials: "same-origin"`，跨源时按「非凭据请求」处理，服务端默认的宽松 CORS（`Access-Control-Allow-Origin: *`）即可放行——把组件挂到任意域名都能正常读写评论，不需要同源，也不需要在服务端额外配置。

游客身份不依赖 cookie：它走 `x-yoin-guest-id` 请求头 + `localStorage`（服务端也会写 `yoin_guest_id` cookie，但 `SameSite=Lax` 本就不参与跨站 XHR）。只有将来把认证改成 cookie 之后，才需要在服务端放开带凭据的跨源：

```rust
CorsLayer::new()
  .allow_origin(AllowOrigin::mirror_request())
  .allow_credentials(true)
  .allow_methods(Any)
  .allow_headers(Any)
```

同源部署（例如把 `/api` 反向代理到 Yoin）依然是最省事的方案，此时 `api_base` 留空即可。

## 游客身份

未登录访客靠 `guest_id` 区分：

- 请求头 `x-yoin-guest-id`，响应头同名回传，组件会存进 `localStorage`；
- 服务端同时会写 `yoin_guest_id` cookie（HttpOnly、SameSite=Lax）；
- 同一条评论的删除权限按「登录用户 id 或 guest_id 相同」判定。
