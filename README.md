[![crates.io](https://img.shields.io/crates/v/openlark)](https://crates.io/crates/openlark)
[![Apache 2.0](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://github.com/foxzool/openlark/blob/main/LICENSE-APACHE)
[![Quality](https://github.com/foxzool/openlark/actions/workflows/quality.yml/badge.svg)](https://github.com/foxzool/openlark/actions/workflows/quality.yml)
[![Documentation](https://docs.rs/openlark/badge.svg)](https://docs.rs/openlark)

# 飞书开放平台非官方SDK - 企业级高覆盖率Rust实现

> 🏗️ 18 个业务模块，**1,640** 个目录 API（排除 `meta.Version=old`），企业级质量保证。
>
> 🎯 测试覆盖率 ~47%，已通过工作区 check/test，全模块 Builder 模式统一。

支持自定义机器人、长连接机器人、云文档、飞书卡片、消息、群组、招聘管理等API调用。

## 🚀 快速开始

Canonical public API 入口已经冻结：

- 普通用户默认使用根 crate `openlark`，从 `use open_lark::prelude::*;` 和 `Client` 开始
- 高级用户才直接使用 `openlark-client`
- 单业务域场景直接使用对应 `openlark-{domain}` crate

详细规则见 [`docs/PUBLIC_REEXPORT_POLICY.md`](docs/PUBLIC_REEXPORT_POLICY.md)。

### 1. 添加依赖

在您的 `Cargo.toml` 中添加：

```toml
[dependencies]
openlark = { version = "0.20.0", default-features = false, features = ["auth"] }
```

### 2. 选择功能组合

**默认配置**（仅认证能力）：
```toml
openlark = "0.20.0"
```

**推荐业务开发**：
```toml
openlark = { version = "0.20.0", default-features = false, features = ["auth", "communication"] }
```

**按需选择**：
```toml
# 文档协作：多维表格 + Sheets helper
openlark = { version = "0.20.0", default-features = false, features = ["auth", "docs-bitable"] }

# 云盘上传/文件夹遍历
openlark = { version = "0.20.0", default-features = false, features = ["auth", "docs-drive"] }

# 自定义机器人卡片 + 签名
openlark = { version = "0.20.0", default-features = false, features = ["webhook-card", "webhook-signature"] }

# 通讯 + 文档组合
openlark = { version = "0.20.0", default-features = false, features = ["auth", "communication", "docs-bitable"] }

# 文档全量能力
openlark = { version = "0.20.0", default-features = false, features = ["auth", "docs-full"] }
```

### 2.1 Feature 组合矩阵

以下是常用 feature 组合的依赖关系说明：

| Feature 组合 | 隐式依赖 | 说明 |
|-------------|---------|------|
| `docs-bitable` | `docs-ccm`, `openlark-docs/bitable` | 多维表格需要 CCM 基础 |
| `docs-drive` | `docs-ccm`, `openlark-docs/ccm-drive` | 云盘需要 CCM 基础 |
| `docs-full` | `docs-ccm` + 所有 docs 子模块 | 完整文档能力 |
| `webhook-card` | `webhook` | 卡片消息需要基础 webhook |
| `webhook-signature` | `webhook`, `hmac`, `sha2`, `base64` | 签名需要加密依赖 |
| `webhook-full` | `webhook-card` + `webhook-signature` | 完整 webhook 能力 |
| `essential` | `auth` + `communication` + `docs` | 核心业务功能 |
| `enterprise` | `essential` + `security` + `hr` + `workflow` | 企业级功能 |

**重要提示**：
- `docs-*` 类 feature 都隐式依赖 `auth`（通过 `docs-ccm` 依赖）
- `webhook-*` 类 feature 默认包含 `robot` 基础功能
- 推荐使用组合 feature（如 `essential`, `enterprise`）来简化依赖管理

### 2.2 选择平台 Endpoint

OpenLark 默认使用国内飞书开放平台 endpoint：

- 国内飞书：`https://open.feishu.cn`
- 国际版 Lark：`https://open.larksuite.com`

两者的 API 路径结构保持一致，通常都是 `/open-apis/...`，切换时只需要修改 `base_url`。

### 2.3 配置 Endpoint

可以通过构建器或环境变量切换：

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

```bash
export OPENLARK_BASE_URL="https://open.larksuite.com"
```

### 3. 基础使用

```rust,no_run
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build()?;

    // 自动分页读取文件夹子项
    let children = client
        .docs
        .list_folder_children_all("folder_token", Some("sheet"))
        .await?;
    println!("文件夹子项数量: {}", children.len());

    // 按标题查找工作表并批量读取范围
    let sheet = client
        .docs
        .find_sheet_by_title("spreadsheet_token", "汇总表")
        .await?;
    let ranges = client
        .docs
        .read_multiple_ranges(
            "spreadsheet_token",
            vec![format!("{}!A1:C10", sheet.sheet_id)],
        )
        .await?;
    println!("读取范围数量: {}", ranges.value_ranges.len());

    // 多维表格全量读取
    let records = client
        .docs
        .search_bitable_records_all("app_token", "table_id")
        .await?;
    println!("记录数量: {}", records.len());

    Ok(())
}
```

对应的可编译示例见 `examples/01_getting_started/readme_quick_start.rs`。

## 🔄 架构重构

### 重构原因

本次重构旨在将项目从早期快速开发模式升级为企业级 SDK 架构：

| 目标 | 说明 |
|------|------|
| **🔗 统一 API 调用模式** | 将散乱的 API 实现统一为 Builder 模式，提供一致的开发体验 |
| **🚪 单入口架构** | 业务经 `Client` meta 链访问（`client.<domain>`）；能力是否编译由 Cargo feature + trybuild 编译期保证 |
| **📦 模块化设计** | 按业务领域拆分为独立 crates（通讯、文档、HR、会议等），支持按需引入 |
| **🧹 技术债务清理** | 清理过时模块、简化 trait 系统、移除死代码和硬编码 URL |

### 重构进度

| 模块 | 状态 | API 数量 | 说明 |
|------|------|---------|------|
| openlark-core | ✅ 完成 | — | 核心基础设施，HTTP 客户端，错误处理（非目录 crate） |
| openlark-client | ✅ 完成 | — | 单入口 + capability catalog（meta 链式字段访问） |
| lark-websocket-protobuf | ✅ 完成 | — | WebSocket protobuf 协议（预生成源码） |
| openlark-auth | ✅ 完成 | 15 | Token 管理，认证服务 |
| openlark-hr | ✅ 完成 | 583 | 招聘、CoreHR、考勤、薪酬等 |
| openlark-docs | ✅ 完成 | 216 | 云文档、多维表格、知识库、会议纪要 |
| openlark-communication | ✅ 完成 | 179 | IM 消息、联系人、群组 |
| openlark-workflow | ✅ 完成 | 134 | 任务、审批、看板 |
| openlark-meeting | ✅ 完成 | 112 | 视频会议、日历（会议室均为 old） |
| openlark-platform | ✅ 完成 | 124 | 平台服务、Transport API |
| openlark-mail | ✅ 完成 | 107 | 邮件服务 |
| openlark-helpdesk | ✅ 完成 | 50 | 帮助台 |
| openlark-application | ✅ 完成 | 35 | 应用管理 |
| openlark-security | ✅ 完成 | 27 | 安全服务 |
| openlark-ai | ✅ 完成 | 23 | AI 智能助手 |
| openlark-analytics | ✅ 完成 | 18 | 数据分析 |
| openlark-cardkit | ✅ 完成 | 10 | 卡片组件 |
| openlark-user | ✅ 完成 | 6 | 个人设置 |
| openlark-bot | ✅ 完成 | 1 | 机器人目录 API |
| openlark-pay | ✅ 完成 | 0 | 目录仅 3 条 old，当前口径为 0 |
| openlark-webhook | ✅ 完成 | — | 自定义机器人 HTTP 入口，**不在** `api_list_export.csv` |

API 数量口径：`python3 tools/validate_apis.py --all-crates`（默认 `--skip-old`），对比 `api_list_export.csv` 与 `tools/api_coverage.toml` 映射的 17 个业务 crate。2026-09-15 测得 **1,640 / 1,640（100%）**；bizTag 拆分见 [`crates.md`](crates.md)，方法见 [`docs/typed-api-coverage.md`](docs/typed-api-coverage.md)。

### 新架构特点

```rust,no_run
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;

    // 文档 helper
    let _items = client.docs.list_folder_children_all("folder_token", None).await?;
    let _sheet = client
        .docs
        .find_sheet_by_title("spreadsheet_token", "汇总表")
        .await?;

    // 通讯模块
    let endpoint = open_lark::communication::endpoints::IM_V1_MESSAGES;
    println!("{}", endpoint);

    Ok(())
}
```

- **🔗 单入口访问** - 从 `Client` 出发，按 feature 打开需要的能力
- **🛡️ 类型安全** - 编译时验证所有 API 调用参数
- **⚡ 按需编译** - 50+ feature flags 支持按需引入功能
- **🏢 企业级质量** - 零警告构建，严格 clippy 检查

## 📖 文档和资源

- **[openlark-docs AGENTS.md](crates/openlark-docs/AGENTS.md)** - 文档服务模块知识库
- **[Typed API 覆盖率说明](docs/typed-api-coverage.md)** - 覆盖率口径、批量报告与规划输入规范
- **[API 一致性复查指南](docs/api-consistency-review.md)** - 复查实现与飞书官网一致性的方法论总纲（五维度 + 决策流 + 陷阱）
- **[API Contract 校验说明](docs/api-contract-validation.md)** - endpoint 与 request/response 字段对齐官网的验证流程
- **[快速启动示例](examples/)** - 完整功能演示示例集

## ​​📊 项目状态

- 核心模块稳定 - openlark-core、openlark-client、openlark-auth 生产就绪
- 功能模块持续完善 - 各业务模块 API 持续补充中
- 测试覆盖率 ~47% - 核心模块完整测试覆盖

**已完成**：
- ✅ 核心基础设施（core、client、lark-websocket-protobuf、auth）
- ✅ 18 个业务模块全部实现，目录 API **1,640**（排除 old，见上表口径）
- ✅ openlark-hr 583 个 API（招聘、CoreHR、考勤、薪酬）
- ✅ openlark-communication 179 个 API
- ✅ openlark-docs 216 个 API（云文档、多维表格、知识库）
- ✅ openlark-workflow 134 个 API（任务、审批、看板）
- ✅ openlark-meeting 112 个 API（视频会议、日历）
- ✅ 链式调用架构 + Builder 模式统一
- ✅ Feature flags 按需编译
- ✅ 测试覆盖率 ~47%

**进行中**：
- 🔄 部分 API 结构体字段文档补充
- 🔄 更多示例和文档完善

**计划中**：
- [ ] 更多事件处理器支持
- [ ] WebSocket 功能完善

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

[Apache-2.0](LICENSE-APACHE)

## 📚 更多文档

维护者文档索引见 [`docs/README.md`](docs/README.md)。
