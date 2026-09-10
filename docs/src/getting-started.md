# 部署与首次运行

## 前置条件

- Rust（edition 2024；`cargo +nightly fmt` 需要 nightly，运行本身用 stable 即可）
- Node.js + npm：`build.rs` 会构建前端产物，缺了会编译失败

## 环境变量

配置来自 `.env`（可选）与环境变量，字段见 [src/config.rs](https://github.com/JQiue/yoin/blob/dev/src/config.rs)。

| 变量 | 默认值 | 说明 |
| --- | --- | --- |
| `DATABASE_URL` | `sqlite://./yoin.sqlite?mode=rwc` | 数据库连接串 |
| `HOST` | `127.0.0.1` | 监听地址 |
| `PORT` | `7410` | 监听端口 |
| `JWT_KEY` | 随机 32 位 | 签发令牌的密钥，**至少 32 字符** |

`JWT_KEY` 不设置时会生成临时密钥并打警告，进程重启后所有令牌失效。生产环境必须显式设置。

## 启动

```bash
cargo run
```

编译期会把前端产物 `include_str!` 进二进制，所以：

- `build.rs` 会在 `ui/` 里执行 `npm run build`；`YOIN_SKIP_UI_BUILD=1` 可跳过（跳过前请确认 `ui/dist/client/client.js` 与 `ui/dist/admin/admin.js` 已存在）。
- debug 构建下 `cargo run` 会顺带启动前端 playground（widget 在 `http://localhost:3000/client`，后台在 `http://localhost:3001/admin`），退出时自动收掉；`YOIN_UI_DEV=0` 只跑后端，`YOIN_UI_DEV=admin` 只起后台。

启动后：

```bash
curl http://127.0.0.1:7410/api/health
```

## 第一个用户与默认站点

第一个注册的用户会自动拿到全局 `super_admin` 角色，并得到一个默认站点。注册：

```bash
curl -X POST http://127.0.0.1:7410/api/auth/register \
  -H 'content-type: application/json' \
  -d '{"email":"me@example.com","password":"secret123","nickname":"me"}'
```

返回的 `data.token` 就是后续请求要用的 Bearer 令牌。

## 接下来

1. 用管理后台（`/static/admin.js` 挂载的页面）登录，确认站点列表。
2. 按需修改站点配置（见 [站点配置](configuration.md)），例如允许匿名评论、调整评论长度上限。
3. 把评论组件挂到你的网站上（见 [嵌入到网页](embedding.md)），初始化时用站点的 `id`。

## 数据库

默认 SQLite 文件在进程工作目录下（`yoin.sqlite`）。迁移见 [migration/README.md](https://github.com/JQiue/yoin/blob/dev/migration/README.md)，服务启动时自动执行，无需手动迁移。
