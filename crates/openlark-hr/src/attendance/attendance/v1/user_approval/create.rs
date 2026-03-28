//! 写入审批结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_approval/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 写入审批结果请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 用户 ID（必填）
    user_id: String,
    /// 审批类型（必填）
    approval_type: i32,
    /// 审批结果（必填）
    result: i32,
    /// 审批时间（Unix 时间戳，必填）
    approval_time: i64,
    /// 审批备注（可选）
    remark: Option<String>,
    /// 配置信息
    config: Config,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(
        config: Config,
        user_id: String,
        approval_type: i32,
        result: i32,
        approval_time: i64,
    ) -> Self {
        Self {
            user_id,
            approval_type,
            result,
            approval_time,
            remark: None,
            config,
        }
    }

    /// 设置审批备注（可选）
    pub fn remark(mut self, remark: String) -> Self {
        self.remark = Some(remark);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.user_id.trim(), "user_id");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserApprovalCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = CreateRequestBody {
            user_id: self.user_id,
            approval_type: self.approval_type,
            result: self.result,
            approval_time: self.approval_time,
            remark: self.remark,
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
                "写入审批结果响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 写入审批结果请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 用户 ID
    pub user_id: String,
    /// 审批类型
    pub approval_type: i32,
    /// 审批结果
    pub result: i32,
    /// 审批时间
    pub approval_time: i64,
    /// 审批备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

/// 写入审批结果响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 是否成功
    pub success: bool,
    /// 审批记录 ID
    pub approval_id: String,
}

impl ApiResponseTrait for CreateResponse {
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
    fn test_create_request_builder_new() {
        let request = CreateRequest::new(
            TestConfigBuilder::new().build(),
            "test".to_string(),
            1,
            1,
            1,
        );
        let _ = request;
    }
}
