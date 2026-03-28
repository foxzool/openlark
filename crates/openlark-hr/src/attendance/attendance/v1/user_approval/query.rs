//! 获取审批数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_approval/query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取审批数据请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 用户 ID（可选）
    user_id: Option<String>,
    /// 审批类型（可选）
    approval_type: Option<i32>,
    /// 开始时间（Unix 时间戳，可选）
    start_time: Option<i64>,
    /// 结束时间（Unix 时间戳，可选）
    end_time: Option<i64>,
    /// 分页大小（可选，默认 50，最大 100）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            user_id: None,
            approval_type: None,
            start_time: None,
            end_time: None,
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置用户 ID（可选）
    pub fn user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// 设置审批类型（可选）
    pub fn approval_type(mut self, approval_type: i32) -> Self {
        self.approval_type = Some(approval_type);
        self
    }

    /// 设置开始时间（可选）
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// 设置结束时间（可选）
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// 设置分页大小（可选，默认 50，最大 100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 构建端点
        let api_endpoint = AttendanceApiV1::UserApprovalQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            user_id: self.user_id,
            approval_type: self.approval_type,
            start_time: self.start_time,
            end_time: self.end_time,
            page_size: self.page_size,
            page_token: self.page_token,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {}", e),
            )
        })?;
        let request = request.body(request_body_json);

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取审批数据响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取审批数据请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 用户 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 审批类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_type: Option<i32>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 获取审批数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 审批数据列表
    pub items: Vec<UserApproval>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 用户审批数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserApproval {
    /// 审批 ID
    pub approval_id: String,
    /// 用户 ID
    pub user_id: String,
    /// 审批类型
    pub approval_type: i32,
    /// 审批状态
    pub status: i32,
    /// 审批时间（Unix 时间戳）
    pub approval_time: i64,
    /// 审批内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
}

impl ApiResponseTrait for QueryResponse {
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
    fn test_query_request_builder_new() {
        let request =
            QueryRequest::new(TestConfigBuilder::new().build()).user_id("test".to_string());
        let _ = request;
    }
}
