# Issue #41 整改清单与优先级

> **目标**: 统一 OpenLark 全仓库 API 实现规范，消除风格混用  
> **范围**: 18 个业务模块 crates，1,560+ APIs  
> **状态**: P0 阻塞级已完成，P1/P2 待评估

---

## 1. 混用点概览（按优先级排序）

### P0 - 阻塞级（影响 API 可用性）

| # | 混用类型 | 具体位置 | 问题描述 | 整改动作 |
|---|---------|---------|---------|---------|
| 1 | **函数式 API 与 Builder 模式混用** | `crates/openlark-docs/src/ccm/explorer/v2/mod.rs` | 使用 `pub async fn get_folder_meta(config: &Config, ...)` 函数式 API，而非 Builder 模式 | 重构为 `GetFolderMetaRequest` + `execute()`/`execute_with_options()` |
| 2 | **函数式 API 与 Builder 模式混用** | `crates/openlark-docs/src/ccm/permission/v2/mod.rs` | 同上，使用函数式 API `check_member_permission(config, params)` | 重构为 `CheckMemberPermissionRequest` + Builder 模式 |
| 3 | **缺少 `execute_with_options`** | `crates/openlark-meeting/src/calendar/calendar/v4/*` | 44 个 API 仅提供 `execute()`，无 `execute_with_options(RequestOption)` | 为所有 calendar/v4 API 添加 `execute_with_options` 方法 | **已验证**: 实际检查显示 calendar/v4 所有 59 个文件均已包含 `execute_with_options`，无需修改 |

### P1 - 高风险级（影响一致性）

| # | 混用类型 | 具体位置 | 问题描述 | 整改动作 |
|---|---------|---------|---------|---------|
| 4 | **RequestOption 透传不一致** | `openlark-platform` (16 files), `openlark-helpdesk` (24 files) | 部分 API 使用 `Transport::request(..., None)`，部分使用 `Some(option)` | **已完成**: 统一将 `execute()` 委托给 `execute_with_options(RequestOption::default())`，消除 40 个文件中的代码重复 |
| 5 | **端点定义方式混用** | `openlark-docs` vs `openlark-communication` | docs 使用 `enum ApiEndpoint`，communication 使用常量 `IM_V1_MESSAGES` | 保持现状但文档化：新增 API 时优先模仿同 crate 现有风格 |
| 6 | **validate_required! 使用不完整** | `calendar/v4` 等模块 | 部分 API 缺少 `validate_required!` 校验 | 补充必填字段校验 |

### P2 - 中风险级（影响可维护性）

| # | 混用类型 | 具体位置 | 问题描述 | 整改动作 |
|---|---------|---------|---------|---------|
| 7 | **mod.rs 导出风格不一致** | 各 crate 子模块 | 部分使用 `pub mod models; pub use models::*`，部分仅 `pub mod` | 统一为显式 `pub use` 导出 |
| 8 | **serde_json::Value 过度使用** | `calendar/v4/exchange_binding/*`, `calendar/v4/freebusy/*` | 使用 `serde_json::Value` 作为请求/响应体，而非强类型结构体 | 逐步替换为强类型结构体 |
| 9 | **文档注释格式不统一** | 分散 | 部分有完整 docPath，部分缺少 doc 链接 | 统一添加 `//! docPath:` 注释 |

---

## 2. 详细混用点证据

### 2.1 函数式 API 混用（P0）

**位置**: `crates/openlark-docs/src/ccm/explorer/v2/mod.rs:90-119`

```rust
// ❌ 当前实现（函数式）
pub async fn get_folder_meta(config: &Config, folder_token: &str) -> SDKResult<FolderMetaResponse> {
    validate_required!(folder_token.trim(), "文件夹Token不能为空");
    let api_endpoint = CcmDriveExplorerApiOld::FolderMeta(folder_token.to_string());
    let api_request: ApiRequest<FolderMetaResponse> = ApiRequest::get(&api_endpoint.to_url());
    let response = Transport::request(api_request, config, None).await?;  // ❌ 无 RequestOption
    extract_response_data(response, "获取文件夹元信息")
}
```

**整改目标**:
```rust
// ✅ 目标实现（Builder 模式）
pub struct GetFolderMetaRequest {
    config: Arc<Config>,
    folder_token: String,
}

impl GetFolderMetaRequest {
    pub fn new(config: Arc<Config>, folder_token: impl Into<String>) -> Self { ... }
    
    pub async fn execute(self) -> SDKResult<FolderMetaResponse> {
        self.execute_with_options(RequestOption::default()).await
    }
    
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<FolderMetaResponse> {
        validate_required!(self.folder_token.trim(), "文件夹Token不能为空");
        let api_endpoint = CcmDriveExplorerApiOld::FolderMeta(self.folder_token);
        let api_request: ApiRequest<FolderMetaResponse> = ApiRequest::get(&api_endpoint.to_url());
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取文件夹元信息")
    }
}
```

