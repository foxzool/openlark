//! 搜索服务 V2 API
//!
//! 提供搜索服务 V2 版本的 API 访问（ADR 0001：扁平收口，无 `SearchV2` 导航壳；
//! 各资源经 `crate::search::search::v2::<resource>::XxxRequest::new(Arc<Config>)` 直路径访问）。

pub mod app;
pub mod data_source;
pub mod doc_wiki;
pub mod message;
pub mod query;
pub mod schema;
pub mod user;
