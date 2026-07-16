//! 查询导出任务结果
//!
//! 获取导出任务的执行状态。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/export_task/get>

use openlark_core::{
    SDKResult,
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取导出任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExportTaskRequest {
    #[serde(skip)]
    /// SDK 配置。
    config: Config,
    /// 任务ticket
    pub ticket: String,
    /// 文件token
    pub token: String,
}

impl GetExportTaskRequest {
    /// 创建新的导出任务查询请求。
    pub fn new(config: Config, ticket: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            config,
            ticket: ticket.into(),
            token: token.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetExportTaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetExportTaskResponse> {
        validate_required!(self.ticket, "ticket 不能为空");
        let token_len = self.token.len();
        if token_len == 0 || token_len > 27 {
            return Err(openlark_core::error::validation_error(
                "token",
                "token 长度必须在 1~27 字节之间",
            ));
        }

        let api_endpoint = DriveApi::GetExportTask(self.ticket.clone());

        let api_request = api_endpoint
            .to_request::<GetExportTaskResponse>()
            .query("token", &self.token);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取")
    }
}

/// 获取导出任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExportTaskResponse {
    /// 导出任务结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ExportTaskResult>,
}

/// 导出任务结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTaskResult {
    /// 导出的文件的扩展名
    pub file_extension: String,
    /// 要导出的云文档类型
    pub r#type: String,
    /// 导出的文件名称
    pub file_name: Option<String>,
    /// 导出的文件 token（用于下载导出文件）
    pub file_token: Option<String>,
    /// 导出文件大小（字节）
    pub file_size: Option<i32>,
    /// 导出任务失败原因
    pub job_error_msg: Option<String>,
    /// 导出任务状态
    pub job_status: Option<i32>,
}

impl ApiResponseTrait for GetExportTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_get_export_task_request_builder() {
        let config = Config::default();
        let request = GetExportTaskRequest::new(config, "ticket", "token");
        assert_eq!(request.ticket, "ticket");
        assert_eq!(request.token, "token");
    }

    /// 测试响应格式
    #[test]
    fn test_response_trait() {
        assert_eq!(GetExportTaskResponse::data_format(), ResponseFormat::Data);
    }

    /// 测试 token 边界值
    #[test]
    fn test_token_boundaries() {
        let config = Config::default();

        // 1 字节（最小有效值）
        let request1 = GetExportTaskRequest::new(config.clone(), "ticket", "a");
        assert_eq!(request1.token.len(), 1);

        // 27 字节（最大有效值）
        let token27 = "a".repeat(27);
        let request2 = GetExportTaskRequest::new(config, "ticket", token27);
        assert_eq!(request2.token.len(), 27);
    }

    /// 端到端：GET .../export_tasks/{ticket}?token=... → 强类型 GetExportTaskResponse（单层 data 信封）。
    #[tokio::test]
    async fn test_get_export_task_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/drive/v1/export_tasks/ticket_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "result": {
                        "file_extension": "pdf",
                        "type": "docx",
                        "file_name": "导出文件.pdf",
                        "file_token": "ftk001",
                        "file_size": 1024,
                        "job_error_msg": "",
                        "job_status": 0
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

        let resp = GetExportTaskRequest::new(config, "ticket_001", "boxcnBj8N4yKVRhiMtDBTsXfQqb")
            .execute()
            .await
            .expect("查询导出任务应成功");
        let result = resp.result.expect("result 应非空");
        assert_eq!(result.file_extension, "pdf");
        assert_eq!(result.r#type, "docx");
        assert_eq!(result.file_token.as_deref(), Some("ftk001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/drive/v1/export_tasks/ticket_001"
        );
        assert!(
            received[0]
                .url
                .query()
                .unwrap_or("")
                .contains("token=boxcnBj8N4yKVRhiMtDBTsXfQqb")
        );
    }
}
