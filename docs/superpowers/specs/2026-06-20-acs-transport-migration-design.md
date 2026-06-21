# acs 子模块迁移到 Transport + core Config — 设计文档

- **日期**: 2026-06-20
- **状态**: 待批准
- **触发**: main 分支代码评审发现 `886ab6bb4`（"acs 子模块消除死代码，真实实现移到覆盖率路径"）与 issue `fc2a1d22b`（14 个接口为空壳）的描述相反，且整个 `openlark-security` crate 存在 crate 级架构债务（23 处原始 `reqwest::Client::new()`）。

> ## ⚠️ 实现期修正（2026-06-21，优先级高于本文下方代码示例）
>
> 样板实现（Task 2，分支 `refactor/acs-transport-migration` commit `ec48d07ac`）验证后发现下方 §3/§4 的代码示例有 3 处偏差，**以本节为准**：
>
> 1. **`Config` 是 owned，非 `Arc<Config>`**。`openlark_core::Config` 内部已 `Arc<ConfigInner>`，clone 廉价；communication/auth crate 惯例都是 owned `Config`。`AcsProject::new(Config)`、Service/Request 持 `config: Config`。
> 2. **`R`（`ApiRequest<R>` 泛型）是响应 `data` 字段的内容类型，不是包装层**。core 把 JSON 解析为 `Response<R> = {code,msg,data: R}`，`resp.data: Option<R>`。所以响应 struct **不要**写成 `XxxResponse { data: Option<...> }`（会双重嵌套）。正确：`R` 直接是数据内容——无 schema 用 `serde_json::Value`，execute 返回解包后的值；有 schema（如 face）用 typed struct 并 `impl ApiResponseTrait { data_format() = Data }`。
> 3. **31 个端点文件此前从未编译**（单数目录 `user/`/`device/`… 没被任何 `mod.rs` 声明），是纯覆盖率填充。实现时需新建 `<singular>/mod.rs` 声明子模块，并在 `v1/mod.rs` 加 `pub mod <singular>;`。
>
> **可信模板：commit `ec48d07ac` 的 `user/get.rs`、`user/face/get.rs`、`users/mod.rs`、`v1/mod.rs`。**

## 1. 背景与问题

### 1.1 评审发现

`openlark-security` crate 的 acs 子模块存在三套并存的类型：

1. **stub `*Request`/`*Response`**（`acs/acs/v1/user/get.rs` 等 31 个文件）：从未被构造（已 grep 确认无外部引用），存在仅为了 `validate_apis.py` 按文件路径计数。`886ab6bb4` 还从这些文件**删除**了 `validate_required!`、把 `GetUserFaceResponse` 的 `FaceData` 类型化字段退化成 `serde_json::Value`、并让 `user/face/get` 丢弃响应体（`Ok(GetUserFaceResponse { data: None })`）。
2. **`Service`/`Builder`**（`users/mod.rs` 等 6 个 `mod.rs`）：实际被 `AcsV1Service` 调用，但绕过 SDK 的 `Transport` 层，直接 `reqwest::Client::new()`、手工拼 URL、手工塞 `Authorization: Bearer` 头、`get_app_token` 手工取 token。
3. **`models::acs::*`**（`PermissionRuleRequest`、`DeviceBindRuleRequest` 等）：仅被 `Builder` 使用，与 stub 类型重复。

提交信息称"真实实现移到覆盖率路径"，实际上没有任何实现被移动——只是新增了重复的 stub，并删除了已有的校验。

### 1.2 范围更广的 crate 级债务

经 grep 确认，`reqwest::Client::new()` 的原始请求栈**不是 acs 独有**，而是整个 `openlark-security` crate（acs 6 个 `mod.rs` + security_and_compliance 4 个 `mod.rs`）共 **23 处**都这样。这是预先存在的架构债务，与 SDK 其余 crate（communication/helpdesk/platform/docs 等都用 `openlark_core::http::Transport`）不一致。

`SecurityConfig`（crate 自有的配置类型，含 `app_id`/`app_secret`/`base_url` + `get_app_access_token`）是这个原始栈的根基，被 `SecurityClient`、`AcsProject`、`security_and_compliance` 的 Service、`openlark-client/src/client.rs` 共同使用。它是公开导出类型（`pub mod models`），不能简单删除。

