//! 获取单个职务信息
//!
//! docPath: <https://open.feishu.cn/document/contact-v3/job_title/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{
    contact::contact::v3::job_title::models::JobTitleResponse, endpoints::CONTACT_V3_JOB_TITLES,
};

/// 获取单个职务信息请求
pub struct GetJobTitleRequest {
    /// 配置信息。
    config: Config,
    /// 职务 ID。
    job_title_id: String,
}

impl GetJobTitleRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            job_title_id: String::new(),
        }
    }

    /// 职务 ID（路径参数）
    pub fn job_title_id(mut self, job_title_id: impl Into<String>) -> Self {
        self.job_title_id = job_title_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/contact-v3/job_title/get>
    pub async fn execute(self) -> SDKResult<JobTitleResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<JobTitleResponse> {
        validate_required!(self.job_title_id, "job_title_id 不能为空");

        // url: GET:/open-apis/contact/v3/job_titles/:job_title_id
        let req: ApiRequest<JobTitleResponse> =
            ApiRequest::get(format!("{}/{}", CONTACT_V3_JOB_TITLES, self.job_title_id));

        Transport::request_typed(req, &self.config, Some(option), "获取单个职务信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/contact/v3/job_titles/test001
    #[tokio::test]
    async fn test_get_job_title_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/contact/v3/job_titles/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "job_title": {} }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        GetJobTitleRequest::new(config)
            .job_title_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
