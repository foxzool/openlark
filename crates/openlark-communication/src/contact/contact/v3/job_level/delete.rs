//! 删除职级
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_level/delete>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{
    common::{api_utils::extract_response_data, models::EmptyData},
    endpoints::CONTACT_V3_JOB_LEVELS,
};

/// 删除职级请求
pub struct DeleteJobLevelRequest {
    /// 配置信息。
    config: Config,
    /// 职级 ID。
    job_level_id: String,
}

impl DeleteJobLevelRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_level/delete>
    pub async fn execute(self) -> SDKResult<EmptyData> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        validate_required!(self.job_level_id, "job_level_id 不能为空");

        // url: DELETE:/open-apis/contact/v3/job_levels/:job_level_id
        let req: ApiRequest<EmptyData> =
            ApiRequest::delete(format!("{}/{}", CONTACT_V3_JOB_LEVELS, self.job_level_id));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除职级")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：DELETE /open-apis/contact/v3/job_levels/test001
    #[tokio::test]
    async fn test_delete_job_level_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/contact/v3/job_levels/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        DeleteJobLevelRequest::new(config)
            .job_level_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
