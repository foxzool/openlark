//! 更新序列
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_family/update>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::job_family::models::{I18nContent, JobFamilyResponse},
    endpoints::CONTACT_V3_JOB_FAMILIES,
};

/// 更新序列请求体
///
/// 说明：字段均为可选，不传表示不更新。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateJobFamilyBody {
    /// 序列名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 序列描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 父序列 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_job_family_id: Option<String>,
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

/// 更新序列请求
pub struct UpdateJobFamilyRequest {
    /// 配置信息。
    config: Config,
    /// 序列 ID。
    job_family_id: String,
}

impl UpdateJobFamilyRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            job_family_id: String::new(),
        }
    }

    /// 序列 ID（路径参数）
    pub fn job_family_id(mut self, job_family_id: impl Into<String>) -> Self {
        self.job_family_id = job_family_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_family/update>
    pub async fn execute(self, body: UpdateJobFamilyBody) -> SDKResult<JobFamilyResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: UpdateJobFamilyBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<JobFamilyResponse> {
        validate_required!(self.job_family_id, "job_family_id 不能为空");

        // url: PUT:/open-apis/contact/v3/job_families/:job_family_id
        let req: ApiRequest<JobFamilyResponse> = ApiRequest::put(format!(
            "{}/{}",
            CONTACT_V3_JOB_FAMILIES, self.job_family_id
        ))
        .body(serialize_params(&body, "更新序列")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "更新序列")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PUT /open-apis/contact/v3/job_families/test001
    #[tokio::test]
    async fn test_update_job_family_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/contact/v3/job_families/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "job_family": {} }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body: UpdateJobFamilyBody = serde_json::from_value(json!({})).expect("body 构造");
        UpdateJobFamilyRequest::new(config)
            .job_family_id("test001".to_string())
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
