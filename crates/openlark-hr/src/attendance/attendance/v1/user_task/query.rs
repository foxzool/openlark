//! 查询打卡结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_task/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list,
};

use super::models::{QueryUserTaskRequestBody, QueryUserTaskResponse};

/// 查询打卡结果请求
#[derive(Debug, Clone)]
pub struct QueryUserTaskRequest {
    /// 员工 ID 类型（必填，query 参数，可选值：employee_id、employee_no）
    employee_type: String,
    /// 查询的用户 ID 列表（必填，最多支持 50 个用户）
    user_ids: Vec<String>,
    /// 查询的起始工作日（必填，格式为 `yyyyMMdd`）
    check_date_from: i32,
    /// 查询的结束工作日（必填，格式为 `yyyyMMdd`）
    check_date_to: i32,
    /// 是否忽略无效和没有权限的用户（可选）
    ignore_invalid_users: Option<bool>,
    /// 是否包含离职员工数据（可选）
    include_terminated_user: Option<bool>,
    /// 是否需要加班班段打卡结果（可选）
    need_overtime_result: Option<bool>,
    /// 配置信息
    config: Config,
}

impl QueryUserTaskRequest {
    /// 创建查询打卡结果请求
    pub fn new(config: Config) -> Self {
        Self {
            employee_type: String::new(),
            user_ids: Vec::new(),
            check_date_from: 0,
            check_date_to: 0,
            ignore_invalid_users: None,
            include_terminated_user: None,
            need_overtime_result: None,
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

    /// 设置查询起始工作日（必填，`yyyyMMdd`）
    pub fn check_date_from(mut self, check_date_from: i32) -> Self {
        self.check_date_from = check_date_from;
        self
    }

    /// 设置查询结束工作日（必填，`yyyyMMdd`）
    pub fn check_date_to(mut self, check_date_to: i32) -> Self {
        self.check_date_to = check_date_to;
        self
    }

    /// 设置是否忽略无效和没有权限的用户（可选）
    pub fn ignore_invalid_users(mut self, ignore_invalid_users: bool) -> Self {
        self.ignore_invalid_users = Some(ignore_invalid_users);
        self
    }

    /// 设置是否包含离职员工数据（可选）
    pub fn include_terminated_user(mut self, include_terminated_user: bool) -> Self {
        self.include_terminated_user = Some(include_terminated_user);
        self
    }

    /// 设置是否需要加班班段打卡结果（可选）
    pub fn need_overtime_result(mut self, need_overtime_result: bool) -> Self {
        self.need_overtime_result = Some(need_overtime_result);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryUserTaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryUserTaskResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.employee_type, "employee_type 不能为空");
        validate_required_list!(self.user_ids, 50, "用户 ID 列表不能为空且不能超过 50 个");
        if self.check_date_from <= 0 || self.check_date_to <= 0 {
            return Err(openlark_core::error::validation_error(
                "查询日期无效",
                "check_date_from / check_date_to 必须为 yyyyMMdd 格式的非零日期",
            ));
        }
        if self.check_date_to < self.check_date_from {
            return Err(openlark_core::error::validation_error(
                "日期范围无效",
                "check_date_to 不能早于 check_date_from",
            ));
        }

        // 2. 构建端点并附加必填 query 参数 employee_type
        let api_endpoint = AttendanceApiV1::UserTaskQuery;
        let mut request = ApiRequest::<QueryUserTaskResponse>::post(api_endpoint.to_url())
            .query("employee_type", &self.employee_type);

        // 3. 添加可选查询参数
        if let Some(ignore_invalid_users) = self.ignore_invalid_users {
            request = request.query("ignore_invalid_users", ignore_invalid_users.to_string());
        }
        if let Some(include_terminated_user) = self.include_terminated_user {
            request = request.query(
                "include_terminated_user",
                include_terminated_user.to_string(),
            );
        }

        // 4. 序列化请求体
        let request_body = QueryUserTaskRequestBody {
            user_ids: self.user_ids,
            check_date_from: self.check_date_from,
            check_date_to: self.check_date_to,
            need_overtime_result: self.need_overtime_result,
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
            "查询打卡结果响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for QueryUserTaskResponse {
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
    fn test_query_user_task_request_builder_new() {
        let request = QueryUserTaskRequest::new(TestConfigBuilder::new().build())
            .employee_type("employee_id".to_string())
            .check_date_from(20_190_817);
        let _ = request;
    }

    #[test]
    fn test_query_user_task_request_validation_fails_on_default_request() {
        let request = QueryUserTaskRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_user_task_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"records": [], "has_more": false}"#).unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/user_tasks/query"))
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

        let data = QueryUserTaskRequest::new(config)
            .employee_type("employee_id".to_string())
            .user_ids(vec!["id_001".to_string()])
            .check_date_from(20_240_101)
            .check_date_to(20_240_131)
            .need_overtime_result(true)
            .execute()
            .await
            .expect("attendance_v1_user_task_query 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_tasks/query"
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
            body.contains("\"need_overtime_result\""),
            "请求体缺 need_overtime_result: {body}"
        );
        assert!(
            !body.contains("\"start_date\""),
            "请求体不应含旧字段 start_date: {body}"
        );
        assert!(
            !body.contains("\"check_in_type\""),
            "请求体不应含旧字段 check_in_type: {body}"
        );
    }
}
