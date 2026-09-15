# OpenLark 会议与日程服务模块

飞书开放平台日历（calendar）、视频会议（vc）、会议室（meeting_room）相关 API。

## 模块与 API 数量

- **calendar**：日历功能 API
- **vc**：视频会议功能 API
- **meeting_room**：会议室历史版本 API

## 目录结构说明

本 crate (`openlark-meeting`) 包含会议和日程相关的所有业务模块代码，通过 Cargo feature 条件编译控制模块的启用。

### Feature 控制

本 crate 通过 Cargo.toml 的 feature 标志控制模块的启用：

```toml
[features]
# 默认配置 - 包含视频会议和日历功能
default = ["vc", "calendar"]

# 完整配置 - 包含所有功能
full = ["vc", "calendar", "meeting-room"]

# 视频会议功能
vc = ["vc-v1"]

# 日历功能
calendar = ["calendar-v4"]

# 会议室功能（历史版本）
meeting-room = ["meeting-room-v1"]
```

### 目录结构示例

```text
# crates/openlark-meeting/src/
├── common/                    # 公共工具和模型
├── endpoints/                 # API 端点定义
├── calendar/                 # 日历模块 (calendar-v4)
│   ├── calendar/
│   │   └── v4/
│   │       ├── calendar/
│   │       ├── freebusy/
│   │       └── setting/
│   └── service.rs
├── vc/                       # 视频会议模块 (vc-v1)
│   ├── vc/
│   │   └── v1/
│   │       ├── meeting/
│   │       ├── room/
│   │       └── reserve/
│   └── service.rs
└── meeting_room/             # 会议室模块 (meeting-room-v1)
    ├── room/
    ├── building/
    └── service.rs
```

### 功能模块说明

本 crate 通过以下 feature 提供三个主要功能：

1. **视频会议（vc）**
   - 通过 `features = ["vc"]` 启用
   - API 版本：v1
   - 主要功能：会议室管理、会议管理、预约管理

2. **日历（calendar）**
   - 通过 `features = ["calendar"]` 启用
   - API 版本：v4
   - 主要功能：日历管理、日程管理

3. **会议室（meeting-room）**
   - 通过 `features = ["meeting-room"]` 启用
   - 历史版本 API
   - 主要功能：会议室管理、建筑管理

- `resource` 内用 `.` 分割目录（例如 `calendar.event.attendee` => `calendar/event/attendee`）
- 如 `meta.name` 内包含 `/`（历史接口），则按目录继续下沉（例如 `building/list` => `building/list.rs`）

## 功能特性

本模块支持条件编译，可根据需要选择功能：

```toml
[dependencies]
# 默认配置 - 包含视频会议和日历功能
openlark-meeting = "0.15"

# 按需选择功能
openlark-meeting = { version = "0.15", default-features = false, features = ["vc", "calendar"] }

# 完整功能
openlark-meeting = { version = "0.15", features = ["full"] }
```

### 可用 Feature

- `default`: 包含 `vc` 和 `calendar`（推荐）
- `full`: 包含 `vc`、`calendar` 和 `meeting-room`
- `vc`: 仅视频会议功能
- `calendar`: 仅日历功能
- `meeting-room`: 仅会议室功能

**注意**：当使用单独的 feature（如 `vc`、`calendar` 或 `meeting-room`）时，必须设置 `default-features = false`。</think>

## 快速开始

### 基础使用

```rust,no_run
use openlark_meeting::prelude::*;
use openlark_meeting::vc::v1::room::{CreateRoomRequest};
use openlark_meeting::vc::v1::reserve::{ApplyReserveRequest};
use openlark_meeting::calendar::v4::calendar::{CreateCalendarRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建会议室
    let config = Config::default();
    let room_request = CreateRoomRequest::new(config)
        .room_name("会议室1")
        .capacity(10);
    let room_response = room_request.execute().await?;
    println!("创建会议室成功: {}", room_response.room_id);

    // 预约会议
    let reserve_request = ApplyReserveRequest::new(config)
        .topic("周会")
        .start_time("2024-01-15 14:00:00")
        .end_time("2024-01-15 16:00:00");
    let reserve_response = reserve_request.execute().await?;
    println!("预约会议成功: {}", reserve_response.meeting_id);

    // 创建日历
    let calendar_request = CreateCalendarRequest::new(config)
        .summary("项目截止日")
        .description("项目相关事项");
    let calendar_response = calendar_request.execute().await?;
    println!("创建日历成功: {}", calendar_response.calendar_id);

    Ok(())
}
```

### 视频会议功能

