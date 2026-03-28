//! 查询排班表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_daily_shift/query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 查询排班表请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 用户 ID 列表（可选）
    user_ids: Option<Vec<String>>,
    /// 开始日期（Unix 时间戳，必填）
    start_date: i64,
    /// 结束日期（Unix 时间戳，必填）
    end_date: i64,
    /// 分页大小（可选，默认 50，最大 100）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config, start_date: i64, end_date: i64) -> Self {
        Self {
            user_ids: None,
            start_date,
            end_date,
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置用户 ID 列表（可选）
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
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
        let api_endpoint = AttendanceApiV1::UserDailyShiftQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            user_ids: self.user_ids,
            start_date: self.start_date,
            end_date: self.end_date,
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
                "查询排班表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询排班表请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 用户 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 开始日期
    pub start_date: i64,
    /// 结束日期
    pub end_date: i64,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 查询排班表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 排班记录列表
    pub items: Vec<UserDailyShift>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 用户每日排班记录
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserDailyShift {
    /// 用户 ID
    pub user_id: String,
    /// 排班日期（Unix 时间戳）
    pub date: i64,
    /// 班次 ID
    pub shift_id: String,
    /// 班次名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_name: Option<String>,
    /// 工作时长（小时）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_hours: Option<f64>,
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
        let request = QueryRequest::new(TestConfigBuilder::new().build(), 1, 1);
        let _ = request;
    }
}
