//! 列出自动化流程
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-workflow/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::BitableApiV1;

/// 自动化流程信息（app.workflow）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// 自动化流程的 ID
    pub workflow_id: String,
    /// 自动化流程的状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 自动化流程的名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// 列出自动化流程响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWorkflowResponse {
    /// 自动化流程信息
    pub workflows: Vec<Workflow>,
}

impl ApiResponseTrait for ListWorkflowResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出自动化流程请求
///
/// 用于分页查询多维表格中的自动化流程。
pub struct ListWorkflowRequest {
    config: Config,
    app_token: String,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl ListWorkflowRequest {
    /// 创建新的自动化流程列表请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            page_token: None,
            page_size: None,
        }
    }

    /// 多维表格 app_token（路径参数）
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 分页大小（默认 20）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListWorkflowResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListWorkflowResponse> {
        validate_required!(self.app_token, "app_token 不能为空");

        let api_endpoint = BitableApiV1::WorkflowList(self.app_token);
        // #439: method 来自 catalog
        let mut api_request: ApiRequest<ListWorkflowResponse> = api_endpoint.to_request();
        api_request = api_request.query_opt("page_token", self.page_token);
        api_request = api_request.query_opt("page_size", self.page_size.map(|v| v.to_string()));

        Transport::request_typed(api_request, &self.config, Some(option), "列出自动化流程").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../workflows → ListWorkflowResponse。
    #[tokio::test]
    async fn test_list_workflow_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/bitable/v1/apps/app001/workflows"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "workflows": [] }
            })))
            .mount(&server)
            .await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        ListWorkflowRequest::new(config)
            .app_token("app001")
            .execute()
            .await
            .expect("列出工作流应成功");
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/bitable/v1/apps/app001/workflows"
        );
    }
}
