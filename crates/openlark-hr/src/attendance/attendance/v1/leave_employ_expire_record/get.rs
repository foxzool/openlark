//! 通过过期时间获取发放记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/leave_employ_expire_record/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
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

    fn validate(&self) -> SDKResult<()> {
        if self.expire_time_start <= 0 {
            return Err(openlark_core::error::validation_error(
                "expire_time_start 无效",
                "expire_time_start 必须为正整数时间戳",
            ));
        }
        if self.expire_time_end <= 0 {
            return Err(openlark_core::error::validation_error(
                "expire_time_end 无效",
                "expire_time_end 必须为正整数时间戳",
            ));
        }
        if self.expire_time_end < self.expire_time_start {
            return Err(openlark_core::error::validation_error(
                "时间范围无效",
                "expire_time_end 不能早于 expire_time_start",
            ));
        }
        Ok(())
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

        self.validate()?;

        // 1. 构建端点
        let record_key = format!("{}-{}", self.expire_time_start, self.expire_time_end);
        let api_endpoint = AttendanceApiV1::LeaveEmployExpireRecordGet(record_key).to_url();
        let mut request = ApiRequest::<GetResponse>::get(&api_endpoint);

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
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "通过过期时间获取发放记录响应数据为空",
        )
        .await
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
    use openlark_core::config::Config;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_get_request_builder_new() {
        let request = GetRequest::new(TestConfigBuilder::new().build(), 1, 1);
        let _ = request;
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_leave_employ_expire_record_get_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"items": [], "has_more": false}"#).unwrap();
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/attendance/v1/leave_employ_expire_records/1700000000-1700000000",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = GetRequest::new(config, 1_700_000_000, 1_700_000_000)
            .execute()
            .await
            .expect("attendance_v1_leave_employ_expire_record_get 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/leave_employ_expire_records/1700000000-1700000000"
        );
    }
}
