# docs

Yoin 的文档，用 [mdBook](https://rust-lang.github.io/mdBook/) 构建。

```bash
mdbook serve docs --port 3002   # 预览 http://localhost:3002（3000/3001 留给前端 playground）
mdbook build docs               # 输出到 docs/book（已 gitignore）
```

> mdBook 0.5 没有 `[serve]` 配置表，端口只能在命令行传；`book.toml` 里只留了注释提醒。

## 结构

```
docs/
├─ book.toml
├─ yoin-docs.js        评论组件的挂载脚本（走 mdBook 的 additional-js）
└─ src/
   ├─ SUMMARY.md       目录：用两个一级标题把书分成两大部分
   ├─ introduction.md  简介与两部分导读
   ├─ guide/           使用者指南
   └─ dev/             开发者指南
```

## 写作约定

- **使用者指南**（`guide/`）：只讲「在界面上做什么、会有什么效果」，不出现字段名、接口路径、权限码；需要精确取值时链接到 `dev/` 的对应小节；
- **开发者指南**（`dev/`）：可以自由使用字段名、接口、类型和代码位置；
- 两部分的划分靠 `SUMMARY.md` 里的一级标题（`# 使用者指南` / `# 开发者指南`），侧边栏会分组显示；
- 新增页面要登记进 `src/SUMMARY.md`。

## 给文档本站开评论

每页底部的评论组件走 `book.toml` 的 `additional-js`，入口是 `docs/yoin-docs.js`：

1. 填写 `SITE_ID`（管理后台建站点后拿到的 id）；后端与文档不同源时再填 `API_BASE`；
2. `mdbook build docs`。

`SITE_ID` 保持 0 时什么都不做（默认值）。组件脚本是运行时从 `${API_BASE}/static/client.js` 动态加载的，仓库里不需要存放 widget 产物。

注意：评论线程按**页面地址**归类，所以改动章节的文件路径（重命名、换目录）等于换了一条线程，旧评论不会跟过去。
