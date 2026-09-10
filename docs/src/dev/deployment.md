# 部署

这里放平台相关的部署细节。只想先把服务跑起来，看使用者指南的[快速开始](../guide/getting-started.md)。

## 默认形态：单进程 + SQLite

一个二进制、一个数据库文件，`cargo run` 或下面的容器镜像都能跑。**自部署默认用 SQLite**：单文件、零运维，挂个数据卷就完事。

| 变量 | 默认 | 说明 |
| --- | --- | --- |
| `DATABASE_URL` | `sqlite://./yoin.sqlite?mode=rwc` | 自部署保持默认即可 |
| `HOST` / `PORT` | `127.0.0.1` / `7410` | 二进制的默认值；容器镜像里已经设成 `0.0.0.0` / `80` |
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

仓库根目录的 `Dockerfile` 是多阶段构建：Node 阶段构建前端 → cargo-chef 编译静态 musl 二进制 → `scratch` 运行。镜像里已经用 `--features postgres` 同时编译了 SQLite 与 Postgres 两种驱动，`DATABASE_URL` 填哪种连接串都能连；默认环境是 `HOST=0.0.0.0` + `PORT=80`——**容器监听 80** 是惯例，也是 Vercel 容器的默认入口端口，平台注入的 `PORT`/`HOST` 会覆盖镜像里的值。

```bash
docker build -t yoin .
docker run -p 7410:7410 -v yoin-db:/app/data \
  -e PORT=7410 \
  -e DATABASE_URL="sqlite:///app/data/yoin.sqlite?mode=rwc" \
  -e JWT_KEY="至少32字符的固定密钥" \
  yoin
```

- SQLite 要把数据目录挂成**持久卷**（上例的 `/app/data`）；`docker-compose.yml` 已经这么配了；
- `/static/client.js`、`/static/admin.js` 由进程直接提供（产物编译进二进制），不需要额外静态服务器；
- `curl http://127.0.0.1:7410/api/health` 可做健康检查。

## 部署到 Vercel

