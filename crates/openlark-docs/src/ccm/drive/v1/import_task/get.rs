//! 查询导入任务结果
//!
//! 获取导入任务的执行状态。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/import_task/get

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
    use openlark_core::testing::prelude::test_runtime;

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

    /// 测试 ticket 为空时的验证
    #[test]
    fn test_empty_ticket_validation() {
        let config = Config::default();
        let request = GetImportTaskRequest::new(config, "");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
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
}
