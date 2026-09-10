# docs

Yoin 的使用者文档，用 [mdBook](https://rust-lang.github.io/mdBook/) 构建。

```bash
mdbook serve docs --port 3002   # 本地预览 http://localhost:3002（3000/3001 是前端 playground）
mdbook build docs               # 输出到 docs/book（已 gitignore）
```

> mdBook 0.5 没有 `[serve]` 配置表，端口只能在命令行传；`book.toml` 里只留了注释提醒。

内容在 `docs/src/`，目录结构由 `docs/src/SUMMARY.md` 决定。

## 给文档本站开评论

每页底部的评论组件走 mdBook 的 `additional-js`（见 `book.toml`），入口是 `docs/yoin-docs.js`：

1. 编辑 `docs/yoin-docs.js`，填 `SITE_ID`（管理后台建站点后拿到的 id）；后端与文档不同源时再填 `API_BASE`。
2. 重新构建：`mdbook build docs`。

`SITE_ID` 保持 0 时什么都不做：不注入容器，也不发任何请求（默认值）。

组件脚本是运行时从 `${API_BASE}/static/client.js` 动态加载的，所以文档仓库里不用存放 widget 构建产物，发布时也不需要额外拷贝文件。

跨域部署注意：组件的请求带凭据，而服务端目前是宽松 CORS（`Access-Control-Allow-Origin: *`），浏览器会拒绝这类跨源请求。推荐把文档与后端放在同一域名下（例如 `/api` 反向代理到 Yoin），此时 `API_BASE` 留空即可。细节见 [嵌入到网页](src/embedding.md)。
