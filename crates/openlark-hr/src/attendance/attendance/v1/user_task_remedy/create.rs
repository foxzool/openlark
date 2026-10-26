//! 通知补卡审批发起
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_task_remedy/create>

use super::UserTaskRemedy;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 通知补卡审批发起请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 员工 ID 类型（必填，query 参数）：employee_id / employee_no
    employee_type: String,
    /// 用户 ID（必填）
    user_id: String,
    /// 补卡日期，格式 `yyyyMMdd`（必填）
    remedy_date: i32,
    /// 第几次上下班（必填）：`0`=第 1 次，`1`=第 2 次，`2`=第 3 次；自由班制填 `0`
    punch_no: i32,
    /// 上班/下班（必填）：`1`=上班，`2`=下班；自由班制填 `0`
    work_type: i32,
    /// 补卡时间，格式 `yyyy-MM-dd HH:mm`（必填）
    remedy_time: String,
    /// 补卡原因（必填）
    reason: String,
    /// 配置信息
    config: Config,
}

impl CreateRequest {
    /// 创建请求
    ///
    /// - `employee_type`: 员工 ID 类型（query 参数，必填），user_id 需与其取值一致
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        config: Config,
        employee_type: String,
        user_id: String,
        remedy_date: i32,
        punch_no: i32,
        work_type: i32,
        remedy_time: String,
        reason: String,
    ) -> Self {
        Self {
            employee_type,
            user_id,
            remedy_date,
            punch_no,
            work_type,
            remedy_time,
            reason,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.employee_type, "employee_type 不能为空");
        validate_required!(self.user_id.trim(), "user_id");
        validate_required!(self.remedy_time.trim(), "remedy_time");
        validate_required!(self.reason.trim(), "reason");

        // 2. 构建端点并附加必填 query 参数 employee_type
        let api_endpoint = AttendanceApiV1::UserTaskRemedyCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url())
            .query("employee_type", &self.employee_type);

        // 3. 构建请求体
        let request_body = CreateRequestBody {
            user_id: self.user_id,
            remedy_date: self.remedy_date,
            punch_no: self.punch_no,
            work_type: self.work_type,
            remedy_time: self.remedy_time,
            reason: self.reason,
            time: None,
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
            "通知补卡审批发起响应数据为空",
        )
        .await
    }
}

/// 通知补卡审批发起请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 用户 ID
    pub user_id: String,
    /// 补卡日期 `yyyyMMdd`
    pub remedy_date: i32,
    /// 第几次上下班
    pub punch_no: i32,
    /// 上班/下班
    pub work_type: i32,
    /// 补卡时间 `yyyy-MM-dd HH:mm`
    pub remedy_time: String,
    /// 补卡原因
    pub reason: String,
    /// 补卡时间（官方标注「字段已失效」，示例值 "-"），默认不发送
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

/// 通知补卡审批发起响应
///
/// 官网 response `data.user_remedy` 为 `user_task_remedy` 对象，字段已全部类型化；
/// 其中 `status`/`time`/`time_zone`/`create_time`/`update_time` 官方标注为无效字段
/// （示例恒为 "-" 或固定审批中）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 补卡审批结果
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_remedy: Option<UserTaskRemedy>,
}

impl ApiResponseTrait for CreateResponse {
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
    fn test_create_request_builder_new() {
        let request = CreateRequest::new(
            TestConfigBuilder::new().build(),
            "employee_id".to_string(),
            "abd754f7".to_string(),
            20_210_701,
            0,
            1,
            "2021-07-01 08:00".to_string(),
            "忘记打卡".to_string(),
        );
        let _ = request;
    }

    /// 端到端：Builder→execute→Transport→mock→assert 请求体字段对齐飞书官网 schema。
    #[tokio::test]
    async fn test_attendance_v1_user_task_remedy_create_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/user_task_remedys"))
            .and(query_param("employee_type", "employee_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "user_remedy": {
                    "user_id": "abd754f7",
                    "remedy_date": 20210701,
                    "punch_no": 0,
                    "work_type": 1,
                    "approval_id": "6737202939523236113",
                    "remedy_time": "2021-07-01 08:00",
                    "status": 2,
                    "reason": "忘记打卡",
                    "time": "-",
                    "time_zone": "-",
                    "create_time": "-",
                    "update_time": "-"
                } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = CreateRequest::new(
            config,
            "employee_id".to_string(),
            "abd754f7".to_string(),
            20_210_701,
            0,
            1,
            "2021-07-01 08:00".to_string(),
            "忘记打卡".to_string(),
        )
        .execute()
        .await
        .expect("attendance_v1_user_task_remedy_create 应成功");

        let remedy = data.user_remedy.expect("user_remedy 应存在");
        assert_eq!(remedy.user_id, "abd754f7");
        assert_eq!(remedy.remedy_date, 20_210_701);
        assert_eq!(remedy.approval_id, "6737202939523236113");
        assert_eq!(remedy.remedy_time, "2021-07-01 08:00");
        assert_eq!(remedy.reason, "忘记打卡");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_task_remedys"
        );
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("employee_type=employee_id"),
            "缺必填 query 参数 employee_type: {query}"
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(
            body.contains("\"remedy_date\""),
            "请求体缺 remedy_date: {body}"
        );
        assert!(body.contains("\"punch_no\""), "请求体缺 punch_no: {body}");
        assert!(body.contains("\"work_type\""), "请求体缺 work_type: {body}");
        assert!(
            !body.contains("\"original_time\""),
            "请求体不应含旧字段 original_time: {body}"
        );
    }
}
