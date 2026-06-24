# Issue #216 修复设计 — 获取长连接在线数量接口 + 目录漂移归零

- **Issue**: [#216 飞书 API 变动检测：2026-06-22](https://github.com/foxzool/openlark/issues/216)
- **日期**: 2026-06-24
- **状态**: 待实现

## 背景

`feishu-api-catalog-watch.yml` 每周一定时刷新飞书服务端 API 清单并与仓库内基准
`api_list_export.csv` 对比，发现差异即自动开 Issue。#216 报告 **1 个新增 API + 44 条字段变化**。
经分析，二者性质完全不同，需要拆成两个独立工作流处理。

## 变更分类（triage）

| 类别 | 数量 | 性质 | 处理动作 |
|---|---|---|---|
| **A. 目录元数据漂移（噪声）** | ~44 | 飞书侧对其自身元数据做的标准化，与 SDK 行为无关 | 仅刷新基准 CSV |
| **B. 真实新增 API** | 1 | `获取长连接在线数量` `GET /open-apis/event/v1/connection` | 实现 SDK 接口 + 纳入基准 CSV |

### A 类噪声明细（无需改 SDK 代码）

飞书侧对自身 catalog 的字段做了下列标准化，全部是元数据格式变动：

- `fullPath`: `/uAjLw4CM/...` → `/document/uAjLw4CM/...`（文档站 URL 重构，加 `/document/` 前缀）
- `url`: `/open-apis/...` → `METHOD:/open-apis/...`（统一加 HTTP 方法前缀）
- `docPath`: `https://open.feishu.cn/document/...` → `（空）`（改用 fullPath）
- `bizTag` / `chargingMethod` / `isCharge`: 原本为空 → 现在补齐
- `supportAppTypes`: 数组元素顺序调整（如 `["custom","isv"]` → `["isv","custom"]`）
- OKR v2 `meta.Resource`: 去掉扁平名 → 加 `okr.` 命名空间（`alignment` → `okr.alignment`）

其中 9 个 API（机器人搜索、群组搜索、7 个 mail 搜索/撤回/签名接口）正是 #199 中新增的接口，
当时 catalog 尚未回填其元数据；现在回填完毕，因此本期再次以「字段变化」出现。SDK 侧无需任何改动。

受影响 API：bot 搜索、im 群组搜索、mail 搜索系列（7）、OKR v2 全系列（约 18）、aily agents（5）、
vc notes 订阅（2）、task 搜索（2）。

### B 类新增 API 明细

- **名称**: 获取长连接在线数量
- **方法/路径**: `GET /open-apis/event/v1/connection`
- **用途**: 查询应用（由 `tenant_access_token` 标识）的长连接在线数量，用于监控。
- **鉴权**: `tenant_access_token`；权限要求：无。
- **频控**: 10 次/秒。
- **支持应用类型**: custom、isv。
- **官方响应字段**（已从 `fullPath=/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-v1/connection/get` 拉取的 apiSchema 核对）：

| 字段 | 类型 | 描述 |
|---|---|---|
| `code` | int | 错误码，非 0 表示失败 |
| `msg` | string | 错误描述 |
| `data.online_instance_cnt` | int | 在线连接数量 |

> 该接口在结构与位置上与已有的 `event/v1/outbound_ip/list.rs`（GET，返回 IP 列表）高度相似，
> 作为实现参照。

## 设计

### 工作流 A —— 刷新基准 CSV（吸收 44 条噪声）

运行 `tools/export_server_api_list.py --output api_list_export.csv` 重新生成全量基准。
该工具调用飞书开放 catalog 接口，无需凭证。刷新后 1723 → 1724 行（纳入新增的 connection 接口），
且下周 watch 运行不再重复报告这 44 条已标准化字段。

### 工作流 B —— 新增 SDK 接口

#### 落盘路径

```
crates/openlark-communication/src/event/event/v1/connection/
├── mod.rs                # `pub mod get;`
└── get.rs                # 请求 + 强类型响应
```

#### 端点常量

在 `crates/openlark-communication/src/endpoints/event.rs` 追加：

```rust
/// 获取长连接在线数量接口。
pub const EVENT_V1_CONNECTION: &str = "/open-apis/event/v1/connection";
```

#### 强类型响应

遵循 `contact/v3/job_family` 的既有强类型范式：`Transport::request` 返回 `Response<R>`，
其中 `R` 是 **data 业务载荷类型**（`extract_response_data` 解包的就是这个 `R`），
而 `code`/`msg` 由 envelope `RawResponse` 承载，**不在**载荷 struct 内。故只定义载荷 struct：

```rust
/// 获取长连接在线数量响应（data 业务载荷）
///
/// 官方 apiSchema 响应体：code (int, envelope) / msg (string, envelope) /
/// data.online_instance_cnt (int)。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConnectionOnlineCountResponse {
    /// 在线连接数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_instance_cnt: Option<i64>,
}
```

> `online_instance_cnt` 用 `Option<i64>` + `#[serde(default)]`，与项目「向后兼容的非破坏性
> 字段追加」惯例一致（官方未来收紧为必填时不破坏调用方）。命名为 `...Response` 而非
> `...Data`，对齐 `JobFamilyResponse` / `CreateUnitResponse` 既有命名。

#### 请求结构（对齐 `contact/v3/job_family/get.rs` 强类型范式）

```rust
/// 获取长连接在线数量请求
pub struct GetConnectionOnlineCountRequest {
    config: Config,
}

impl GetConnectionOnlineCountRequest {
    pub fn new(config: Config) -> Self { Self { config } }

    /// 执行请求
    ///
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/event-v1/connection/get
    pub async fn execute(self) -> SDKResult<GetConnectionOnlineCountResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetConnectionOnlineCountResponse> {
        // url: GET:/open-apis/event/v1/connection
        let req: ApiRequest<GetConnectionOnlineCountResponse> = ApiRequest::get(EVENT_V1_CONNECTION);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取长连接在线数量")
    }
}
```

`extract_response_data` 从 `Response<R>` 解出 `R = GetConnectionOnlineCountResponse`；
`code`/`msg` 由 envelope 校验（非 0 时 Transport 层已转 `CoreError`）。

#### 模块挂载

模块链为 `event/mod.rs → event/event/mod.rs → event/event/v1/mod.rs`（目录规范
`src/biztag/project/version/resource/name.rs`）。发现 `event` 模块此前**未在
`crates/openlark-communication/src/lib.rs` 声明**（连同已有的 `outbound_ip` 一起为死代码）。
实现时需补齐该声明，使新接口（及既有 `outbound_ip`）真正可达：

- `lib.rs`：在 `pub mod endpoints;` 之后追加 `pub mod event;`（无条件，无 feature gating ——
  event 无对应 feature，与 aily/im/contact 的 feature-gated 声明不同）
- `event/mod.rs`：补文档注释 + `#![allow(clippy::module_inception)]`（`event::event` 故意嵌套，
  对齐 contact/moments/aily/im 既有写法）
- `event/event/mod.rs`、`event/event/v1/mod.rs`：补 `//!` 模块文档（满足 `missing_docs`）
- `event/event/v1/mod.rs`：由 `pub mod outbound_ip;` 扩为 `pub mod connection;` + `pub mod outbound_ip;`
- `get.rs` 导入沿用 `outbound_ip/list.rs` 的写法：
  `use crate::{common::api_utils::extract_response_data, endpoints::EVENT_V1_CONNECTION};`

> 附带修复：`outbound_ip/list.rs` 的 `new`/`execute_with_options` 因 event 模块激活而触发
> `missing_docs`，一并补文档注释（纯文档，无行为改动）。

#### 强类型响应的 ApiResponseTrait 要求

`Transport::request` 约束 `R: ApiResponseTrait`。故载荷类型须实现该 trait（对齐
`JobFamilyResponse`）：

```rust
impl ApiResponseTrait for GetConnectionOnlineCountResponse {
    fn data_format() -> ResponseFormat { ResponseFormat::Data }
}
```

import：`use openlark_core::api::{ApiRequest, ApiResponseTrait, ResponseFormat};`

### 数据流与错误处理

`execute()` → `ApiRequest::<GetConnectionOnlineCountResponse>::get(EVENT_V1_CONNECTION)` →
`Transport::request(...)` → `extract_response_data(resp, "获取长连接在线数量")` →
返回 `SDKResult<GetConnectionOnlineCountResponse>`。`code`/`msg` 经 envelope `RawResponse`
校验（非 0 时 Transport 层转 `CoreError`），错误处理与所有现有接口一致。

## 测试

- `get.rs` 内 2 个单测：
  1. 序列化往返（与 `outbound_ip/list.rs` 同款）
  2. 用官方 data 载荷形状 `{"online_instance_cnt":42}` 反序列化为
     `GetConnectionOnlineCountResponse` 并断言 `online_instance_cnt == Some(42)`
- `endpoints/event.rs` 内补 `connection` 常量的存在性断言
- 验证：`just build` + `just lint` + 受影响 crate 的定向测试通过

## 验证（Definition of Done）

- [ ] 新增 `connection` 模块，`GetConnectionOnlineCountRequest` 可编译、可挂载
- [ ] 强类型响应字段与官方 apiSchema 一致（`online_instance_cnt`）
- [ ] `api_list_export.csv` 刷新至 1724 行，下周 watch 不再重复报告 44 条噪声
- [ ] `just build` / `just lint` 通过
- [ ] 关闭 #216 的 PR 引用本设计

## 风险与向后兼容

- 纯新增（新模块、新常量、新类型），无公共 API 破坏。
- `online_instance_cnt` 用 `Option`，官方未来收紧为必填亦不破坏调用方。
- CSV 刷新使用开放 catalog 接口，完全可复现，无凭证依赖。

## 不在本次范围内（YAGNI）

- 不为 44 条字段变化的 API 做任何代码改动（纯属元数据漂移）。
- 不新增 feature flag，不重构 event 模块布局。
- 不接入真实长连接实例做端到端联调（遵循项目「测试用 `.env` 凭证、不在 CI 联网」惯例，仅做单测）。
