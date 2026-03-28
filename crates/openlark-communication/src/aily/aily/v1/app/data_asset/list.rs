//! 查询数据知识列表
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/list>

use std::collections::HashMap;

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_DATA_ASSETS};
use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

/// 查询数据知识列表请求
///
/// 用于分页查询指定应用下的所有数据知识。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `app_id`: 应用 ID，必填
///
/// # 查询参数
///
/// 通过 `query_param` 方法添加查询参数，如：
/// - `page_size`: 分页大小
/// - `page_token`: 分页标记
///
/// # 示例
///
/// ```rust,ignore
/// let request = ListDataAssetsRequest::new(config)
///     .app_id("app_xxx")
///     .query_param("page_size", "50");
/// ```
pub struct ListDataAssetsRequest {
    config: Config,
    app_id: String,
    query: HashMap<String, String>,
}

impl ListDataAssetsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_id: String::new(),
            query: HashMap::new(),
        }
    }

    /// 应用 ID（路径参数）
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 添加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(key.into(), value.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/list>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.app_id, "app_id 不能为空");

        let url = AILY_V1_DATA_ASSETS.replace("{app_id}", &self.app_id);
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);
        for (k, v) in self.query {
            req = req.query(k, v);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询数据知识列表")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_list_data_assets_request_builder() {
        let config = Config::default();
        let request = ListDataAssetsRequest::new(config).app_id("app_xxx");
        assert_eq!(request.app_id, "app_xxx");
    }

    #[test]
    fn test_list_data_assets_request_with_query_param() {
        let config = Config::default();
        let request = ListDataAssetsRequest::new(config)
            .app_id("app_xxx")
            .query_param("page_size", "50");
        assert_eq!(request.query.get("page_size"), Some(&"50".to_string()));
    }

    #[test]
    fn test_list_data_assets_request_default_values() {
        let config = Config::default();
        let request = ListDataAssetsRequest::new(config);
        assert_eq!(request.app_id, "");
        assert!(request.query.is_empty());
    }

    #[test]
    fn test_list_data_assets_request_with_multiple_params() {
        let config = Config::default();
        let request = ListDataAssetsRequest::new(config)
            .app_id("app_xxx")
            .query_param("page_size", "50")
            .query_param("page_token", "token123");
        assert_eq!(request.query.len(), 2);
        assert_eq!(
            request.query.get("page_token"),
            Some(&"token123".to_string())
        );
    }

    #[test]
    fn test_list_data_assets_request_empty_query() {
        let request = ListDataAssetsRequest::new(Config::default()).app_id("test_app");
        assert!(request.query.is_empty());
    }
}
