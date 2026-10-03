//! 修改发放记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/leave_accrual_record/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 修改发放记录请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 发放记录 ID（path 参数 `leave_id`，必填）
    leave_id: String,
    /// 发放记录的唯一 ID（必填）
    leave_granting_record_id: String,
    /// 员工 ID（必填）
    employment_id: String,
    /// 假期类型 ID（必填）
    leave_type_id: String,
    /// 修改原因（必填，多语言文本）
    reason: Vec<LangText>,
    /// 时间偏移（可选，东八区 = `480`）
    time_offset: Option<i32>,
    /// 失效日期（可选，格式 `2020-01-01`）
    expiration_date: Option<String>,
    /// 修改发放数量（可选）
    quantity: Option<String>,
    /// 配置信息
    config: Config,
}

impl PatchRequest {
    /// 创建请求
    ///
    /// - `leave_id`: path 参数（发放记录 ID）
    /// - `leave_granting_record_id`: 发放记录唯一 ID
    /// - `employment_id`: 员工 ID
    /// - `leave_type_id`: 假期类型 ID
    /// - `reason`: 修改原因（多语言）
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        config: Config,
        leave_id: String,
        leave_granting_record_id: String,
        employment_id: String,
        leave_type_id: String,
        reason: Vec<LangText>,
    ) -> Self {
        Self {
            leave_id,
            leave_granting_record_id,
            employment_id,
            leave_type_id,
            reason,
            time_offset: None,
            expiration_date: None,
            quantity: None,
            config,
        }
    }

    /// 设置时间偏移（可选）
    pub fn time_offset(mut self, time_offset: i32) -> Self {
        self.time_offset = Some(time_offset);
        self
    }

    /// 设置失效日期（可选）
    pub fn expiration_date(mut self, expiration_date: String) -> Self {
        self.expiration_date = Some(expiration_date);
        self
    }

    /// 设置修改发放数量（可选）
    pub fn quantity(mut self, quantity: String) -> Self {
        self.quantity = Some(quantity);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.leave_id.trim(), "leave_id");
        validate_required!(
            self.leave_granting_record_id.trim(),
            "leave_granting_record_id"
        );
        validate_required!(self.employment_id.trim(), "employment_id");
        validate_required!(self.leave_type_id.trim(), "leave_type_id");
        validate_required_list!(self.reason, 10, "reason 不能为空");

        // 2. 构建端点（leave_id 为 path 参数）
        let api_endpoint = AttendanceApiV1::LeaveAccrualRecordPatch(self.leave_id.clone()).to_url();
        let request = ApiRequest::<PatchResponse>::patch(&api_endpoint);

        // 3. 构建请求体
        let request_body = PatchRequestBody {
            leave_granting_record_id: self.leave_granting_record_id,
            employment_id: self.employment_id,
            leave_type_id: self.leave_type_id,
            reason: self.reason,
            time_offset: self.time_offset,
            expiration_date: self.expiration_date,
            quantity: self.quantity,
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
            "修改发放记录响应数据为空",
        )
        .await
    }
}

/// 修改发放记录请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequestBody {
    /// 发放记录唯一 ID
    pub leave_granting_record_id: String,
    /// 员工 ID
    pub employment_id: String,
    /// 假期类型 ID
    pub leave_type_id: String,
    /// 修改原因（多语言）
    pub reason: Vec<LangText>,
    /// 时间偏移
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_offset: Option<i32>,
    /// 失效日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// 修改发放数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
}

/// 多语言文本
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LangText {
    /// 语言码（如 `zh_CN`）
    pub lang: String,
    /// 语言码对应的文本
    pub value: String,
}

/// 修改发放记录响应
///
/// 官网 response `data.record` 为 object，schema 未完整给出，透传 `Value`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 发放记录
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchResponse {
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
    fn test_patch_request_builder_new() {
        let request = PatchRequest::new(
            TestConfigBuilder::new().build(),
            "6893014062142064135".to_string(),
            "6893014062142064135".to_string(),
            "6982509313466189342".to_string(),
            "7111688079785723436".to_string(),
            vec![LangText {
                lang: "zh_CN".to_string(),
                value: "test".to_string(),
            }],
        );
        let _ = request;
    }

    /// 端到端：Builder→execute→Transport→mock→assert 请求体字段对齐飞书官网 schema。
    #[tokio::test]
    async fn test_attendance_v1_leave_accrual_record_patch_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/attendance/v1/leave_accrual_record/6893014062142064135",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "record": { "leave_granting_record_id": "6893014062142064135" } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = PatchRequest::new(
            config,
            "6893014062142064135".to_string(),
            "6893014062142064135".to_string(),
            "6982509313466189342".to_string(),
            "7111688079785723436".to_string(),
            vec![LangText {
                lang: "zh_CN".to_string(),
                value: "test".to_string(),
            }],
        )
        .execute()
        .await
        .expect("attendance_v1_leave_accrual_record_patch 应成功");

        assert!(data.record.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/leave_accrual_record/6893014062142064135"
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(
            body.contains("\"leave_granting_record_id\""),
            "请求体缺 leave_granting_record_id: {body}"
        );
        assert!(
            body.contains("\"employment_id\""),
            "请求体缺 employment_id: {body}"
        );
        assert!(
            body.contains("\"leave_type_id\""),
            "请求体缺 leave_type_id: {body}"
        );
        assert!(
            !body.contains("\"remaining_days\""),
            "请求体不应含旧字段 remaining_days: {body}"
        );
    }
}
