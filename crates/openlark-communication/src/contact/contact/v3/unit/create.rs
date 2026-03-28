//! 创建单位
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/create>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::unit::models::CreateUnitResponse,
    endpoints::CONTACT_V3_UNIT,
};

/// 创建单位请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUnitBody {
    /// 自定义单位 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    /// 单位名字
    pub name: String,
    /// 单位类型
    pub unit_type: String,
}

/// 创建单位请求
pub struct CreateUnitRequest {
    config: Config,
}

impl CreateUnitRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/create>
    pub async fn execute(self, body: CreateUnitBody) -> SDKResult<CreateUnitResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreateUnitBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateUnitResponse> {
        validate_required!(body.name, "name 不能为空");
        validate_required!(body.unit_type, "unit_type 不能为空");

        // url: POST:/open-apis/contact/v3/unit
        let req: ApiRequest<CreateUnitResponse> =
            ApiRequest::post(CONTACT_V3_UNIT).body(serialize_params(&body, "创建单位")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "创建单位")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_unit_request_builder() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateUnitRequest::new(config);

        // 验证 request 被成功创建且配置正确
        assert_eq!(request.config.app_id(), "test_app");
    }

    #[test]
    fn test_create_unit_request_new() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateUnitRequest::new(config);

        // 验证配置正确
        assert_eq!(request.config.app_id(), "test_app");
        assert_eq!(request.config.app_secret(), "test_secret");
    }

    #[test]
    fn test_create_unit_body() {
        let body = CreateUnitBody {
            unit_id: Some("unit_123".to_string()),
            name: "Test Unit".to_string(),
            unit_type: "company".to_string(),
        };

        assert_eq!(body.unit_id, Some("unit_123".to_string()));
        assert_eq!(body.name, "Test Unit");
        assert_eq!(body.unit_type, "company");
    }

    #[test]
    fn test_create_unit_body_without_id() {
        let body = CreateUnitBody {
            unit_id: None,
            name: "Another Unit".to_string(),
            unit_type: "department".to_string(),
        };

        assert!(body.unit_id.is_none());
        assert_eq!(body.name, "Another Unit");
        assert_eq!(body.unit_type, "department");
    }
}
