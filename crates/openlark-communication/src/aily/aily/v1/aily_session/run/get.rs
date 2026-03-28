//! 获取运行
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/aily_session-run/get>

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_RUN};
use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

/// 获取运行请求
///
/// 用于获取指定运行的详细信息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `aily_session_id`: 会话 ID，必填
/// - `run_id`: 运行 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let request = GetRunRequest::new(config)
///     .aily_session_id("session_xxx")
///     .run_id("run_xxx")
///     .execute().await?;
/// ```
pub struct GetRunRequest {
    config: Config,
    aily_session_id: String,
    run_id: String,
}

impl GetRunRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            aily_session_id: String::new(),
            run_id: String::new(),
        }
    }

    /// 会话 ID（路径参数）
    pub fn aily_session_id(mut self, aily_session_id: impl Into<String>) -> Self {
        self.aily_session_id = aily_session_id.into();
        self
    }

    /// 运行 ID（路径参数）
    pub fn run_id(mut self, run_id: impl Into<String>) -> Self {
        self.run_id = run_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/aily-v1/aily_session-run/get>
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
        validate_required!(self.run_id, "run_id 不能为空");

        let url = AILY_V1_RUN
            .replace("{aily_session_id}", &self.aily_session_id)
            .replace("{run_id}", &self.run_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取运行")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_run_request_builder() {
        let config = Config::default();
        let request = GetRunRequest::new(config)
            .aily_session_id("session_xxx")
            .run_id("run_xxx");
        assert_eq!(request.aily_session_id, "session_xxx");
        assert_eq!(request.run_id, "run_xxx");
    }

    #[test]
    fn test_get_run_request_default_values() {
        let config = Config::default();
        let request = GetRunRequest::new(config);
        assert_eq!(request.aily_session_id, "");
        assert_eq!(request.run_id, "");
    }

    #[test]
    fn test_get_run_request_with_session_id_only() {
        let config = Config::default();
        let request = GetRunRequest::new(config).aily_session_id("session_123");
        assert_eq!(request.aily_session_id, "session_123");
        assert_eq!(request.run_id, "");
    }

    #[test]
    fn test_get_run_request_url_construction() {
        let request = GetRunRequest::new(Config::default())
            .aily_session_id("sess_1")
            .run_id("run_1");
        assert_eq!(request.aily_session_id, "sess_1");
        assert_eq!(request.run_id, "run_1");
    }

    #[test]
    fn test_get_run_request_chaining() {
        let config = Config::default();
        let request = GetRunRequest::new(config)
            .aily_session_id("session_xxx")
            .run_id("run_xxx");
        assert_eq!(request.aily_session_id, "session_xxx");
        assert_eq!(request.run_id, "run_xxx");
    }
}
