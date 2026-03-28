//! 通知审批状态更新
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/approval_info/process>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 通知审批状态更新请求
#[derive(Debug, Clone)]
pub struct ProcessRequest {
    /// 审批实例 ID（必填）
    approval_instance_id: String,
    /// 审批结果（必填）
    result: i32,
    /// 审批意见（可选）
    comment: Option<String>,
    /// 配置信息
    config: Config,
}

impl ProcessRequest {
    /// 创建请求
    pub fn new(config: Config, approval_instance_id: String, result: i32) -> Self {
        Self {
            approval_instance_id,
            result,
            comment: None,
            config,
        }
    }

    /// 设置审批意见（可选）
    pub fn comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ProcessResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ProcessResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.approval_instance_id.trim(), "approval_instance_id");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::ApprovalInfoProcess;
        let request = ApiRequest::<ProcessResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = ProcessRequestBody {
            approval_instance_id: self.approval_instance_id,
            result: self.result,
            comment: self.comment,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {}", e),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "通知审批状态更新响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 通知审批状态更新请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRequestBody {
    /// 审批实例 ID
    pub approval_instance_id: String,
    /// 审批结果
    pub result: i32,
    /// 审批意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 通知审批状态更新响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessResponse {
    /// 是否成功
    pub success: bool,
    /// 审批实例 ID
    pub approval_instance_id: String,
}

impl ApiResponseTrait for ProcessResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_process_request_builder_new() {
        let request = ProcessRequest::new(TestConfigBuilder::new().build(), "test".to_string(), 1);
        let _ = request;
    }
}
