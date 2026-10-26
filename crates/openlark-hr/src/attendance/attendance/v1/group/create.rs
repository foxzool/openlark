//! 创建或修改考勤组
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/group/create>
//!
//! 官方请求体为嵌套结构 `{"group": {...}, "operator_id": "..."}`，
//! 必填 query 参数 `employee_type` 与 `dept_type`。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::{AttendanceGroup, CreateGroupRequestBody, CreateGroupResponse};

/// 创建考勤组请求
#[derive(Debug, Clone)]
pub struct CreateGroupRequest {
    /// 员工 ID 类型（必填，query 参数）：employee_id / employee_no
    employee_type: String,
    /// 部门 ID 的类型（必填，query 参数）：open_id
    dept_type: String,
    /// 考勤组信息（必填）
    group: AttendanceGroup,
    /// 操作人 uid，对应 employee_type（未操作考勤管理后台「API 接入」流程时必填）
    operator_id: Option<String>,
    /// 配置信息
    config: Config,
}

impl CreateGroupRequest {
    /// 创建请求
    ///
    /// - `employee_type`: 员工 ID 类型（query 参数，必填）
    /// - `dept_type`: 部门 ID 的类型（query 参数，必填，暂只支持 open_id）
    /// - `group`: 考勤组信息（含官方必填字段）
    pub fn new(
        config: Config,
        employee_type: String,
        dept_type: String,
        group: AttendanceGroup,
    ) -> Self {
        Self {
            employee_type,
            dept_type,
            group,
            operator_id: None,
            config,
        }
    }

    /// 设置操作人 uid，对应 employee_type（未操作考勤管理后台「API 接入」流程时必填）
    pub fn operator_id(mut self, operator_id: String) -> Self {
        self.operator_id = Some(operator_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateGroupResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateGroupResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.employee_type, "employee_type 不能为空");
        validate_required!(self.dept_type, "dept_type 不能为空");
        validate_required!(self.group.group_name, "考勤组名称不能为空");
        validate_required!(self.group.time_zone, "time_zone 不能为空");
        validate_required!(self.group.group_leader_ids, "group_leader_ids 不能为空");

        // 2. 构建端点并附加必填 query 参数 employee_type / dept_type
        let api_endpoint = AttendanceApiV1::GroupCreate;
        let request = ApiRequest::<CreateGroupResponse>::post(api_endpoint.to_url())
            .query("employee_type", &self.employee_type)
            .query("dept_type", &self.dept_type);

        // 3. 序列化请求体（官方结构 {"group": {...}, "operator_id": "..."}）
        let request_body = CreateGroupRequestBody {
            group: self.group,
            operator_id: self.operator_id,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建考勤组响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for CreateGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use serde_json::json;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    fn sample_group() -> AttendanceGroup {
        let mut group = AttendanceGroup::new(
            "开心考勤".to_string(),
            "Asia/Shanghai".to_string(),
            vec!["2bg4a9be".to_string()],
            1,
            vec!["xxx".to_string(), "0".to_string()],
            1,
        );
        group.bind_dept_ids = Some(vec!["od-fcb45c28a45311afd440b7869541fce8".to_string()]);
        group.allow_out_punch = Some(true);
        group
    }

    #[test]
    fn test_create_group_request_builder() {
        let request = CreateGroupRequest::new(
            create_test_config(),
            "employee_id".to_string(),
            "open_id".to_string(),
            sample_group(),
        )
        .operator_id("dd31248a".to_string());

        assert_eq!(request.employee_type, "employee_id");
        assert_eq!(request.dept_type, "open_id");
        assert_eq!(request.operator_id.as_deref(), Some("dd31248a"));
        assert_eq!(request.group.group_name, "开心考勤");
    }

    #[test]
    fn test_create_group_request_body_serialize() {
        let group = sample_group();
        let body = CreateGroupRequestBody {
            group,
            operator_id: Some("dd31248a".to_string()),
        };

        let value = serde_json::to_value(&body).expect("序列化请求体失败");
        // 官方结构：顶层 group + operator_id，考勤组字段在 group 内
        assert!(value["group"].is_object(), "请求体缺 group 对象: {value}");
        assert_eq!(value["group"]["group_name"], json!("开心考勤"));
        assert_eq!(value["group"]["time_zone"], json!("Asia/Shanghai"));
        assert_eq!(value["group"]["group_leader_ids"], json!(["2bg4a9be"]));
        assert_eq!(value["group"]["group_type"], json!(1));
        assert_eq!(value["group"]["calendar_id"], json!(1));
        assert_eq!(value["operator_id"], json!("dd31248a"));
        // 可选字段不序列化
        assert!(
            value["group"].get("except_user_ids").is_none(),
            "未设置的 optional 字段不应序列化: {value}"
        );
    }

    #[test]
    fn test_create_group_response_deserialize() {
        let value = json!({"group_id": "grp_123"});
        let response: CreateGroupResponse =
            serde_json::from_value(value).expect("反序列化响应失败");
        assert_eq!(response.group_id, "grp_123");
    }

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_group_create_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/groups"))
            .and(query_param("employee_type", "employee_id"))
            .and(query_param("dept_type", "open_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {"group_id": "test"}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = CreateGroupRequest::new(
            config,
            "employee_id".to_string(),
            "open_id".to_string(),
            sample_group(),
        )
        .execute()
        .await
        .expect("attendance_v1_group_create 应成功");

        assert_eq!(data.group_id, "test");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/attendance/v1/groups");
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("employee_type=employee_id"),
            "缺必填 query 参数 employee_type: {query}"
        );
        assert!(
            query.contains("dept_type=open_id"),
            "缺必填 query 参数 dept_type: {query}"
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(body.contains("\"group\""), "请求体缺 group 对象: {body}");
        assert!(
            body.contains("\"group_leader_ids\""),
            "请求体缺 group.group_leader_ids: {body}"
        );
        assert!(
            !body.contains("\"manager_list\""),
            "请求体不应含旧字段 manager_list: {body}"
        );
    }
}
