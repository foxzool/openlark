//! 查询导入任务结果
//!
//! 获取导入任务的执行状态。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/import_task/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取导入任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImportTaskRequest {
    #[serde(skip)]
    /// SDK 配置。
    config: Config,
    /// 任务ticket
    pub ticket: String,
}

impl GetImportTaskRequest {
    /// 创建新的导入任务查询请求。
    pub fn new(config: Config, ticket: impl Into<String>) -> Self {
        Self {
            config,
            ticket: ticket.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetImportTaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetImportTaskResponse> {
        // ===== 验证必填字段 =====
        validate_required!(self.ticket, "ticket 不能为空");

        let api_endpoint = DriveApi::GetImportTask(self.ticket.clone());

        let api_request = ApiRequest::<GetImportTaskResponse>::get(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取")
    }
}

/// 获取导入任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImportTaskResponse {
    /// 导入结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ImportTaskResult>,
}

/// 导入任务结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportTaskResult {
    /// 导入任务 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
    /// 导入的在线云文档类型（docx/sheet/bitable）
    pub r#type: String,
    /// 任务状态
    pub job_status: Option<i32>,
    /// 任务失败原因
    pub job_error_msg: Option<String>,
    /// 导入云文档的 token
    pub token: Option<String>,
    /// 导入云文档的 URL
    pub url: Option<String>,
    /// 导入成功的额外提示
    #[serde(default)]
    pub extra: Vec<String>,
}

impl ApiResponseTrait for GetImportTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_get_import_task_request_builder() {
        let config = Config::default();
        let request = GetImportTaskRequest::new(config, "ticket");
        assert_eq!(request.ticket, "ticket");
    }

    /// 测试响应格式
    #[test]
    fn test_response_trait() {
        assert_eq!(GetImportTaskResponse::data_format(), ResponseFormat::Data);
    }

    /// 测试 ticket 边界值
    #[test]
    fn test_ticket_boundaries() {
        let config = Config::default();

        // 单字符 ticket
        let request1 = GetImportTaskRequest::new(config.clone(), "a");
        assert_eq!(request1.ticket, "a");

        // 长 ticket
        let long_ticket = "a".repeat(100);
        let request2 = GetImportTaskRequest::new(config, long_ticket);
        assert_eq!(request2.ticket.len(), 100);
    }

    /// 端到端：GET /open-apis/drive/v1/import_tasks/{ticket} → GetImportTaskResponse（result）。
    #[tokio::test]
    async fn test_get_import_task_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/drive/v1/import_tasks/import_ticket_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "result": {
                        "ticket": "import_ticket_001",
                        "type": "docx",
                        "job_status": 0,
                        "token": "doccnXxXxXxXxXxXxXxXxXxXxXxXxXxX",
                        "url": "https://example.feishu.cn/docx/doccnXxXxXxXxXxXxXxXxXxXxXxXxXxX"
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetImportTaskRequest::new(config, "import_ticket_001")
            .execute()
            .await
            .expect("查询导入任务应成功");
        let result = resp.result.expect("响应应包含 result");
        assert_eq!(result.ticket.as_deref(), Some("import_ticket_001"));
        assert_eq!(result.r#type, "docx");
        assert_eq!(result.job_status, Some(0));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/drive/v1/import_tasks/import_ticket_001"
        );
    }
}
