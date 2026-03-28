//! 通过过期时间获取发放记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/leave_employ_expire_record/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 通过过期时间获取发放记录请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 过期时间范围开始（Unix 时间戳，必填）
    expire_time_start: i64,
    /// 过期时间范围结束（Unix 时间戳，必填）
    expire_time_end: i64,
    /// 分页大小（可选，默认 50，最大 100）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config, expire_time_start: i64, expire_time_end: i64) -> Self {
        Self {
            expire_time_start,
            expire_time_end,
            page_size: None,
            page_token: None,
            config,
        }
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
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 构建端点
        let api_endpoint = AttendanceApiV1::LeaveEmployExpireRecordGet;
        let mut request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数
        request = request.query("expire_time_start", self.expire_time_start.to_string());
        request = request.query("expire_time_end", self.expire_time_end.to_string());
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "通过过期时间获取发放记录响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 通过过期时间获取发放记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 发放记录列表
    pub items: Vec<LeaveEmployExpireRecord>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 假期发放过期记录
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeaveEmployExpireRecord {
    /// 记录 ID
    pub record_id: String,
    /// 员工 ID
    pub employee_id: String,
    /// 假期类型 ID
    pub leave_type_id: String,
    /// 过期时间（Unix 时间戳）
    pub expire_time: i64,
    /// 剩余天数
    pub remaining_days: f64,
    /// 过期天数
    pub expire_days: f64,
}

impl ApiResponseTrait for GetResponse {
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
    fn test_get_request_builder_new() {
        let request = GetRequest::new(TestConfigBuilder::new().build(), 1, 1);
        let _ = request;
    }
}
