// 把 Yoin 评论组件挂到每个章节页底部。
//
// 这是 mdBook 的 `additional-js` 入口：它会在每页 </body> 前按声明顺序加载一次。
// 下面两个常量填好后重新 `mdbook build docs` 即生效；SITE_ID 保持 0 表示不启用评论。
(() => {
  const SITE_ID = 1; // Yoin 站点 id（管理后台里创建站点后拿到）
  const API_BASE = "http://127.0.0.1:7410"; // 后端地址，如 https://comments.example.com；与文档同源时留空

  // 打印页是整本合一页，不适合共用一条评论线程
  if (!SITE_ID || location.pathname.endsWith("print.html")) {
    return;
  }

  const base = API_BASE.replace(/\/+$/, "");

  const mount = (YoinClient) => {
    const container = document.createElement("div");
    container.id = "yoin-comments";
    (document.querySelector("main") || document.body).appendChild(container);
    new YoinClient({
      containerId: "yoin-comments",
      site_id: SITE_ID,
      api_base: base,
    });
  };

  // 组件从后端同源的 /static/client.js 取，避免把构建产物再拷贝一份进文档
  const script = document.createElement("script");
  script.src = `${base}/static/client.js`;
  script.onload = () => mount(window.YoinClient);
  script.onerror = () => console.error(`Yoin: 无法加载 ${script.src}`);
  document.head.appendChild(script);
})();