## 2. 目标与非目标

### 目标

- acs 子模块（`acs/acs/v1/**` 31 个端点 + 6 个资源 Service）**唯一规范实现**走 `openlark_core::http::Transport` + `openlark_core::config::Config` + `validate_required!`，与 SDK 其余 crate 一致。
- 删除 acs 的原始 reqwest 路径、重复的 `*Builder` 类型、无意义的序列化测试。
- 修复 `rule_external` 子模块的字段语义错误（经文档核对，见 §5）。
- 保持向后兼容：`SecurityClient::new(SecurityConfig)` 签名不变，`client.security().acs()...` 调用链照常工作。

### 非目标（显式排除）

- `security_and_compliance/**` 子模块（9 处原始 reqwest）**本次不动**。完成后开 GitHub issue 跟踪同类迁移。
- 不删除 `SecurityConfig` 类型本身（仍被 security_and_compliance + client.rs 使用）。
- 不改 `validate_apis.py` 的路径计数逻辑（31 个文件路径不变，覆盖率统计不受影响）。

## 3. 架构：边界转换（Approach A）

```
openlark-client (client.rs)
    │  构造 SecurityConfig（现有逻辑不变）
    ▼
SecurityClient::new(SecurityConfig)
    │  SecurityServices::new 内：为 acs 构造一个 openlark_core::Config
    │  （由 SecurityConfig.{app_id,app_secret,base_url} 转换）
    ▼
AcsProject::new(Arc<Config>)        ← acs 改用 core Config
    ▼
AcsV1Service → 6 个资源 Service（每个只持 Arc<Config>，返回 *Request 构建器）
    ▼
*Request::execute_with_options()
    → validate_required! / validate_required_list!
    → ApiRequest::{get,post,patch,delete}(&path).body(...).query(...)
    → Transport::request(req, &self.config, Some(option))
    → resp.data.ok_or_else(|| validation_error(...))
```

**转换点**：`SecurityServices::new(security_config)` 内部用 `Config::new(...)`（或等价构造）从 `security_config` 字段构建 core `Config`，传给 `AcsProject::new`。security_and_compliance 继续直接吃 `SecurityConfig`，不受影响。

这是一次性 shim；待 security_and_compliance 迁移完成后，shim 删除，整个 crate 统一到 core `Config`。

### 3.1 选 A 不选 B/C 的理由

- **B（acs 也用 SecurityConfig，只换 Transport）**：`Transport` 当前与 core `Config` 强耦合，要么改 Transport（影响所有 crate），要么每次请求临时构造 core `Config`（浪费）。且 acs 与 SDK 其余 crate 不一致。
- **C（删 SecurityConfig，crate 根也改 core Config）**：超出"只修 acs"范围，强制连带迁移 security_and_compliance，且破坏公开的 `SecurityClient::new(SecurityConfig)` 签名。
- **A**：满足"acs 规范化"承诺，不波及 security_and_compliance，保持公开 API 兼容。shim 是可控的临时债。

## 4. 规范端点实现模式

每个端点文件遵循 analytics `RemoveReportRuleViewRequest` 的既有模式：

```rust
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GetUserRequest {
    config: Arc<Config>,
    user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetUserResponse {
    fn data_format() -> ResponseFormat { ResponseFormat::Data }
}

impl GetUserRequest {
    pub fn new(config: Arc<Config>, user_id: impl Into<String>) -> Self {
        Self { config, user_id: user_id.into() }
    }

    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    pub async fn execute(self) -> SDKResult<GetUserResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetUserResponse> {
        validate_required!(self.user_id.trim(), "user_id 不能为空");
        let path = format!("/open-apis/acs/v1/users/{}", self.user_id);
        let req = ApiRequest::get(&path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取用户信息", "响应数据为空")
        })
    }
}
```

### 4.1 核心要点

