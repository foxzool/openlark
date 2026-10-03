//! 查询排班表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_daily_shift/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 查询排班表请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 用户 ID 列表（必填，最多 50 人）
    user_ids: Vec<String>,
    /// 查询起始工作日（必填，格式 `yyyyMMdd`）
    check_date_from: i32,
    /// 查询结束工作日（必填，格式 `yyyyMMdd`）
    check_date_to: i32,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    ///
    /// - `user_ids`: 用户 ID 列表（最多 50 人）
    /// - `check_date_from`: 起始工作日（`yyyyMMdd`）
    /// - `check_date_to`: 结束工作日（`yyyyMMdd`）
    pub fn new(
        config: Config,
        user_ids: Vec<String>,
        check_date_from: i32,
        check_date_to: i32,
    ) -> Self {
        Self {
            user_ids,
            check_date_from,
            check_date_to,
            config,
        }
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

        // 1. 验证必填字段
        validate_required_list!(self.user_ids, 50, "user_ids 不能为空且不能超过 50 个");
        if self.check_date_to < self.check_date_from {
            return Err(openlark_core::error::validation_error(
                "日期范围无效",
                "check_date_to 不能早于 check_date_from",
            ));
        }

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserDailyShiftQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = QueryRequestBody {
            user_ids: self.user_ids,
            check_date_from: self.check_date_from,
            check_date_to: self.check_date_to,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询排班表响应数据为空",
        )
        .await
    }
}

/// 查询排班表请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 用户 ID 列表
    pub user_ids: Vec<String>,
    /// 查询起始工作日（`yyyyMMdd`）
    pub check_date_from: i32,
    /// 查询结束工作日（`yyyyMMdd`）
    pub check_date_to: i32,
}

/// 查询排班表响应
///
/// 官网 response `data.user_daily_shifts` 为数组，items schema 未完整给出，透传 `Value`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 排班记录列表
    #[serde(default)]
    pub user_daily_shifts: Vec<serde_json::Value>,
}

impl ApiResponseTrait for QueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_query_request_builder_new() {
        let request = QueryRequest::new(
            TestConfigBuilder::new().build(),
            vec!["abd754f7".to_string()],
            20_190_817,
            20_190_820,
        );
        let _ = request;
    }

    /// 端到端：Builder→execute→Transport→mock→assert 请求体字段对齐飞书官网 schema。
    #[tokio::test]
    async fn test_attendance_v1_user_daily_shift_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/user_daily_shifts/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "user_daily_shifts": [] }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = QueryRequest::new(config, vec!["abd754f7".to_string()], 20_190_817, 20_190_820)
            .execute()
            .await
            .expect("attendance_v1_user_daily_shift_query 应成功");

        assert!(data.user_daily_shifts.is_empty());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_daily_shifts/query"
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(
            body.contains("\"check_date_from\""),
            "请求体缺 check_date_from: {body}"
        );
        assert!(
            body.contains("\"check_date_to\""),
            "请求体缺 check_date_to: {body}"
        );
        assert!(
            !body.contains("\"start_date\""),
            "请求体不应含旧字段 start_date: {body}"
        );
    }
}
