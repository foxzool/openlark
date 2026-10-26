//! 按 ID 查询考勤组
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/group/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::GetGroupResponse;

/// 按 ID 查询考勤组请求
#[derive(Debug, Clone)]
pub struct GetGroupRequest {
    /// 考勤组 ID（必填）
    group_id: String,
    /// 员工 ID 类型（必填，query 参数，可选值：employee_id、employee_no）
    employee_type: String,
    /// 部门 ID 的类型（必填，query 参数，暂时只支持 open_id）
    dept_type: String,
    /// 配置信息
    config: Config,
}

impl GetGroupRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            group_id: String::new(),
            employee_type: String::new(),
            dept_type: String::new(),
            config,
        }
    }

    /// 设置考勤组 ID（必填）
    pub fn group_id(mut self, group_id: String) -> Self {
        self.group_id = group_id;
        self
    }

    /// 设置员工 ID 类型（必填，query 参数）
    pub fn employee_type(mut self, employee_type: String) -> Self {
        self.employee_type = employee_type;
        self
    }

    /// 设置部门 ID 的类型（必填，query 参数）
    pub fn dept_type(mut self, dept_type: String) -> Self {
        self.dept_type = dept_type;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetGroupResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetGroupResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.group_id.trim(), "考勤组 ID 不能为空");
        validate_required!(self.employee_type, "employee_type 不能为空");
        validate_required!(self.dept_type, "dept_type 不能为空");

        // 2. 构建端点并附加必填 query 参数
        let api_endpoint = AttendanceApiV1::GroupGet(self.group_id.clone());
        let request = ApiRequest::<GetGroupResponse>::get(api_endpoint.to_url())
            .query("employee_type", &self.employee_type)
            .query("dept_type", &self.dept_type);

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取考勤组响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for GetGroupResponse {
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
    fn test_get_group_request_builder_new() {
        let request = GetGroupRequest::new(TestConfigBuilder::new().build())
            .group_id("test".to_string())
            .employee_type("employee_id".to_string())
            .dept_type("open_id".to_string());
        let _ = request;
    }

    #[test]
    fn test_get_group_request_validation_fails_on_default_request() {
        let request =
            GetGroupRequest::new(TestConfigBuilder::new().build()).group_id("g_1".to_string());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_group_get_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"group": {"group_id": "test", "group_name": "test", "group_type": 0}}"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/attendance/v1/groups/group_001"))
            .and(query_param("employee_type", "employee_id"))
            .and(query_param("dept_type", "open_id"))
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

        let data = GetGroupRequest::new(config)
            .group_id("group_001".to_string())
            .employee_type("employee_id".to_string())
            .dept_type("open_id".to_string())
            .execute()
            .await
            .expect("attendance_v1_group_get 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/groups/group_001"
        );
    }
}
