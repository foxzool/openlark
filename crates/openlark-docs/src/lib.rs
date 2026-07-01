#![warn(clippy::all)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::needless_borrows_for_generic_args)]

//! # OpenLark 文档服务模块
//!
//! 飞书开放平台云文档服务模块，提供文档、表格、知识库等 API 访问能力。
//! Docs helper 的设计与命名规范见 `docs/docs-helper-guidelines.md`。
//!
//! ## 文档模块业务域结构 (严格按照 bizTag/project/version/resource 组织)
//!
//! ### CCM - 云文档协同 (174 APIs)
//! - **docx**: 新版文档块与群公告（19 APIs）
//! - **drive**: 云空间文件管理（59 APIs）
//! - **sheets**: 电子表格（27 APIs）
//! - **wiki**: 知识库（16 APIs）
//! - **doc**: 文档基础服务
//! - **docs**: 云文档管理
//!
//! ### BASE - 基础服务 (49 APIs)
//! - **base**: 基础应用服务
//! - **bitable**: 多维表格（46 APIs）
//!
//! ### BAIKE - 企业知识库 (27 APIs)
//! - **baike**: 知识库空间和节点
//! - **lingo**: 知识库分类和实体
//!
//! ### MINUTES - 会议纪要 (4 APIs)
//! - **minutes**: 会议转录和纪要管理
//!
//! ## 快速开始
//!
//! ```rust,ignore
//! use openlark_core::config::Config;
//! use openlark_docs::DocsClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::builder()
//!         .app_id("app_id")
//!         .app_secret("app_secret")
//!         .build();
//!     let client = DocsClient::new(config);
//!
//!     // 获取配置后构建 Request
//!     let config = client.ccm.config().clone();
//!     // 使用 openlark_docs::ccm::drive::v1::file::UploadAllRequest
//!
//!     // 高频读取场景也可以直接使用 helper
//!     let _records = client.search_bitable_records_all("app_token", "table_id").await?;
//!     let _sheet = client.find_sheet_by_title("spreadsheet_token", "2026").await?;
//!     let _range = client
//!         .resolve_sheet_range_by_title("spreadsheet_token", "2026", "A1:C5")
//!         .await?;
//!     let _ranges = client
//!         .read_sheet_ranges("spreadsheet_token", vec![_range.clone()])
//!         .await?;
//!     let _bytes = client.download_drive_file("file_token").await?;
//!     let _records = client
//!         .query_bitable_records(
//!             openlark_docs::BitableRecordQuery::new("app_token", "table_id")
//!                 .where_equals("状态", "进行中"),
//!         )
//!         .await?;
//!     let _wiki_node = client
//!         .find_wiki_node_by_path("space_id", "产品文档/发布计划")
//!         .await?;
//!     let mut pager = client.folder_children_pager("folder_token").page_size(50);
//!     let _first_page = pager.fetch_next_page().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## 特性
//!
//! - ✅ **202 APIs 全覆盖** - 飞书云文档服务完整实现（不含旧版本）
//! - ✅ **类型安全** - 强类型请求/响应结构
//! - ✅ **异步支持** - 基于 tokio 的异步 API
//! - ✅ **版本化 API** - 支持 v1/v2/v3/v4 多版本 API
//! - ✅ **构建器模式** - 流畅的 API 调用体验
//! - ✅ **标准架构** - 严格按照 bizTag/project/version/resource/name.rs 模式组织

// Include macros first - 对所有功能都启用宏
#[macro_use]
mod macros;

// Core modules
/// 通用文档数据模型。
pub mod models;

// 功能模块按业务域组织
#[cfg(feature = "ccm-core")]
/// 云文档协同能力模块。
pub mod ccm;

#[cfg(any(feature = "base", feature = "bitable"))]
/// 多维表格与基础服务模块。
pub mod base;

#[cfg(any(feature = "baike", feature = "lingo"))]
/// 企业知识库模块。
pub mod baike;

#[cfg(feature = "minutes")]
/// 会议纪要模块。
pub mod minutes;

// 通用模块 - 工具宏和类型
/// 通用工具、链式 helper 与端点定义。
pub mod common;

// Prelude模块 - 常用导入
/// 常用类型预导出模块。
pub mod prelude;

// 重新导出主要类型
#[cfg(feature = "bitable")]
pub use common::chain::BitableRecordQuery;
pub use common::chain::DocsClient;
pub use common::chain::TypedPage;
#[cfg(feature = "ccm-core")]
pub use common::chain::WikiNodePath;
#[cfg(feature = "ccm-core")]
pub use common::chain::{DriveDownloadRange, DriveUploadFile};
#[cfg(feature = "ccm-core")]
pub use common::chain::{FolderChildrenPage, FolderChildrenPager, SheetRange, SheetWriteRange};

// === 入口设计说明 ===
//
// openlark-docs 采用简化的入口设计：
//
// 1. **DocsClient** (公开入口)
//    - 唯一推荐的公开入口
//    - 提供配置获取：`docs.ccm.config()`, `docs.base.bitable.config()`
//    - 按业务域分组：`docs.ccm`, `docs.base`, `docs.baike`, `docs.minutes`
//    - 自动根据 feature 裁剪编译
//
// 2. **使用方式**
//    - 通过 DocsClient 获取配置
//    - 使用 `*Request::new(config, ...)` 构建请求
//    - 调用 `.execute().await?` 执行
//
// === 示例代码 ===
//
// ```rust
// use openlark_docs::DocsClient;
// use openlark_docs::ccm::drive::v1::file::UploadAllRequest;
//
// let docs = DocsClient::new(config);
//
// // 访问云盘服务
// let config = docs.ccm.config().clone();
// let request = UploadAllRequest::new(config, ...);
// let file = request.execute().await?;
//
// // 访问多维表格
// let config = docs.base.bitable.config().clone();
// let request = CreateTableRequest::new(config, ...);
// let table = request.execute().await?;
//
// // 高频 helper
// let records = docs.search_bitable_records_all("app_token", "table_id").await?;
// let sheet = docs.find_sheet_by_title("spreadsheet_token", "2026").await?;
// ```
//
// === 导出说明 ===
//
// 已移除所有 Service 类型，统一使用 Request 模式
// 用户应通过以下方式使用 API：
// 1. 从 `openlark_docs::*` 模块导入 Request 类型
// 2. 使用 `DocsClient` 获取配置
// 3. 构建并执行 Request

#[cfg(test)]
mod client_tests {
    use super::*;
    use openlark_core::config::Config;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_docs_client_creation() {
        let config = create_test_config();
        let client = DocsClient::new(config);
        assert_eq!(client.config().app_id(), "test_app");
    }

    #[test]
    fn test_docs_client_clone() {
        let config = create_test_config();
        let client = DocsClient::new(config);
        let cloned = client.clone();
        assert_eq!(cloned.config().app_id(), "test_app");
    }

    #[cfg(feature = "ccm-core")]
    #[test]
    fn test_docs_client_ccm() {
        let config = create_test_config();
        let client = DocsClient::new(config);
        assert_eq!(client.ccm.config().app_id(), "test_app");
    }

    #[cfg(any(feature = "base", feature = "bitable"))]
    #[test]
    fn test_docs_client_base() {
        let config = create_test_config();
        let client = DocsClient::new(config);
        let _base = &client.base;
    }
}
