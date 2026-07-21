# OpenLark Knowledge Base

**Project**: OpenLark - 飞书开放平台 Rust SDK  
**Stack**: Rust, Tokio, Reqwest, WebSocket  
**Version**: 0.18.0
**Repository**: https://github.com/foxzool/openlark

## OVERVIEW

OpenLark 是为飞书（Feishu/Lark）开放平台构建的企业级 Rust SDK，提供 1,560+ 个 API 的类型安全访问。采用模块化架构设计，支持按需编译和功能组合。

## STRUCTURE

```
.
├── crates/                    # 18 个业务模块 crates
│   ├── openlark-core/        # 核心基础设施（HTTP、错误处理）
│   ├── openlark-client/      # 高级客户端（meta 链式入口）
│   ├── openlark-protocol/    # WebSocket 协议
│   ├── openlark-auth/        # 认证服务
│   ├── openlark-communication/  # IM 消息和联系人
│   ├── openlark-docs/        # 云文档和表格（158 APIs）
│   ├── openlark-hr/          # HR 和招聘（562 APIs）
│   ├── openlark-workflow/    # 任务和审批
│   ├── openlark-meeting/     # 视频会议
│   └── ... (其他业务模块)
├── src/                      # 根 crate 导出
├── examples/                 # 使用示例
├── tests/                    # 集成测试
└── tools/                    # 开发和维护脚本
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| 添加新 API | `crates/openlark-*/src/**/v*/` | 按业务模块和版本组织 |
| 错误处理 | `crates/openlark-core/src/error/` | CoreError 企业级错误系统 |
| HTTP 客户端 | `crates/openlark-core/src/http.rs` | 共享 reqwest 配置 |
| 模型定义 | `*/models.rs` 或 `*/models/` | Serde 序列化结构体 |
| Feature flags | `Cargo.toml` `[features]` | 50+ 功能标志 |

## CONVENTIONS

### 模块组织
- **业务领域驱动**: 按功能（HR/Docs/IM）而非技术层次组织
- **版本化 API**: `v1/`, `v2/` 子目录管理不同 API 版本
- **模型分离**: 请求/响应模型放在 `models/` 或 `models.rs`

### 代码风格
- **中文优先**: 文档和注释使用中文，面向中国开发者
- **Builder 模式**: API 请求使用流畅的构建器模式
- **Feature 条件编译**: `#[cfg(feature = "...")]` 控制模块编译
- **错误处理**: 统一使用 `CoreError`，支持链式错误上下文

### API 实现模式
```rust
// 1. 定义请求结构体（Builder 模式）
pub struct CreateXxxRequest { config: Config, ... }
impl CreateXxxRequest {
    pub fn new(config: Config) -> Self { ... }
    pub fn field(mut self, value: T) -> Self { ... }
    pub async fn execute(self) -> Result<XxxResponse> { ... }
}

// 2. 模型使用 Serde
#[derive(Debug, Serialize, Deserialize)]
pub struct XxxResponse { ... }

// 3. 验证使用宏
validate_required!(self.field, "字段不能为空");
```

### 命名规范
- **Crates**: `openlark-{module}`（小写，连字符）
- **模块**: `snake_case`
- **结构体**: `PascalCase`
- **API 方法**: `snake_case`
- **Feature flags**: 短横线连接的小写（`core-services`, `cloud-docs`）
- **Client 类型**: 所有业务 crate 导出 `XxxClient` 类型（详见 `docs/CLIENT_NAMING_CONVENTION.md`）
- **Feature 命名**: 模块 feature 为主、版本 feature 为单 API 版本 crate 合法例外（详见 `docs/FEATURE_NAMING_CONVENTION.md`）

## ANTI-PATTERNS

- ❌ 不要直接暴露内部实现细节
- ❌ 不要硬编码 URL（使用 `constants::BASE_URL`）
- ❌ 不要在 core crate 引入业务逻辑
- ❌ 不要使用 `unwrap()` 或 `expect()` 在库代码中
- ❌ 不要破坏向后兼容性（公开 API）

## COMMANDS

```bash
# 开发
just fmt              # 格式化代码
just lint             # Clippy 检查
just test             # 运行测试
just build            # 构建项目

# 质量
just check-all        # 完整检查（fmt + lint + test + coverage + audit）
just coverage         # 生成覆盖率报告
just audit            # 安全审计

# 发布
just release VERSION  # 发布新版本
```

## SKILLS

项目内置 7 个领域技能（位于 `.agents/skills/`），Agent 在对应场景下应优先调用。调用权限如需预授权，在各端本地配置中放行（`.claude/settings.json` 已被 `.gitignore` 忽略，不入库）：

