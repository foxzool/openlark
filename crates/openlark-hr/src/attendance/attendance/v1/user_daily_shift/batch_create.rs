//! 创建或修改排班表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_daily_shift/batch_create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 创建或修改排班表请求
#[derive(Debug, Clone)]
pub struct BatchCreateRequest {
    /// 排班表信息列表（必填，不超过 50 个）
    user_daily_shifts: Vec<UserDailyShift>,
    /// 操作人 UID（可选；未走「API 接入」流程时为必填）
    operator_id: Option<String>,
    /// 配置信息
    config: Config,
}

impl BatchCreateRequest {
    /// 创建请求
    pub fn new(config: Config, user_daily_shifts: Vec<UserDailyShift>) -> Self {
        Self {
            user_daily_shifts,
            operator_id: None,
            config,
        }
    }

    /// 设置操作人 UID（可选）
    pub fn operator_id(mut self, operator_id: String) -> Self {
        self.operator_id = Some(operator_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchCreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchCreateResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required_list!(
            self.user_daily_shifts,
            50,
            "user_daily_shifts 不能为空且不能超过 50 个"
        );

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserDailyShiftBatchCreate;
        let request = ApiRequest::<BatchCreateResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = BatchCreateRequestBody {
            user_daily_shifts: self.user_daily_shifts,
            operator_id: self.operator_id,
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
            "创建或修改排班表响应数据为空",
        )
        .await
    }
}

/// 创建或修改排班表请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateRequestBody {
    /// 排班表信息列表
    pub user_daily_shifts: Vec<UserDailyShift>,
    /// 操作人 UID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
}

/// 用户每日排班记录
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserDailyShift {
    /// 考勤组 ID
    pub group_id: String,
    /// 班次 ID（传 `0` 代表休息）
    pub shift_id: String,
    /// 月份，格式 `yyyyMM`
    pub month: i32,
    /// 用户 ID
    pub user_id: String,
    /// 日期
    pub day_no: i32,
    /// 是否清空班次（优先于 `shift_id`，为 `true` 时 `shift_id` 失效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_clear_schedule: Option<bool>,
}

/// 创建或修改排班表响应
///
/// 官网 response `data.user_daily_shifts` 为数组，items 结构 schema 未完整给出，透传 `Value`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateResponse {
    /// 排班表信息列表
    #[serde(default)]
    pub user_daily_shifts: Vec<serde_json::Value>,
}

impl ApiResponseTrait for BatchCreateResponse {
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
    fn test_batch_create_request_builder_new() {
        let request = BatchCreateRequest::new(
            TestConfigBuilder::new().build(),
            vec![UserDailyShift {
                group_id: "6737202939523236110".to_string(),
                shift_id: "6753520403404030215".to_string(),
                month: 202_101,
                user_id: "abd754f7".to_string(),
                day_no: 21,
                is_clear_schedule: None,
            }],
        );
        let _ = request;
    }

    /// 端到端：Builder→execute→Transport→mock→assert 请求体字段对齐飞书官网 schema。
    #[tokio::test]
    async fn test_attendance_v1_user_daily_shift_batch_create_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/attendance/v1/user_daily_shifts/batch_create",
            ))
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

        let data = BatchCreateRequest::new(
            config,
            vec![UserDailyShift {
                group_id: "6737202939523236110".to_string(),
                shift_id: "6753520403404030215".to_string(),
                month: 202_101,
                user_id: "abd754f7".to_string(),
                day_no: 21,
                is_clear_schedule: None,
            }],
        )
        .execute()
        .await
        .expect("attendance_v1_user_daily_shift_batch_create 应成功");

        assert!(data.user_daily_shifts.is_empty());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_daily_shifts/batch_create"
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(
            body.contains("\"user_daily_shifts\""),
            "请求体缺 user_daily_shifts: {body}"
        );
        assert!(body.contains("\"month\""), "请求体缺 month: {body}");
        assert!(body.contains("\"day_no\""), "请求体缺 day_no: {body}");
        assert!(
            !body.contains("\"shifts\""),
            "请求体不应含旧字段 shifts: {body}"
        );
    }
}
