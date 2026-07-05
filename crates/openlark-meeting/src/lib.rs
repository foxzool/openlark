#![warn(clippy::all)]

//! # OpenLark 会议与日程服务模块
//!
//! 飞书开放平台日历（calendar）、视频会议（vc）、会议室（meeting_room）相关 API。
//!
//! ## 模块与 API 数量（目标）
//! - **calendar**：44 APIs
//! - **vc**：56 APIs
//! - **meeting_room**：17 APIs
//!
//! ## 目录结构约定
//! 实现文件严格按 `src/bizTag/project/version/resource/name.rs` 组织，其中：
//! - `bizTag`: 业务标签（calendar、vc、meeting_room）
//! - `project`: 项目标识（如 v4、v1）
//! - `resource`: 资源名称（如 calendar、meeting、room）
//! - `name`: 操作名称（如 get、list、create）
//!
//! ### 目录结构示例
//! ```text
//! src/
//! ├── calendar/v4/              # 日历 v4 API
//! │   ├── calendar/
//! │   ├── freebusy/
//! │   └── setting/
//! ├── vc/v1/                    # 视频会议 v1 API
//! │   ├── meeting/
//! │   ├── room/
//! │   └── reserve/
//! └── meeting_room/             # 会议室历史版本 API
//!     ├── room/
//!     ├── building/
//!     └── instance/
//! ```
//!
//! - `resource` 内用 `.` 分割目录（例如 `calendar.event.attendee` => `calendar/event/attendee`）
//! - 如 `meta.name` 内包含 `/`（历史接口），则按目录继续下沉（例如 `building/list` => `building/list.rs`）

pub mod common;
pub mod endpoints;

#[cfg(feature = "vc")]
pub mod vc;

#[cfg(feature = "calendar")]
pub mod calendar;

#[cfg(feature = "meeting-room")]
pub mod meeting_room;

/// Prelude 模块 - 常用导入
pub mod prelude;

// 重新导出主要类型
pub use common::chain::MeetingClient;

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_meeting_client_creation() {
        let config = create_test_config();
        let client = MeetingClient::new(config);
        assert!(client.config().app_id() == "test_app");
    }

    #[test]
    fn test_meeting_client_clone() {
        let config = create_test_config();
        let client = MeetingClient::new(config);
        let cloned = client.clone();
        assert!(cloned.config().app_id() == "test_app");
    }

    #[cfg(feature = "vc")]
    #[test]
    fn test_meeting_client_vc() {
        let config = create_test_config();
        let client = MeetingClient::new(config);
        let _vc = &client.vc;
    }
}
