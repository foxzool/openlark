//! 查询异步任务状态
//!
//! 查询异步任务状态，用于获取长时间运行任务的执行结果。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/file/async-task/task_check>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 查询异步任务状态请求
#[derive(Debug, Clone, Serialize)]
pub struct CheckTaskStatusRequest {
    #[serde(skip)]
    config: Config,
    /// 异步任务的 ID
    pub task_id: String,
}

impl CheckTaskStatusRequest {
    /// 创建新的请求实例
    pub fn new(config: Config, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
        }
    }

    /// 执行查询异步任务状态操作
    pub async fn execute(self) -> SDKResult<CheckTaskStatusResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CheckTaskStatusResponse> {
        // ===== 参数校验 =====
        if self.task_id.is_empty() {
            return Err(openlark_core::error::validation_error(
                "task_id",
                "task_id 不能为空",
            ));
        }

        // ===== 构建请求 =====
        let api_endpoint = DriveApi::TaskCheck;
        let api_request = ApiRequest::<CheckTaskStatusResponse>::get(&api_endpoint.to_url())
            .query("task_id", &self.task_id);

        // ===== 发送请求 =====
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "任务检查")
    }
}

/// 查询异步任务状态响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckTaskStatusResponse {
    /// 异步任务的执行状态。枚举值有：
    /// - success：任务执行成功
    /// - fail：任务执行失败
    /// - process：任务还在执行中
    pub status: String,
}

impl ApiResponseTrait for CheckTaskStatusResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_check_task_status_request_builder() {
        let config = Config::default();
        let request = CheckTaskStatusRequest::new(config, "task_123");

        assert_eq!(request.task_id, "task_123");
    }

    /// 测试响应数据结构
    #[test]
    fn test_task_check_status_response() {
        let data = CheckTaskStatusResponse {
            status: "success".to_string(),
        };

        assert_eq!(data.status, "success".to_string());
    }

    /// 测试响应trait实现
    #[test]
    fn test_response_trait_implementation() {
        assert_eq!(CheckTaskStatusResponse::data_format(), ResponseFormat::Data);
    }

    /// 测试process状态
    #[test]
    fn test_task_status_process() {
        let data = CheckTaskStatusResponse {
            status: "process".to_string(),
        };

        assert_eq!(data.status, "process");
    }

    /// 测试fail状态
    #[test]
    fn test_task_status_fail() {
        let data = CheckTaskStatusResponse {
            status: "fail".to_string(),
        };

        assert_eq!(data.status, "fail");
    }

    /// 测试空task_id场景
    #[test]
    fn test_empty_task_id() {
        let config = Config::default();
        let request = CheckTaskStatusRequest::new(config, "");

        assert_eq!(request.task_id, "");
    }
}
