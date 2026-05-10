# OpenLark Knowledge Base

**Project**: OpenLark - 飞书开放平台 Rust SDK  
**Stack**: Rust, Tokio, Reqwest, WebSocket  
**Version**: 0.16.0  
**Repository**: https://github.com/foxzool/openlark

## OVERVIEW

OpenLark 是为飞书（Feishu/Lark）开放平台构建的企业级 Rust SDK，提供 1,560+ 个 API 的类型安全访问。采用模块化架构设计，支持按需编译和功能组合。

## STRUCTURE

```
.
├── crates/                    # 18 个业务模块 crates
│   ├── openlark-core/        # 核心基础设施（HTTP、错误处理）
│   ├── openlark-client/      # 高级客户端和服务注册表
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
| 服务注册 | `crates/openlark-client/src/` | ServiceRegistry 模式 |
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

## NOTES

- **MSRV**: Rust 1.88+
- **默认 Features**: `auth`
- **WebSocket**: 需要单独启用 `websocket` feature
- **测试**: 使用 `.env` 文件管理测试凭证（不要提交到 git）
- **文档**: 使用 `cargo doc --workspace --all-features` 生成完整文档
