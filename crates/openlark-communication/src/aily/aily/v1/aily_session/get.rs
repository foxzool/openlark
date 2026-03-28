//! 获取会话
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/aily_session/get>

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_SESSION};
use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

/// 获取会话请求
///
/// 用于获取指定 Aily AI 会话的详细信息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `aily_session_id`: 会话 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let request = GetSessionRequest::new(config)
///     .aily_session_id("session_xxx")
///     .execute().await?;
/// ```
pub struct GetSessionRequest {
    config: Config,
    aily_session_id: String,
}

impl GetSessionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            aily_session_id: String::new(),
        }
    }

    /// 会话 ID（路径参数）
    pub fn aily_session_id(mut self, aily_session_id: impl Into<String>) -> Self {
        self.aily_session_id = aily_session_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/aily-v1/aily_session/get>
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

        let url = AILY_V1_SESSION.replace("{aily_session_id}", &self.aily_session_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取会话")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_session_request_builder() {
        let config = Config::default();
        let request = GetSessionRequest::new(config).aily_session_id("session_xxx");
        assert_eq!(request.aily_session_id, "session_xxx");
    }

    #[test]
    fn test_get_session_request_default_values() {
        let config = Config::default();
        let request = GetSessionRequest::new(config);
        assert_eq!(request.aily_session_id, "");
    }

    #[test]
    fn test_get_session_request_with_multiple_ids() {
        let config = Config::default();
        let request1 = GetSessionRequest::new(config.clone()).aily_session_id("session_111");
        let request2 = GetSessionRequest::new(config).aily_session_id("session_222");
        assert_eq!(request1.aily_session_id, "session_111");
        assert_eq!(request2.aily_session_id, "session_222");
    }

    #[test]
    fn test_get_session_request_empty_after_new() {
        let request = GetSessionRequest::new(Config::default());
        assert!(request.aily_session_id.is_empty());
    }
}
