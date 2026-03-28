//! 列出运行
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/aily_session-run/list>

use std::collections::HashMap;

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_RUNS};
use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

/// 列出运行请求
///
/// 用于分页查询指定会话中的所有运行。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `aily_session_id`: 会话 ID，必填
///
/// # 查询参数
///
/// 通过 `query_param` 方法添加查询参数，如：
/// - `page_size`: 分页大小
/// - `page_token`: 分页标记
/// - `status`: 状态筛选
///
/// # 示例
///
/// ```rust,ignore
/// let request = ListRunsRequest::new(config)
///     .aily_session_id("session_xxx")
///     .query_param("page_size", "50")
///     .query_param("status", "success");
/// ```
pub struct ListRunsRequest {
    config: Config,
    aily_session_id: String,
    query: HashMap<String, String>,
}

impl ListRunsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            aily_session_id: String::new(),
            query: HashMap::new(),
        }
    }

    /// 会话 ID（路径参数）
    pub fn aily_session_id(mut self, aily_session_id: impl Into<String>) -> Self {
        self.aily_session_id = aily_session_id.into();
        self
    }

    /// 添加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(key.into(), value.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/aily-v1/aily_session-run/list>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.aily_session_id, "aily_session_id 不能为空");

        let url = AILY_V1_RUNS.replace("{aily_session_id}", &self.aily_session_id);
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);
        for (k, v) in self.query {
            req = req.query(k, v);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "列出运行")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_list_runs_request_builder() {
        let config = Config::default();
        let request = ListRunsRequest::new(config).aily_session_id("session_xxx");
        assert_eq!(request.aily_session_id, "session_xxx");
    }

    #[test]
    fn test_list_runs_request_with_query_param() {
        let config = Config::default();
        let request = ListRunsRequest::new(config)
            .aily_session_id("session_xxx")
            .query_param("page_size", "50");
        assert_eq!(request.query.get("page_size"), Some(&"50".to_string()));
    }

    #[test]
    fn test_list_runs_request_default_values() {
        let config = Config::default();
        let request = ListRunsRequest::new(config);
        assert_eq!(request.aily_session_id, "");
        assert!(request.query.is_empty());
    }

    #[test]
    fn test_list_runs_request_with_multiple_params() {
        let config = Config::default();
        let request = ListRunsRequest::new(config)
            .aily_session_id("session_xxx")
            .query_param("page_size", "50")
            .query_param("status", "success")
            .query_param("page_token", "token123");
        assert_eq!(request.query.len(), 3);
        assert_eq!(request.query.get("status"), Some(&"success".to_string()));
    }

    #[test]
    fn test_list_runs_request_empty_query() {
        let request = ListRunsRequest::new(Config::default()).aily_session_id("test_id");
        assert!(request.query.is_empty());
    }

    #[test]
    fn test_list_runs_request_with_status_param() {
        let config = Config::default();
        let request = ListRunsRequest::new(config)
            .aily_session_id("session_xxx")
            .query_param("status", "running");
        assert_eq!(request.query.get("status"), Some(&"running".to_string()));
    }
}
