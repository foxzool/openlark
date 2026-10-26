//! 获取可补卡时间
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_task_remedy/query_user_allowed_remedys>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 获取可补卡时间请求
#[derive(Debug, Clone)]
pub struct QueryUserAllowedRemedysRequest {
    /// 员工 ID 类型（必填，query 参数，可选值：employee_id、employee_no）
    employee_type: String,
    /// 用户 ID（必填，对应 employee_type）
    user_id: String,
    /// 补卡日期（必填，格式为 yyyyMMdd）
    remedy_date: i32,
    /// 配置信息
    config: Config,
}

impl QueryUserAllowedRemedysRequest {
    /// 创建请求
    ///
    /// - `employee_type`: 员工 ID 类型（query 参数）
    /// - `user_id`: 用户 ID
    /// - `remedy_date`: 补卡日期（`yyyyMMdd`）
    pub fn new(config: Config, employee_type: String, user_id: String, remedy_date: i32) -> Self {
        Self {
            employee_type,
            user_id,
            remedy_date,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryUserAllowedRemedysResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryUserAllowedRemedysResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.employee_type, "employee_type 不能为空");
        validate_required!(self.user_id.trim(), "user_id 不能为空");

        // 2. 构建端点并附加必填 query 参数 employee_type
        let api_endpoint = AttendanceApiV1::UserTaskRemedyQueryUserAllowedRemedys;
        let request = ApiRequest::<QueryUserAllowedRemedysResponse>::post(api_endpoint.to_url())
            .query("employee_type", &self.employee_type);

        // 3. 构建请求体
        let request_body = QueryUserAllowedRemedysRequestBody {
            user_id: self.user_id,
            remedy_date: self.remedy_date,
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
            "获取可补卡时间响应数据为空",
        )
        .await
    }
}

/// 获取可补卡时间请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserAllowedRemedysRequestBody {
    /// 用户 ID，对应 employee_type
    pub user_id: String,
    /// 补卡日期，格式为 yyyyMMdd
    pub remedy_date: i32,
}

/// 获取可补卡时间响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryUserAllowedRemedysResponse {
    /// 用户可补卡时间列表（user_allowed_remedy[]）
    #[serde(default)]
    pub user_allowed_remedys: Vec<UserAllowedRemedy>,
}

/// 用户可补卡时间（官方 `user_allowed_remedy`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAllowedRemedy {
    /// 用户 ID，对应 employee_type
    pub user_id: String,
    /// 补卡日期，格式为 yyyyMMdd
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy_date: Option<i64>,
    /// 是否为自由班次（自由班次不用选择第几次上下班，直接选择补卡时间即可）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_free_punch: Option<bool>,
    /// 第几次上下班：0 第 1 次、1 第 2 次、2 第 3 次
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_no: Option<i32>,
    /// 上班/下班：1 上班、2 下班
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_type: Option<i32>,
    /// 打卡状态：Early 早退、Late 迟到、Lack 缺卡
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_status: Option<String>,
    /// 正常的应打卡时间（yyyy-MM-dd HH:mm）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal_punch_time: Option<String>,
    /// 可选的补卡时间的最小值（yyyy-MM-dd HH:mm）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy_start_time: Option<String>,
    /// 可选的补卡时间的最大值（yyyy-MM-dd HH:mm）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy_end_time: Option<String>,
}

impl ApiResponseTrait for QueryUserAllowedRemedysResponse {
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
    fn test_query_user_allowed_remedys_request_builder_new() {
        let request = QueryUserAllowedRemedysRequest::new(
            TestConfigBuilder::new().build(),
            "employee_id".to_string(),
            "test".to_string(),
            20_210_104,
        );
        let _ = request;
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_user_task_remedy_query_user_allowed_remedys_returns_data_on_success()
     {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.user_allowed_remedys[]
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"user_allowed_remedys": [{"user_id": "abd754f7", "remedy_date": 20210104, "is_free_punch": false, "punch_no": 0, "work_type": 1, "punch_status": "Lack", "normal_punch_time": "2021-07-01 09:00", "remedy_start_time": "2021-07-01 08:00", "remedy_end_time": "2021-07-01 10:00"}]}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/attendance/v1/user_task_remedys/query_user_allowed_remedys",
            ))
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

        let data = QueryUserAllowedRemedysRequest::new(
            config,
            "employee_id".to_string(),
            "user_001".to_string(),
            20_210_104,
        )
        .execute()
        .await
        .expect("attendance_v1_user_task_remedy_query_user_allowed_remedys 应成功");

        assert_eq!(data.user_allowed_remedys.len(), 1);
        let remedy = &data.user_allowed_remedys[0];
        assert_eq!(remedy.user_id, "abd754f7");
        assert_eq!(remedy.remedy_date, Some(20_210_104));
        assert_eq!(remedy.is_free_punch, Some(false));
        assert_eq!(remedy.punch_status.as_deref(), Some("Lack"));
        assert_eq!(
            remedy.normal_punch_time.as_deref(),
            Some("2021-07-01 09:00")
        );
        assert_eq!(
            remedy.remedy_start_time.as_deref(),
            Some("2021-07-01 08:00")
        );
        assert_eq!(remedy.remedy_end_time.as_deref(), Some("2021-07-01 10:00"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_task_remedys/query_user_allowed_remedys"
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(body.contains("\"user_id\""), "请求体缺 user_id: {body}");
        assert!(
            body.contains("\"remedy_date\""),
            "请求体缺 remedy_date: {body}"
        );
    }
}
