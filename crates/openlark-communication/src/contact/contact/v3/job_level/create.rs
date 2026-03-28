//! 创建职级
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_level/create>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::job_level::models::{I18nContent, JobLevelResponse},
    endpoints::CONTACT_V3_JOB_LEVELS,
};

/// 创建职级请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJobLevelBody {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    pub status: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<Vec<I18nContent>>,
}

/// 创建职级请求
pub struct CreateJobLevelRequest {
    config: Config,
}

impl CreateJobLevelRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_level/create>
    pub async fn execute(self, body: CreateJobLevelBody) -> SDKResult<JobLevelResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreateJobLevelBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<JobLevelResponse> {
        validate_required!(body.name, "name 不能为空");

        // url: POST:/open-apis/contact/v3/job_levels
        let req: ApiRequest<JobLevelResponse> =
            ApiRequest::post(CONTACT_V3_JOB_LEVELS).body(serialize_params(&body, "创建职级")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "创建职级")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_job_level_request_builder() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateJobLevelRequest::new(config);

        // 验证 request 被成功创建且配置正确
        assert_eq!(request.config.app_id(), "test_app");
    }

    #[test]
    fn test_create_job_level_request_new() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateJobLevelRequest::new(config);

        // 验证配置正确
        assert_eq!(request.config.app_id(), "test_app");
        assert_eq!(request.config.app_secret(), "test_secret");
    }

    #[test]
    fn test_create_job_level_body() {
        let body = CreateJobLevelBody {
            name: "P8".to_string(),
            description: Some("Senior Engineer".to_string()),
            order: Some(8),
            status: true,
            i18n_name: None,
            i18n_description: None,
        };

        assert_eq!(body.name, "P8");
        assert_eq!(body.description, Some("Senior Engineer".to_string()));
        assert_eq!(body.order, Some(8));
        assert!(body.status);
    }

    #[test]
    fn test_create_job_level_body_minimal() {
        let body = CreateJobLevelBody {
            name: "P1".to_string(),
            description: None,
            order: None,
            status: false,
            i18n_name: None,
            i18n_description: None,
        };

        assert_eq!(body.name, "P1");
        assert!(body.description.is_none());
        assert!(body.order.is_none());
        assert!(!body.status);
    }
}