**位置**: `crates/openlark-docs/src/ccm/permission/v2/mod.rs:42-60`

```rust
// ❌ 当前实现（函数式）
pub async fn check_member_permission(
    config: &Config,
    params: CheckMemberPermissionParams,
) -> SDKResult<CheckMemberPermissionResponse> {
    validate_required!(params.obj_token.trim(), "文件Token不能为空");
    // ...
    let response = Transport::request(api_request, config, None).await?;  // ❌ 无 RequestOption
    extract_response_data(response, "检查成员权限")
}
```

### 2.2 execute-only 模式混用（P0）

**位置**: `crates/openlark-meeting/src/calendar/calendar/v4/`（44 个文件）

**证据样本**:
- `calendar/v4/exchange_binding/get.rs:35` - `pub async fn execute(self) -> SDKResult<serde_json::Value>`
- `calendar/v4/calendar/create.rs:81` - `pub async fn execute(self, body: serde_json::Value) -> SDKResult<CreateCalendarResponse>`
- `calendar/v4/calendar/event/create.rs:55` - `pub async fn execute(self, body: serde_json::Value) -> SDKResult<CreateCalendarEventResponse>`

**问题**: 所有 44 个 API 均缺少 `execute_with_options` 方法，无法传递 `user_access_token`、`tenant_key` 或自定义 header。

**整改动作**:
为每个文件添加:
```rust
pub async fn execute_with_options(
    self, 
    body: CreateCalendarEventBody,  // 或对应类型
    option: RequestOption
) -> SDKResult<CreateCalendarEventResponse> {
    // ... 实现，确保透传 option 到 Transport::request
}
```

### 2.3 RequestOption 透传不一致（P1）

**对比证据**:

✅ **正确示例**（`openlark-docs/src/base/bitable/v1/app/table/create.rs:164`）:
```rust
let response = Transport::request(api_request, &self.config, Some(option)).await?;
```

❌ **问题示例**（`openlark-docs/src/ccm/explorer/v2/mod.rs:98`）:
```rust
let response = Transport::request(api_request, config, None).await?;
```

---

## 3. 整改优先级矩阵

| 优先级 | 模块 | API 数量 | 预计工作量 | 依赖 |
|-------|------|---------|-----------|------|
| P0 | `docs/ccm/explorer/v2` | 8 | 2-3 天 | 无 |
| P0 | `docs/ccm/permission/v2` | 3 | 1 天 | 无 |
| P0 | `meeting/calendar/v4` | 44 | 5-7 天 | 无 |
| P1 | `meeting/vc/v1` | 检查中 | 待定 | P0 完成后 |
| P1 | 其他 crate 的 `execute_with_options` 补全 | 检查中 | 待定 | P0 完成后 |
| P2 | 全仓库 mod.rs 导出统一 | ~200 | 3-5 天 | P1 完成后 |
| P2 | serde_json::Value 替换 | ~30 | 2-3 天 | 低优先级 |

---

## 4. 验收标准

### 4.1 P0 验收 checklist

- [ ] `docs/ccm/explorer/v2/mod.rs` 8 个函数全部重构为 Builder 模式
- [ ] `docs/ccm/permission/v2/mod.rs` 3 个函数全部重构为 Builder 模式
- [ ] `meeting/calendar/v4/` 44 个 API 全部添加 `execute_with_options` 方法
- [ ] 所有整改后的 API 通过 `just lint` 检查
- [ ] 所有整改后的 API 通过 `just test` 测试

### 4.2 P1 验收 checklist

- [ ] 全仓库 API 统一支持 `execute_with_options`
- [ ] 所有 `Transport::request` 调用使用 `Some(option)` 而非 `None`
- [ ] 新增 API 模板文档已同步到 `.agents/skills/openlark-api/references/`

### 4.3 P2 验收 checklist

- [ ] 所有 mod.rs 使用显式 `pub use` 导出
- [ ] 所有 API 文件包含 `//! docPath:` 注释
- [ ] 整改清单已归档到 `docs/refactoring/issue-41-backlog.md`

---

## 5. 相关资源

- **标准示例**: `.agents/skills/openlark-api/references/standard-example.md`
- **API 模板**: `.worktrees/issue-41-plan-a/docs/api-implementation-template.md`（本任务产出）
- **验证脚本**: `tools/validate_apis.py`
- **父 issue**: #41

---

## 6. 变更日志

| 日期 | 版本 | 变更 |
|------|------|------|
| 2025-04-08 | v1.0 | 初始版本，基于代码审查产出 |
