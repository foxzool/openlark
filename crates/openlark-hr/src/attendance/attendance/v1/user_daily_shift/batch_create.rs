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
    /// 排班记录列表（必填）
    shifts: Vec<UserDailyShift>,
    /// 配置信息
    config: Config,
}

impl BatchCreateRequest {
    /// 创建请求
    pub fn new(config: Config, shifts: Vec<UserDailyShift>) -> Self {
        Self { shifts, config }
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
        validate_required_list!(self.shifts, 100, "shifts 不能为空且不能超过 100 个");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserDailyShiftBatchCreate;
        let request = ApiRequest::<BatchCreateResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = BatchCreateRequestBody {
            shifts: self.shifts,
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
    /// 排班记录列表
    pub shifts: Vec<UserDailyShift>,
}

/// 用户每日排班记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDailyShift {
    /// 用户 ID
    pub user_id: String,
    /// 排班日期（Unix 时间戳）
    pub date: i64,
    /// 班次 ID
    pub shift_id: String,
    /// 工作时长（小时）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_hours: Option<f64>,
}

/// 创建或修改排班表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateResponse {
    /// 是否成功
    pub success: bool,
    /// 成功处理的记录数
    pub processed_count: i32,
    /// 失败的记录数
    pub failed_count: i32,
    /// 失败的排班记录索引列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_indices: Option<Vec<i32>>,
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
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_user_daily_shift_batch_create_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"success": false, "processed_count": 0, "failed_count": 0}"#)
                .unwrap();
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/attendance/v1/user_daily_shifts/batch_create",
            ))
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

        let data = BatchCreateRequest::new(
            config,
            vec![UserDailyShift {
                user_id: "user_id_1".to_string(),
                date: 1,
                shift_id: "shift_id_1".to_string(),
                work_hours: None,
            }],
        )
        .execute()
        .await
        .expect("attendance_v1_user_daily_shift_batch_create 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_daily_shifts/batch_create"
        );
    }
}
