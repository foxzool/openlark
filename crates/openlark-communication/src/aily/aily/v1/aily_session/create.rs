//! 创建会话
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/aily_session/create>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_SESSIONS};

/// 创建会话请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionBody {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateSessionBody {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// 创建会话请求
///
/// 用于创建新的 Aily AI 会话。
///
/// # 字段说明
///
/// - `config`: 配置信息
///
/// # 请求体字段
///
/// - `name`: 会话名称，必填
/// - `description`: 会话描述（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let body = CreateSessionBody::new("我的会话")
///     .description("这是一个测试会话");
/// let request = CreateSessionRequest::new(config)
///     .execute(body).await?;
/// ```
pub struct CreateSessionRequest {
    config: Config,
}

impl CreateSessionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/aily-v1/aily_session/create>
    pub async fn execute(self, body: CreateSessionBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreateSessionBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(body.name, "name 不能为空");

        let req: ApiRequest<CreateSessionBody> =
            ApiRequest::post(AILY_V1_SESSIONS).json_body(&body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建会话")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_session_body_builder() {
        let body = CreateSessionBody::new("测试会话");
        assert_eq!(body.name, "测试会话");
        assert_eq!(body.description, None);
    }

    #[test]
    fn test_create_session_body_with_description() {
        let body = CreateSessionBody::new("测试会话").description("这是一个描述");
        assert_eq!(body.description, Some("这是一个描述".to_string()));
    }

    #[test]
    fn test_create_session_request_builder() {
        let config = Config::default();
        let request = CreateSessionRequest::new(config);
        assert_eq!(request.config.app_id(), "");
    }

    #[test]
    fn test_create_session_body_default() {
        let body = CreateSessionBody::new("会话名称");
        assert_eq!(body.name, "会话名称");
        assert!(body.description.is_none());
    }

    #[test]
    fn test_create_session_body_with_all_fields() {
        let body = CreateSessionBody {
            name: "完整会话".to_string(),
            description: Some("包含所有字段".to_string()),
        };
        assert_eq!(body.name, "完整会话");
        assert_eq!(body.description, Some("包含所有字段".to_string()));
    }
}
