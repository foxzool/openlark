//! 获取审批数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_approval/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 获取审批数据请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 用户 ID 列表（必填）
    user_ids: Vec<String>,
    /// 查询起始日期 `yyyyMMdd`（必填，不能超过当天 +1）
    check_date_from: i32,
    /// 查询结束日期 `yyyyMMdd`（必填，与 `check_date_from` 间隔不超过 30 天）
    check_date_to: i32,
    /// 查询依据的时间类型（可选，默认 `PeriodTime`）
    check_date_type: Option<String>,
    /// 查询状态（可选）
    status: Option<i32>,
    /// 查询起始时间（可选）
    check_time_from: Option<String>,
    /// 查询结束时间（可选）
    check_time_to: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
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
            check_date_type: None,
            status: None,
            check_time_from: None,
            check_time_to: None,
            config,
        }
    }

    /// 设置查询依据的时间类型（可选）
    pub fn check_date_type(mut self, check_date_type: String) -> Self {
        self.check_date_type = Some(check_date_type);
        self
    }

    /// 设置查询状态（可选）
    pub fn status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置查询起始时间（可选）
    pub fn check_time_from(mut self, check_time_from: String) -> Self {
        self.check_time_from = Some(check_time_from);
        self
    }

    /// 设置查询结束时间（可选）
    pub fn check_time_to(mut self, check_time_to: String) -> Self {
        self.check_time_to = Some(check_time_to);
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

        // 1. 验证必填字段
        validate_required_list!(self.user_ids, 50, "user_ids 不能为空且不能超过 50 个");
        if self.check_date_to < self.check_date_from {
            return Err(openlark_core::error::validation_error(
                "日期范围无效",
                "check_date_to 不能早于 check_date_from",
            ));
        }

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserApprovalQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = QueryRequestBody {
            user_ids: self.user_ids,
            check_date_from: self.check_date_from,
            check_date_to: self.check_date_to,
            check_date_type: self.check_date_type,
            status: self.status,
            check_time_from: self.check_time_from,
            check_time_to: self.check_time_to,
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
            "获取审批数据响应数据为空",
        )
        .await
    }
}

/// 获取审批数据请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 用户 ID 列表
    pub user_ids: Vec<String>,
    /// 查询起始日期 `yyyyMMdd`
    pub check_date_from: i32,
    /// 查询结束日期 `yyyyMMdd`
    pub check_date_to: i32,
    /// 查询依据的时间类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_date_type: Option<String>,
    /// 查询状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 查询起始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_time_from: Option<String>,
    /// 查询结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_time_to: Option<String>,
}

/// 获取审批数据响应
///
/// 官网 response `data.user_approvals` 为数组，items schema 未完整给出，透传 `Value`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 审批数据列表
    #[serde(default)]
    pub user_approvals: Vec<serde_json::Value>,
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
    async fn test_attendance_v1_user_approval_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/user_approvals/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "user_approvals": [] }
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
            .expect("attendance_v1_user_approval_query 应成功");

        assert!(data.user_approvals.is_empty());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_approvals/query"
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(
            body.contains("\"check_date_from\""),
            "请求体缺 check_date_from: {body}"
        );
        assert!(body.contains("\"user_ids\""), "请求体缺 user_ids: {body}");
        assert!(
            !body.contains("\"start_time\""),
            "请求体不应含旧字段 start_time: {body}"
        );
    }
}
