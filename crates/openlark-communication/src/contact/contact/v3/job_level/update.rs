//! 更新职级
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_level/update>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::job_level::models::{I18nContent, JobLevelResponse},
    endpoints::CONTACT_V3_JOB_LEVELS,
};

/// 更新职级请求体
///
/// 说明：字段均为可选，不传表示不更新。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateJobLevelBody {
    /// 职级名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 职级描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 排序值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// 状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    /// 国际化名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    /// 国际化描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<Vec<I18nContent>>,
}

/// 更新职级请求
pub struct UpdateJobLevelRequest {
    /// 配置信息。
    config: Config,
    /// 职级 ID。
    job_level_id: String,
}

impl UpdateJobLevelRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            job_level_id: String::new(),
        }
    }

    /// 职级 ID（路径参数）
    pub fn job_level_id(mut self, job_level_id: impl Into<String>) -> Self {
        self.job_level_id = job_level_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_level/update>
    pub async fn execute(self, body: UpdateJobLevelBody) -> SDKResult<JobLevelResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: UpdateJobLevelBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<JobLevelResponse> {
        validate_required!(self.job_level_id, "job_level_id 不能为空");

        // url: PUT:/open-apis/contact/v3/job_levels/:job_level_id
        let req: ApiRequest<JobLevelResponse> =
            ApiRequest::put(format!("{}/{}", CONTACT_V3_JOB_LEVELS, self.job_level_id))
                .body(serialize_params(&body, "更新职级")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "更新职级")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PUT /open-apis/contact/v3/job_levels/test001
    #[tokio::test]
    async fn test_update_job_level_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/contact/v3/job_levels/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "job_level": {} }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body: UpdateJobLevelBody = serde_json::from_value(json!({})).expect("body 构造");
        UpdateJobLevelRequest::new(config)
            .job_level_id("test001".to_string())
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