- `config: Arc<Config>`（core Config，非 SecurityConfig）。
- 路径参数 ID（`user_id`/`visitor_id`/`device_id`/`access_record_id` 等）在 `execute_with_options` 开头 `validate_required!` 校验（恢复 `886ab6bb4` 删除的校验）。
- 返回真实 `resp.data`（恢复 `user/face/get` 等丢弃响应的行为）。
- body 用 `.body(...)` 或 `.json_body(&self.body)`；query 参数用 `ApiRequest::query(k, v)` / `query_opt(k, v)`（core 已有 API）。

## 5. rule_external 字段语义修复（已文档核对）

经 Playwright 渲染飞书文档（`/document/uAjLw4CM/ukTMukTMukTM/acs-v1/rule_external/{create,delete,get,device_bind}`）确认，当前 `rule_external/mod.rs` 的实现**字段语义错误**：

| 端点 | 文档要求 | 当前实现 | 修复 |
|------|---------|---------|------|
| `create` | `?rule_id=...&user_id_type=...` 查询参数 + body `{"rule": {devices:[...]}}` | 仅发 body（`12cc9fe09` 的 `{"rule":...}` 包装正确），缺 query 参数 | 补 `rule_id`/`user_id_type` 查询参数；body 包装保留 |
| `delete` | `?rule_id=...` 查询参数，**无 body** | 发 JSON body `{"rule_id": ...}`（错误） | 改为纯查询参数，删 body |
| `get` | `?device_id=...&user_id_type=...` 查询参数，无 body | （stub，未真正发请求） | 查询参数 `device_id` + `user_id_type` |
| `device_bind` | flat body `{"device_id": "<单个>", "rule_ids": [...]}` | 发 `{rule_id, device_ids[], overwrite}`（字段名错） | 改字段为 `device_id`(单) + `rule_ids`(数组) |

> **注意**：`rule_external` 的 4 个端点文件（`create.rs`/`delete.rs`/`get.rs`/`device_bind.rs`）目前是 stub（`*Request`/`*Response`，未真正发请求）；真实请求逻辑在 `rule_external/mod.rs` 的 `*Builder::send()` 里。本次重写要把真实逻辑落到 4 个端点文件，删除 `mod.rs` 的 `*Service`/`*Builder`（见 §6）。

## 6. Service/Builder 层：瘦化为门面

现有 `*Service`/`*Builder`（raw reqwest）**全部删除**。每个资源保留一个轻量门面 Service（仿 `MailService`），只持 `Arc<Config>`，返回 `*Request` 构建器：

```rust
pub struct UsersService { config: Arc<Config> }
impl UsersService {
    pub fn new(config: Arc<Config>) -> Self { Self { config } }
    pub fn get(&self, user_id: impl Into<String>) -> GetUserRequest {
        GetUserRequest::new(self.config.clone(), user_id)
    }
    pub fn list(&self) -> ListUsersRequest { ListUsersRequest::new(self.config.clone()) }
    pub fn patch(&self, user_id: impl Into<String>) -> PatchUserRequest {
        PatchUserRequest::new(self.config.clone(), user_id)
    }
}
```

删除项：
- 所有 `reqwest::Client::new()` 调用（acs 约 13 处）。
- `get_app_token` 辅助 + 手工 `Authorization: Bearer` 头。
- 所有 `*Builder` 类型（`GetUserBuilder`、`CreateRuleBuilder` 等）—— 被 `*Request` 构建器取代。
- 6 个 acs `mod.rs` 里只测 `serde_json` roundtrip 的无意义测试。
- 重复的 `models::acs::*` 请求结构（`PermissionRuleRequest`、`DeviceBindRuleRequest` 等）—— 折叠进 `*Request` 的 body。`models::acs` 的**响应**模型（`UserInfo`、`PermissionRuleInfo` 等）若被 typed Response 使用则保留公开；未使用的删除。

## 7. 删除清单（acs 范围）

- `acs/acs/v1/{users,devices,visitors,user_faces,access_records,rule_external}/mod.rs` 中的 `*Service`/`*Builder`/`get_app_token`/raw reqwest 逻辑。
- `models::acs` 中仅被 Builder 使用的请求结构体（响应结构体按需保留）。
- acs 6 个 `mod.rs` 里的占位序列化测试。
- **不删**：`SecurityConfig`、`models/mod.rs`、`security_and_compliance/**`。

