//! 查询考勤组下所有成员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/group/list_user>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询考勤组下所有成员请求
#[derive(Debug, Clone)]
pub struct ListUserRequest {
    /// 考勤组 ID（必填，路径参数）
    group_id: String,
    /// 员工 ID 类型（必填，query 参数）：employee_id / employee_no
    employee_type: String,
    /// 部门 ID 的类型（必填，query 参数）：open_id
    dept_type: String,
    /// 查询的考勤组成员的打卡类型（必填）：0-全部打卡类型，1-需要打卡类型，2-无需打卡类型
    member_clock_type: i32,
    /// 分页大小（可选，默认 10，取值范围 1-50）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListUserRequest {
    /// 创建请求
    ///
    /// - `group_id`: 考勤组 ID（路径参数，必填）
    /// - `employee_type`: 员工 ID 类型（query 参数，必填）
    /// - `dept_type`: 部门 ID 的类型（query 参数，必填，暂只支持 open_id）
    /// - `member_clock_type`: 查询的考勤组成员的打卡类型（query 参数，必填）
    pub fn new(
        config: Config,
        group_id: String,
        employee_type: String,
        dept_type: String,
        member_clock_type: i32,
    ) -> Self {
        Self {
            group_id,
            employee_type,
            dept_type,
            member_clock_type,
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置分页大小（可选，默认 10，取值范围 1-50）
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
    pub async fn execute(self) -> SDKResult<ListUserResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListUserResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.group_id.trim(), "group_id");
        validate_required!(self.employee_type, "employee_type 不能为空");
        validate_required!(self.dept_type, "dept_type 不能为空");

        // 2. 构建端点并附加 query 参数（employee_type/dept_type/member_clock_type 必填）
        let api_endpoint = AttendanceApiV1::GroupListUser(self.group_id.clone());
        let mut request = ApiRequest::<ListUserResponse>::get(api_endpoint.to_url())
            .query("employee_type", &self.employee_type)
            .query("dept_type", &self.dept_type)
            .query("member_clock_type", self.member_clock_type.to_string());

        // 3. 添加可选查询参数
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询考勤组下所有成员响应数据为空",
        )
        .await
    }
}

/// 查询考勤组下所有成员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListUserResponse {
    /// 考勤组成员列表（user_base[]）
    #[serde(default)]
    pub users: Vec<GroupMember>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 考勤组成员（官方 `user_base`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupMember {
    /// 用户 ID，对应 employee_type
    pub user_id: String,
    /// 该用户所属部门 ID 列表（该用户所属的部门树，从直属部门到根部门，对应 dept_type）
    #[serde(default)]
    pub department_ids: Vec<String>,
}

impl ApiResponseTrait for ListUserResponse {
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
    fn test_list_user_request_builder_new() {
        let request = ListUserRequest::new(
            TestConfigBuilder::new().build(),
            "test".to_string(),
            "employee_id".to_string(),
            "open_id".to_string(),
            1,
        );
        let _ = request;
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_group_list_user_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.users[]（user_id + department_ids）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"users": [{"user_id": "5874663B", "department_ids": ["od-fcb45c28a45311afd440b7869541fce8"]}], "page_token": "92xKiuWrBCSweSZJBSTpag%3D%3D", "has_more": false}"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/attendance/v1/groups/group_001/list_user"))
            .and(query_param("employee_type", "employee_id"))
            .and(query_param("dept_type", "open_id"))
            .and(query_param("member_clock_type", "1"))
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

        let data = ListUserRequest::new(
            config,
            "group_001".to_string(),
            "employee_id".to_string(),
            "open_id".to_string(),
            1,
        )
        .execute()
        .await
        .expect("attendance_v1_group_list_user 应成功");

        assert_eq!(data.users.len(), 1);
        assert_eq!(data.users[0].user_id, "5874663B");
        assert_eq!(
            data.users[0].department_ids,
            vec!["od-fcb45c28a45311afd440b7869541fce8"]
        );
        assert!(!data.has_more);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/groups/group_001/list_user"
        );
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("employee_type=employee_id"),
            "缺必填 query 参数 employee_type: {query}"
        );
        assert!(
            query.contains("dept_type=open_id"),
            "缺必填 query 参数 dept_type: {query}"
        );
        assert!(
            query.contains("member_clock_type=1"),
            "缺必填 query 参数 member_clock_type: {query}"
        );
    }
}