Vercel 的[容器镜像](https://vercel.com/docs/functions/container-images)会构建仓库里的 Dockerfile、把镜像推到自己 registry，然后**当一个 Function 跑**：请求进来 → 容器内的 HTTP 端口。路由由 `vercel.json` 的 `services` + `rewrites` 决定（仓库里已经写好，不用动）：

```json
{
  "services": { "yoin": { "root": ".", "runtime": "container", "entrypoint": "Dockerfile" } },
  "rewrites": [{ "source": "/(.*)", "destination": { "service": "yoin" } }]
}
```

### 先记住两条硬约束

后面每一步都是在满足它们：

1. **必须用 Postgres**：函数文件系统非持久，SQLite 写不进去（镜像里两种驱动都有，填 `postgres://` 就行）；
2. **必须先迁移、再启动**：`YOIN_MIGRATE=0` 只是不自动迁移，空库启动会直接失败。

端口不用单独配：镜像默认监听 80，正是 Vercel 容器的默认入口端口。只有平台自己注入 `PORT` 时才需要对齐——而那种情况下应用会跟着 `PORT` 走，同样不用改配置。

### 步骤

**0. 前置**：装 Node.js（为了 `vercel` CLI）、准备好 Vercel 账号。`services` 与容器镜像这两个能力目前带权限/套餐要求，控制台里找不到就先确认账号是否已开放。

**1. 登录并关联项目**——在仓库根目录执行：

```bash
npm i -g vercel
vercel login
vercel link        # 没有项目就按提示新建一个
```

**2. 装 Postgres**——用 Marketplace 的 Neon，一条命令装好并连到当前项目：

```bash
vercel integration add neon
```

装完项目里会有 Neon 注入的 `DATABASE_URL`（走连接池）和 `DATABASE_URL_UNPOOLED`（直连）。应用只读 `DATABASE_URL`，代码不用改。

**3. 补齐剩下的两个变量**：

```bash
vercel env add JWT_KEY production                     # 交互式输入 ≥32 字符的固定串
vercel env add YOIN_MIGRATE production --no-sensitive # 0
```

| 变量 | 值 | 不配的后果 |
| --- | --- | --- |
| `DATABASE_URL` | Neon 注入，不用手填 | 回落 SQLite，函数里写不进去 |
| `JWT_KEY` | ≥32 字符、固定 | 每次冷启动所有人都要重新登录 |
| `YOIN_MIGRATE` | `0` | 冷启动跑迁移，多实例互相竞争 |

> **为什么这些不写进 `vercel.json`**：那里的 `env` / `build.env` 两个键在 Vercel 的 schema 里都标着 deprecated（`build.env` 只传给构建阶段），官方文档的配置项清单里已经没有它们；Services 模式下顶层的构建/运行时字段又必须挪进 service，而 service 的配置项里没有环境变量这一项。所以：**非秘密的默认值放镜像的 `ENV`**（端口就是这么处理的），**秘密留在项目环境变量里**。

**4. 先对生产库跑一次迁移**（首次部署、以及之后每次带新迁移的部署）：

```bash
# 用镜像跑（和线上同一份二进制）
docker build -t yoin .
docker run --rm -e DATABASE_URL="postgres://…（直连串）…" yoin migrate

# 或者用本地源码
DATABASE_URL="postgres://…" cargo run --features postgres -- migrate
```

用 Neon 时填 `DATABASE_URL_UNPOOLED` 那条直连串——迁移别走连接池。

**5. 部署**：

```bash
vercel deploy --prod
```

Vercel 会构建镜像 → 推 Container Registry → 建 Function → 绑生产域名。

**6. 验收**：

```bash
curl https://<你的项目>.vercel.app/api/health    # {"code":"ok","msg":"success","data":null}
vercel logs <你的项目>.vercel.app --since 10m
```

然后确认 `https://<你的项目>.vercel.app/static/admin.js` 有内容，挂个后台页面[注册第一个账号](../guide/getting-started.md)——它会自动成为超级管理员并创建默认站点（编号 1），把 1 填进 widget 的 `site_id` 就能发评论了。

### 出问题先看这里

| 现象 | 多半是 | 怎么办 |
| --- | --- | --- |
| 404，或构建日志说没有 `functions`/`static` 目录 | 服务默认是私有的，没被 rewrite 暴露；或项目的 Framework Preset 不是 Services | 确认 `vercel.json` 里有那条 `rewrites`；Project Settings → Framework Preset 选 Services |
| 500 / 容器起不来 | 端口没对上 | 镜像默认听 80；如果你另外在项目里设了 `PORT`，两者必须一致 |
| 502 / 超时 | 连不上数据库 | 看 `vercel logs`；确认 `DATABASE_URL` 是 Neon 注入的那条、第 4 步的迁移已经跑过 |
| 评论一直停在「待审」 | 迁移没跑 / 站点没配审核提供商 / 镜像里少了 CA 证书 | 见下面「镜像里的 CA 证书」 |
| 重启后所有人要重新登录 | `JWT_KEY` 没固定 | 固定一个 ≥32 字符的串 |
| 限流比预期松 | 进程内状态按实例算 | 见下 |

### 平台带来的行为差异

这些都是模型决定的，不是 bug：

- **进程内状态按实例算**：评论限流器与站点配置缓存在进程内存里，多实例、冷启动后各算各的，限流会明显变宽；
- **后台任务可能被截断**：审核是评论落库后 `tokio::spawn` 异步调用提供商的，实例空闲 5 分钟被回收时（SIGTERM + 30 秒宽限）可能没跑完，评论会停在「待审」——安全降级，人工在后台处理即可；
- **没有优雅退出**：进程只监听 Ctrl-C（SIGINT），收到 SIGTERM 直接结束，正在处理的请求可能被中断（客户端拿到 5xx，可重试）；
- **数据库连接**：每个实例的连接池上限默认 10（代码没设，取 sqlx 默认），多实例时用 Neon 的连接池串、别用直连串；
- **限额**：单次调用最长 300s（Hobby 上限也是 300s，Pro 可到 800s）、内存 Hobby 2GB / Pro 4GB、请求与响应体各 4.5MB、同一实例内并发共享 1024 个文件描述符；镜像本身的上限是单层压缩 500MB、整镜像 15GB（存储 $0.10/GB）——对本项目都远远够用；
- 默认只跑单区域 `iad1`，换区域要另外配。

### 镜像里的 CA 证书

LLM 客户端（`async-openai` → reqwest）编译时启用的是 `rustls-native-roots`：它只从系统证书目录读根证书，Linux 上优先探 `/etc/ssl/certs/ca-certificates.crt`。`scratch` 镜像本身空无一物，所以 `Dockerfile` 让 builder 阶段（Debian）装上 `ca-certificates`，再把那个 bundle 拷进运行镜像：

```dockerfile
COPY --from=rust-builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
```

Postgres 连接（sqlx）和 GitHub/QQ 登录（ureq）用的是编译进二进制的根证书，不依赖它。

> 动 `Dockerfile` 或换基础镜像后，本地 `docker build` 一次确认这行 COPY 还能找到文件——CI 只跑 cargo、不构建镜像。少了它不会报错，只会让容器里的 LLM 审核静默失败（评论一律停在「待审」）。

### 为什么不走原生 Rust runtime

Vercel 的原生 Rust runtime 是「每个 `[[bin]]` 一个函数」，要新增函数入口（`vercel_runtime` + `VercelLayer`），并把前端构建塞进它的构建流程（是否带 Node 需要另测）。容器路径直接复用现有 Dockerfile：前端在镜像里构建，二进制照旧 `axum::serve`——而数据库、后台任务、信号这三条约束两种路径完全一样。