| Skill | 用途 | 触发场景 |
|-------|------|----------|
| `openlark-api` | API 接口实现规范（落盘路径、Body/Response、Builder、endpoints） | 添加/重构飞书 API、调用服务端 API |
| `openlark-api-field-verify` | API 字段核对（playwright 渲染飞书文档，对比真实字段与代码实现） | 核对请求/响应字段、用户级接口字段、推断字段验证 |
| `openlark-api-validation` | API 覆盖率验证（`tools/validate_apis.py` 对比 `api_list_export.csv`） | 统计 API 数量、检查覆盖率 |
| `openlark-code-standards` | 架构一致性、命名、导出规范检查 | 审查代码规范 |
| `openlark-design-review` | 公共 API 设计审查（feature gating、端点体系、Builder/Service 一致性） | 设计审查、crate 设计 |
| `openlark-naming` | Client/Service/Resource/Request/Builder 命名规范 | 重命名、调整模块导出/prelude |
| `openlark-validation-style` | `validate_required` 函数 vs 宏、空白字符串处理 | 统一 `validate()` 写法 |

> 另有 OpenSpec 工作流技能（`.claude/skills/openspec-*`）配合 `openspec/` 目录管理变更提案，详见各 skill 文件。

## NOTES

- **MSRV**: Rust 1.88+
- **默认 Features**: `auth`
- **WebSocket**: 需要单独启用 `websocket` feature
- **测试**: 使用 `.env` 文件管理测试凭证（不要提交到 git）
- **文档**: 使用 `cargo doc --workspace --all-features` 生成完整文档
- **架构审核**: 详见 `openspec/changes/architecture-audit-review/` 中的完整审核报告和改进计划
- **Config 迁移**: `openlark_client::Config` 已在 merge-deprecated-config 移除（v0.18 breaking），统一到 `openlark_core::config::Config`；根 crate `openlark::Config` 直接 re-export core（见 CHANGELOG 迁移表）

## Agent skills

本节为 Matt Pocock 工程技能（`to-issues` / `triage` / `to-prd` / `qa` / `improve-codebase-architecture` / `diagnosing-bugs` / `tdd` 等）提供本仓库配置。三份配置文件位于 `docs/agents/`。

### Issue tracker

Issues 在 GitHub Issues（`foxzool/openlark`）跟踪，统一用 `gh` CLI；外部 PR **不**作为 triage 入口。详见 `docs/agents/issue-tracker.md`。

### Triage labels

五个 triage 角色采用默认标签字符串（`needs-triage` / `needs-info` / `ready-for-agent` / `ready-for-human` / `wontfix`），与现有 `area:` / `type:` / `priority:` / `scope:` 分类体系正交。详见 `docs/agents/triage-labels.md`。

### Domain docs

单上下文布局：仓库根一个 `CONTEXT.md` + `docs/adr/`（由 `/domain-modeling` 懒创建）。详见 `docs/agents/domain.md`。

## Cursor Cloud specific instructions

本仓库是 Rust workspace 库 SDK（不是长驻服务）。"运行应用"= 运行 `examples/` 里的示例程序。

- **工具链**: edition 2024 + MSRV 1.88，需 `stable` 工具链（≥1.88）。`rustup default stable` 已在环境预置。
- **系统依赖（非显而易见）**: `lark-websocket-protobuf` 的 build 脚本需要 `protoc`，因此 `--all-features` 构建/测试必须安装 `protobuf-compiler`（提供 `protoc`）。缺失时报 “Could not find `protoc`”。该系统依赖已在环境预置，不在更新脚本内。
- **`just`**: 未安装。`justfile` 记录了权威命令，直接用底层 `cargo` 命令即可（见下）。
- **常用命令**（直接 cargo，等价 `justfile` 目标）:
  - 构建: `cargo build --workspace --all-features`
  - 测试: `cargo test --workspace --all-features`（整套约 3 分钟）
  - Lint（CI 同款双模式）: `cargo clippy --workspace --all-targets --all-features -- -Dwarnings` 与 `cargo clippy --workspace --all-targets --no-default-features -- -Dwarnings`
  - 运行示例: `cargo run --example client_setup --features "auth,communication"`（无需真实凭证即可跑通；示例清单见 `examples/README.md`）
- **凭证**: 示例/测试默认使用占位值或 `.env`（`.env-example` 为模板），不会发真实请求，除非设置对应环境变量（如 `OPENLARK_APP_ID` / `OPENLARK_USER_SEARCH_NAME`）。
