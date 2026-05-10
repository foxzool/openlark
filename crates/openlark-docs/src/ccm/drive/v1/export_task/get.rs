//! 查询导出任务结果
//!
//! 获取导出任务的执行状态。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/export_task/get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
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
        // ===== 验证必填字段 =====
        validate_required!(self.ticket, "ticket 不能为空");
        // ===== 验证字段长度 =====
        let token_len = self.token.len();
        if token_len == 0 || token_len > 27 {
            return Err(openlark_core::error::validation_error(
                "token",
                "token 长度必须在 1~27 字节之间",
            ));
        }

        let api_endpoint = DriveApi::GetExportTask(self.ticket.clone());

        let api_request = ApiRequest::<GetExportTaskResponse>::get(&api_endpoint.to_url())
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
    use openlark_core::testing::prelude::test_runtime;

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

    /// 测试 ticket 为空时的验证
    #[test]
    fn test_empty_ticket_validation() {
        let config = Config::default();
        let request = GetExportTaskRequest::new(config, "", "token");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 token 长度验证
    #[test]
    fn test_token_length_validation() {
        let config = Config::default();

        // 空字符串
        let request1 = GetExportTaskRequest::new(config.clone(), "ticket", "");

        let result1 = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request1.execute().await;
            })
        })
        .join();

        assert!(result1.is_ok());

        // 超过 27 字节
        let long_token = "a".repeat(28);
        let request2 = GetExportTaskRequest::new(config, "ticket", long_token);

        let result2 = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request2.execute().await;
            })
        })
        .join();

        assert!(result2.is_ok());
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
}