```rust,no_run
use openlark_meeting::vc::v1::room::*;
use openlark_meeting::vc::v1::reserve::*;
use openlark_meeting::vc::v1::meeting::*;
use openlark_meeting::vc::v1::responses::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();

    // 创建会议室
    let create_room = CreateRoomRequest::new(config)
        .room_name("会议室A")
        .capacity(20)
        .building_id("building_123");
    let room = create_room.execute().await?;
    println!("会议室 ID: {}", room.room_id);

    // 预约会议
    let reserve = ApplyReserveRequest::new(config)
        .topic("项目会议")
        .room_id(room.room_id)
        .start_time("2024-01-20 10:00:00")
        .end_time("2024-01-20 12:00:00");
    let meeting = reserve.execute().await?;
    println!("会议 ID: {}", meeting.meeting_id);

    // 邀请参会人
    let invite = InviteMeetingRequest::new(config)
        .meeting_id(meeting.meeting_id);
    let result = invite.execute(serde_json::json!({
        "user_ids": vec!["user_123", "user_456"],
    })).await?;
    println!("邀请成功");

    // 开始录制
    let start_recording = StartRecordingRequest::new(config)
        .meeting_id(meeting.meeting_id);
    let recording = start_recording.execute().await?;
    println!("录制任务 ID: {}", recording.task_id);

    Ok(())
}
```

### 日历功能

```rust,no_run
use openlark_meeting::calendar::v4::calendar::*;
use openlark_meeting::calendar::v4::responses::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();

    // 创建共享日历
    let create_calendar = CreateCalendarRequest::new(config)
        .summary("团队日历")
        .description("团队共享日历")
        .color("#1890ff");
    let calendar = create_calendar.execute().await?;
    println!("日历 ID: {}", calendar.calendar_id);

    // 获取日历列表
    let list_calendars = ListCalendarsRequest::new(config)
        .page_size(20);
    let calendars = list_calendars.execute().await?;
    println!("日历数量: {}", calendars.calendars.len());

    Ok(())
}
```

### 错误处理

本模块提供增强的错误处理能力：

```rust,no_run
use openlark_meeting::common::api_utils::*;

// 获取用户友好的错误消息
let friendly_msg = get_user_friendly_message(
    "创建会议室",
    Some(11025),
    "参数验证失败"
);
println!("{}", friendly_msg);

// 获取恢复建议
let recovery = get_recovery_suggestion(
    Some(429),
    "预约会议"
);

// 判断是否可重试
let retryable = is_error_retryable(Some(429), None);
if retryable {
    let delay = calculate_retry_delay(1, Some(429));
    // 执行重试逻辑...
}
```

## 错误处理增强

本模块提供了企业级的错误处理能力：

### 分层错误消息

- **用户友好消息**：面向终端用户，通俗易懂
- **开发者技术消息**：包含错误代码和技术详情
- **运维人员消息**：包含请求ID和操作信息

### 错误恢复机制

- **自动重试判断**：根据错误码和HTTP状态码判断是否可重试
- **智能退避策略**：支持指数退避、线性退避、固定延迟
- **恢复建议**：为不同错误类型提供具体的恢复建议

### 错误上下文

每个错误都包含丰富的上下文信息：

- 错误ID（UUID）
- 时间戳（ISO 8601格式）
- 操作名称
- 端点路径
- 请求ID
- 自定义上下文数据
- 错误严重程度
- 是否可重试
- 恢复建议

## API 设计模式

### 类型安全响应

所有API都使用类型安全的响应结构：

```rust,no_run
// 替换 serde_json::Value
pub async fn execute(self, body: serde_json::Value) -> SDKResult<CreateRoomResponse> {
    // ...
}

// 响应结构包含明确的字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoomResponse {
    pub room_id: String,
}
```

### 错误上下文

```rust,no_run
use std::collections::HashMap;

let mut ctx = HashMap::new();
ctx.insert("operation", "create_room");
ctx.insert("user_id", "user_123");

let error_context = create_error_context(
    "create_room",
    Some("/open-apis/vc/v1/rooms"),
    Some("req_123"),
    ctx
);
```

## 开发指南

### 添加新API

1. 在对应的目录创建API文件
2. 实现请求结构和 `execute` 方法
3. 在 `responses.rs` 中定义响应结构
4. 使用 `extract_response_data` 提取响应数据
5. 添加完整的文档注释和官方文档链接

### 错误处理最佳实践

1. 使用 `serialize_params` 处理参数序列化错误
2. 使用 `extract_response_data` 处理响应数据为空的情况
3. 使用 `get_user_friendly_message` 获取用户友好消息
4. 使用 `get_recovery_suggestion` 获取恢复建议
5. 使用 `is_error_retryable` 判断是否可重试
6. 使用 `calculate_retry_delay` 计算重试延迟

## 依赖关系

- `openlark-core`: 核心基础设施
- `tokio`: 异步运行时
- `serde` / `serde_json`: 序列化/反序列化
- `uuid`: UUID生成（可选）
- `chrono`: 时间处理（可选）

## 版本要求

- Rust 1.88 或更高版本
- Tokio 1.0 或更高版本

## 许可证

[Apache-2.0](../../LICENSE-APACHE)
