# 第三方登录（GitHub / QQ）

配好之后，评论组件的登录弹窗里会多出对应的按钮，读者点一下就能用第三方账号发言——不用注册邮箱账号。

## 先理解三件事

1. **作用域**：创建时可留空（= 全局），也可绑到某个站点。**登录流程只使用全局的那些**，绑到站点上的目前不参与登录，所以想让它出现在登录弹窗里，作用域必须留空。
2. **回调地址必须两端一致**：你在这个第三方平台登记的回调地址，和在 Yoin 后台「Redirect URI」里填的，必须**逐字符一致**，否则平台直接报错。
3. **首次登录也是注册**：读者第一次用第三方账号登录时，Yoin 会自动为他建账号；如果这个第三方账号的邮箱与已有账号相同，则直接关联到那个账号。数据库里还没有任何用户时，用第三方登录同样会创建默认站点并把该账号设为超级管理员。

## GitHub

### 1. 在 GitHub 建一个 OAuth App

GitHub → 头像 → Settings → Developer settings → **OAuth Apps** → New OAuth App：

| 字段 | 填什么 |
| --- | --- |
| Application name | 随便，例如 `Yoin comments` |
| Homepage URL | 你的网站地址 |
| **Authorization callback URL** | `https://你的域名/api/auth/oauth/github/callback` |

回调地址里的域名是**Yoin 服务**的域名，不是评论所在的站点。

建好后记下 **Client ID**，再点 **Generate a new client secret** 并把 **Client Secret** 复制出来（只显示一次）。

### 2. 在 Yoin 后台登记

管理后台 → 「OAuth providers」 → Add provider：

| 界面字段 | 填什么 |
| --- | --- |
| Provider code | `github` |
| Client ID | 上一步的 Client ID |
| Client Secret | 上一步的 Client Secret |
| Redirect URI | 与 GitHub 里那串回调地址**完全一致** |
| Scope | 留空（全局） |
| Enable now | 勾上 |

### 3. 验证

打开挂了评论的页面 → 点登录 → 应该能看到 **GitHub** 按钮 → 点击后在弹窗里授权 → 弹窗自动关闭，页面变成已登录状态。

## QQ

路径一样，只是平台换成 QQ 互联（connect.qq.com → 网站应用）：

- 回调地址填 `https://你的域名/api/auth/oauth/qq/callback`；
- 后台里 Provider code 填 `qq`；
- QQ 需要先在平台侧审核通过网站应用，才能拿到可用的 Client ID / Secret。

## 几个已知的坑

- **弹窗被拦截**：登录依赖弹出窗口，浏览器拦截时会给出提示，需要允许本站在弹出窗口后再点一次。
- **别慢过 3 分钟**：从点击登录到在第三方页面完成授权之间有时效（3 分钟），超时后再回来会失败，重新点一次登录即可。
- **GitHub 拿不到邮箱**：目前的授权没有申请邮箱权限，所以 GitHub 返回的邮箱通常是空的，这类账号在 Yoin 里会使用 `github+<数字>@oauth.yoin.local` 这样的占位邮箱；昵称取 GitHub 用户名，头像取 GitHub 头像。评论功能不受影响。
- **QQ 也没有邮箱**：同样使用占位邮箱。
- **Client Secret 不会回显**：保存后无法再查看，需要更换时重新填一次即可。
- **同一个 provider 在同一作用域只能有一条**：重复创建会被拒绝。

管理接口与数据字段见开发者指南的 [HTTP API](../dev/http-api.md#第三方登录接口)。
