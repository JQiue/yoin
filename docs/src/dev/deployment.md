# 部署

这里放平台相关的部署细节。只想先把服务跑起来，看使用者指南的[快速开始](../guide/getting-started.md)。

## 默认形态：单进程 + SQLite

一个二进制、一个数据库文件，`cargo run` 或下面的容器镜像都能跑。**自部署默认用 SQLite**：单文件、零运维，挂个数据卷就完事。

| 变量 | 默认 | 说明 |
| --- | --- | --- |
| `DATABASE_URL` | `sqlite://./yoin.sqlite?mode=rwc` | 自部署保持默认即可 |
| `HOST` / `PORT` | `127.0.0.1` / `7410` | 容器里用 `HOST=0.0.0.0`、`PORT` 按平台要求给 |
| `JWT_KEY` | 每次随机 | 生产必须固定（≥32 字符），否则每次重启所有人都要重新登录 |
| `YOIN_MIGRATE` | 开 | 见下 |

> `.env` 的优先级**高于进程环境变量**（启动时会 `dotenv_override`）。容器/平台部署不要放 `.env`，否则平台注入的 `DATABASE_URL`、`JWT_KEY` 会被它盖掉。

## 迁移

启动时会先跑迁移，这是单实例自部署最省事的方式。命令行也可以单独跑：

```bash
yoin migrate   # 只执行迁移然后退出
yoin serve     # 默认子命令，等价于直接运行 `yoin`
```

进程随时可能被回收、或同时存在多个实例的平台（容器平台、无服务器），应把迁移从启动路径拿出来，改成部署时跑一次：

```bash
# 对着目标库跑一次（本机、CI 或一次性任务里）
DATABASE_URL="postgres://..." yoin migrate
```

然后让服务启动时跳过迁移：

```bash
YOIN_MIGRATE=0 …
```

`YOIN_MIGRATE` 取 `0` / `false` / `off` / `no` 时跳过启动迁移。注意**跳过迁移不等于能用空库**：启动时的权限/角色引导会读表，表不存在就直接启动失败（日志里是 `no such table: permissions`）。所以顺序是**先迁移、再启动**——容器平台首次部署要先把迁移跑掉，否则实例起不来。

## 容器镜像

仓库根目录的 `Dockerfile` 是多阶段构建：Node 阶段构建前端 → cargo-chef 编译静态 musl 二进制 → `scratch` 运行。镜像里已经设了 `HOST=0.0.0.0`。

```bash
docker build -t yoin .
docker run -p 7410:7410 -v yoin-db:/app/data \
  -e DATABASE_URL="sqlite:///app/data/yoin.sqlite?mode=rwc" \
  -e JWT_KEY="至少32字符的固定密钥" \
  yoin
```

- SQLite 要把数据目录挂成**持久卷**（上例的 `/app/data`）；`docker-compose.yml` 已经这么配了；
- `/static/client.js`、`/static/admin.js` 由进程直接提供（产物编译进二进制），不需要额外静态服务器；
- `curl http://127.0.0.1:7410/api/health` 可做健康检查。

## 部署到 Vercel

Vercel 可以把 OCI 镜像当 Function 跑：构建镜像 → 流量路由到容器 → 按需求扩缩、空闲缩到零。项目里的 `vercel.json` 直接指向现有 Dockerfile，不需要另写一份：

```json
{
  "services": { "yoin": { "root": ".", "entrypoint": "Dockerfile", "runtime": "container" } },
  "rewrites": [{ "source": "/(.*)", "destination": { "service": "yoin" } }]
}
```

项目环境变量：

| 变量 | 值 | 原因 |
| --- | --- | --- |
| `DATABASE_URL` | Postgres 连接串 | **函数文件系统非持久，SQLite 不可用**；用 Marketplace 的 Postgres（Neon 等）并开启连接池 |
| `JWT_KEY` | ≥32 字符固定值 | 同上：不固定则每次实例重启令牌全失效 |
| `PORT` | `80` | Vercel 默认把流量路由到容器的 80，可用 `PORT` 覆盖 |
| `YOIN_MIGRATE` | `0` | 冷启动与多实例都不适合跑迁移，改成部署时手动跑一次（见上） |

首次部署后，先对生产库跑一次迁移：

```bash
DATABASE_URL="postgres://..." ./yoin migrate    # 或 cargo run -- migrate
```

### 平台带来的行为差异

这些都是模型决定的，不是 bug：

- **必须外部数据库**：函数文件系统非持久，SQLite 写不进去；Postgres 后端已支持（`--no-default-features --features postgres`）；
- **进程内状态按实例算**：评论限流器与站点配置缓存在进程内存里，多实例、冷启动后各算各的，限流会明显变宽；
- **后台任务可能被截断**：审核是评论落库后 `tokio::spawn` 异步调用提供商的，实例被回收时（空闲 5 分钟后收到 SIGTERM，30 秒宽限）可能没跑完，评论会停在「待审」——安全降级，人工在后台处理即可；
- **没有优雅退出**：进程只监听 Ctrl-C（SIGINT），收到 SIGTERM 会直接结束，正在处理的请求可能被中断（客户端拿到 5xx，可重试）；
- 默认单区域（`iad1`），面向其他地区延迟更高，可改区域；
- 上限：函数包 250MB、单次调用最长 300s、请求/响应体 4.5MB —— 对本项目都够用；
- 容器镜像与原生 Rust runtime 目前都标着权限/套餐要求（beta 阶段），部署前先确认账号可用。

### 为什么不走原生 Rust runtime

Vercel 的原生 Rust runtime 是「每个 `[[bin]]` 一个函数」，要新增函数入口（`vercel_runtime` + `VercelLayer`），并把前端构建塞进它的构建流程（是否带 Node 需要另测）。容器路径直接复用现有 Dockerfile：前端在镜像里构建，二进制照旧 `axum::serve`——而数据库、后台任务、信号这三条约束两种路径完全一样。
