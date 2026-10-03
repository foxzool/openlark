//! 获取租户职级列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_level/list>

use openlark_core::{SDKResult, api::ApiRequest, config::Config, http::Transport};

use crate::{
    contact::contact::v3::job_level::models::ListJobLevelsResponse,
    endpoints::CONTACT_V3_JOB_LEVELS,
};

/// 获取租户职级列表请求
pub struct ListJobLevelsRequest {
    /// 配置信息。
    config: Config,
    /// 分页大小。
    page_size: Option<i32>,
    /// 分页标记。
    page_token: Option<String>,
    /// 职级名称。
    name: Option<String>,
}

impl ListJobLevelsRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            name: None,
        }
    }

    /// 分页大小（查询参数，可选，默认 10，范围 1~50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 职级名称（查询参数，可选，精确匹配）
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_level/list>
    pub async fn execute(self) -> SDKResult<ListJobLevelsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListJobLevelsResponse> {
        let mut req: ApiRequest<ListJobLevelsResponse> = ApiRequest::get(CONTACT_V3_JOB_LEVELS);

        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(name) = self.name {
            req = req.query("name", name);
        }
        Transport::request_typed(req, &self.config, Some(option), "获取租户职级列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/contact/v3/job_levels
    #[tokio::test]
    async fn test_list_job_levels_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/contact/v3/job_levels"))
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

        ListJobLevelsRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
