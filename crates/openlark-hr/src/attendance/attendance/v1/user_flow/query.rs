//! 批量查询打卡流水
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_flow/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list,
};

use super::models::{QueryUserFlowRequestBody, QueryUserFlowResponse};

/// 批量查询打卡流水请求
#[derive(Debug, Clone)]
pub struct QueryUserFlowRequest {
    /// 员工 ID 类型（必填，query 参数，可选值：employee_id、employee_no）
    employee_type: String,
    /// 查询的用户 ID 列表（必填，最多支持 50 个用户）
    user_ids: Vec<String>,
    /// 查询的起始时间（必填，包含，秒级时间戳）
    check_time_from: String,
    /// 查询的结束时间（必填，不包含，秒级时间戳）
    check_time_to: String,
    /// 是否包含离职员工数据（可选）
    include_terminated_user: Option<bool>,
    /// 配置信息
    config: Config,
}

impl QueryUserFlowRequest {
    /// 创建批量查询打卡流水请求
    pub fn new(config: Config) -> Self {
        Self {
            employee_type: String::new(),
            user_ids: Vec::new(),
            check_time_from: String::new(),
            check_time_to: String::new(),
            include_terminated_user: None,
            config,
        }
    }

    /// 设置员工 ID 类型（必填，query 参数）
    pub fn employee_type(mut self, employee_type: String) -> Self {
        self.employee_type = employee_type;
        self
    }

    /// 设置查询用户 ID 列表（必填）
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// 设置查询起始时间（必填，秒级时间戳）
    pub fn check_time_from(mut self, check_time_from: String) -> Self {
        self.check_time_from = check_time_from;
        self
    }

    /// 设置查询结束时间（必填，秒级时间戳）
    pub fn check_time_to(mut self, check_time_to: String) -> Self {
        self.check_time_to = check_time_to;
        self
    }

    /// 设置是否包含离职员工数据（可选）
    pub fn include_terminated_user(mut self, include_terminated_user: bool) -> Self {
        self.include_terminated_user = Some(include_terminated_user);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryUserFlowResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryUserFlowResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.employee_type, "employee_type 不能为空");
        validate_required_list!(self.user_ids, 50, "用户 ID 列表不能为空且不能超过 50 个");
        validate_required!(self.check_time_from.trim(), "查询起始时间不能为空");
        validate_required!(self.check_time_to.trim(), "查询结束时间不能为空");

        // 2. 构建端点并附加必填 query 参数 employee_type
        let api_endpoint = AttendanceApiV1::UserFlowQuery;
        let mut request = ApiRequest::<QueryUserFlowResponse>::post(api_endpoint.to_url())
            .query("employee_type", &self.employee_type);

        // 3. 添加可选查询参数
        if let Some(include_terminated_user) = self.include_terminated_user {
            request = request.query(
                "include_terminated_user",
                include_terminated_user.to_string(),
            );
        }

        // 4. 序列化请求体
        let request_body = QueryUserFlowRequestBody {
            user_ids: self.user_ids,
            check_time_from: self.check_time_from,
            check_time_to: self.check_time_to,
        };
        request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 5. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询打卡流水响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for QueryUserFlowResponse {
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
    fn test_query_user_flow_request_builder_new() {
        let request = QueryUserFlowRequest::new(TestConfigBuilder::new().build())
            .employee_type("employee_id".to_string())
            .check_time_from("1566641088".to_string());
        let _ = request;
    }

    #[test]
    fn test_query_user_flow_request_validation_fails_on_default_request() {
        let request = QueryUserFlowRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_user_flow_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"flow_list": [], "has_more": false}"#).unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/user_flows/query"))
            .and(query_param("employee_type", "employee_id"))
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

        let data = QueryUserFlowRequest::new(config)
            .employee_type("employee_id".to_string())
            .user_ids(vec!["id_001".to_string()])
            .check_time_from("1566641088".to_string())
            .check_time_to("1566727488".to_string())
            .execute()
            .await
            .expect("attendance_v1_user_flow_query 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_flows/query"
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(
            body.contains("\"check_time_from\""),
            "请求体缺 check_time_from: {body}"
        );
        assert!(
            body.contains("\"check_time_to\""),
            "请求体缺 check_time_to: {body}"
        );
        assert!(
            !body.contains("\"start_date\""),
            "请求体不应含旧字段 start_date: {body}"
        );
    }
}
