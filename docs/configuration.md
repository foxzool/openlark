# OpenLark 配置说明

本文档说明 `openlark 0.15.0-rc.2` 的基础环境变量、Endpoint 切换方式以及示例运行前提。

## 最小必需配置

使用 `Client::builder()` 或 `Client::from_env()` 时，最常见的必需项是：

```bash
export OPENLARK_APP_ID="your_app_id"
export OPENLARK_APP_SECRET="your_app_secret"
```

这两个值来自飞书 / Lark 开放平台应用配置。

## 可选配置

### 切换国际版 Lark

OpenLark 默认使用国内飞书开放平台地址 `https://open.feishu.cn`。

如果需要切换到国际版 Lark，可设置：

```bash
export OPENLARK_BASE_URL="https://open.larksuite.com"
```

也可以在代码里显式指定：

```rust,no_run
use open_lark::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _client = Client::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .base_url("https://open.larksuite.com")
        .build()?;

    Ok(())
}
```

## `.env` 用法

仓库示例默认支持通过 `.env` 文件加载环境变量：

```bash
cp examples/.env.example .env
```

然后将至少以下变量写入 `.env`：

```dotenv
OPENLARK_APP_ID=your_app_id
OPENLARK_APP_SECRET=your_app_secret
```

如果你运行国际版环境，也可以补充：

```dotenv
OPENLARK_BASE_URL=https://open.larksuite.com
```

## 运行示例前的建议检查

- 已复制 `examples/.env.example` 为项目根目录 `.env`
- 已设置 `OPENLARK_APP_ID`
- 已设置 `OPENLARK_APP_SECRET`
- 如需国际版 Lark，已设置 `OPENLARK_BASE_URL`
- 已按示例所需 feature 运行 `cargo run --example ... --features "..."`

## 推荐入口

`0.15.0-rc.2` 推荐统一通过根 crate `openlark` 接入：

```toml
[dependencies]
openlark = { version = "0.15.0-rc.2", default-features = false, features = ["auth"] }
```

对大多数业务开发，建议优先从根 crate 的 `Client` 和 `prelude` 开始，而不是直接依赖底层 feature crate。
