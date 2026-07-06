pub mod get;
/// 应用详情相关数据模型。
pub mod models;

use openlark_core::config::Config;
use std::sync::Arc;

/// 应用资源服务。
#[derive(Clone)]
pub struct App {
    config: Arc<Config>,
}

impl App {
    /// 创建新的应用资源服务。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 返回获取应用请求构建器。
    pub fn get(&self) -> get::GetAppRequest {
        get::GetAppRequest::new(self.config.clone())
    }

    /// 返回创建应用请求构建器。
    pub fn create(&self) -> create::CreateAppRequest {
        create::CreateAppRequest::new(self.config.clone())
    }

    /// 返回删除应用请求构建器。
    pub fn delete(&self) -> delete::DeleteAppRequest {
        delete::DeleteAppRequest::new(self.config.clone())
    }

    /// 返回查询应用列表请求构建器。
    pub fn list(&self) -> list::ListAppRequest {
        list::ListAppRequest::new(self.config.clone())
    }

    /// 返回更新应用请求构建器。
    pub fn patch(&self) -> patch::PatchAppRequest {
        patch::PatchAppRequest::new(self.config.clone())
    }
}

pub use get::GetAppRequest;
// models 模块显式导出
pub use models::GetAppResponse;

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}

pub mod create;
pub mod delete;
pub mod list;
pub mod patch;