## 8. 向后兼容性

- ✅ `SecurityClient::new(SecurityConfig)` 签名不变。
- ✅ `client.security().acs().v1().users().get(id).execute()` 调用链对外不变（Service 方法返回 `*Request`，`.execute()` 模式一致）。
- ⚠️ **公开类型微调**：acs 各 `*Response.data` 当前是 `Option<serde_json::Value>`（或被丢弃）。改为返回真实 `resp.data` 后，结构一致的调用者不受影响；原本丢弃响应的（实际无人调用）只会从必然 `None` 变成真实值。`user/face/get` 恢复 `FaceData { face_url }` typed 字段（公共类型新增，非破坏）。
- ⚠️ `AcsProject`/`AcsV1Service` 构造签名从 `SecurityConfig` 改为 `Config`。这两个类型虽 `lib.rs` re-export，但实际只在 crate 内构造（唯一外部调用点是 `client.rs`，会一并改），无真实外部破坏。设计中注明此点。

## 9. 测试策略

- **替换无意义测试**：每个端点加真实测试，覆盖：
  - 缺必填项时 `validate_required!` 是否触发（返回 `CoreError::validation_msg`）。
  - 路径/查询参数构造正确（可断言 `api_path()` 或构造结果，不发网络）。
- **覆盖率不回归**：文件路径不变，`validate_apis.py` 的 31 个端点计数不变。
- **门禁**：`just check-all`（fmt + lint + test + coverage）提交前必须通过。
- 端到端测试（真实飞书调用）依赖 `.env` 凭证，不在本次范围；本次只保证编译 + 单元测试通过。

## 10. 实施顺序（高层，细节交 writing-plans）

1. 在 `SecurityServices::new` 加 SecurityConfig → core Config 转换，`AcsProject`/`AcsV1Service` 改吃 `Arc<Config>`。验证 crate 仍编译（acs 内部仍指向旧 Builder，但边界已通）。
2. 逐资源（users → devices → visitors → user_faces → access_records → rule_external）重写端点文件为规范模式，同步瘦化对应 `mod.rs` 的 Service 为门面。
3. rule_external 单独按 §5 修字段语义（含 create 的 query 参数、delete 的去 body、device_bind 的字段名修正）。
4. 删除 §7 的死代码，跑 `just fmt` 清理。
5. `just check-all` 全绿。
6. 开 GitHub issue 跟踪 security_and_compliance 的同类迁移。

## 11. 风险与缓解

| 风险 | 缓解 |
|------|------|
| SecurityConfig → core Config 转换丢字段（如 token 缓存） | core `Transport` 已内置 token 解析（按 `supported_access_token_types` 自动注入 `Authorization`，见 `openlark-core/src/http.rs`），无需手工 `get_app_access_token`。实现时确认 acs 端点默认走 **app access token**（飞书 acs 应用级接口所需）——必要时在 `*Request` 构造 `ApiRequest` 后显式设置 `with_supported_access_token_types([App])` |
| 31 个端点逐个重写易遗漏校验 | 每个端点都加"缺必填报错"的单元测试，机械保证 |
| rule_external 字段名改错 | 已用 Playwright 核对真实文档（§5 表）；device_bind 的 `device_id`/`rule_ids` 直接来自文档示例 |
| 公开类型改动破坏下游 | `AcsProject`/`AcsV1Service` 仅内部构造（§8 已确认）；`*Response.data` 改为真实返回是增强非破坏 |

## 12. 验收标准

- [ ] acs 31 个端点全部走 `Transport::request`，无 `reqwest::Client::new()`（grep 验证）。
- [ ] acs 无 `*Builder` 类型，无 `get_app_token` 辅助。
- [ ] 路径参数端点全部恢复 `validate_required!`。
- [ ] `user/face/get` 返回真实 `resp.data`，`FaceData` 类型恢复。
- [ ] rule_external 的 create/delete/get/device_bind 字段语义与文档一致。
- [ ] `SecurityClient::new(SecurityConfig)` 签名不变，`client.security().acs()...` 链路可用。
- [ ] `just check-all` 全绿。
- [ ] GitHub issue 已开（security_and_compliance 迁移跟踪）。
