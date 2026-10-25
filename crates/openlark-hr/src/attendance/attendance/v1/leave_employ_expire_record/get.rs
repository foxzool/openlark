//! 通过过期时间获取发放记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/leave_employ_expire_record/get>
//!
//! 官方路径参数 `:leave_id` 为假期类型 ID；过滤条件在 GET 请求体
//! （employment_id/leave_type_id/start_expiration_date/end_expiration_date/time_offset），
//! query 仅 `user_id_type`。

use crate::common::shared_models::I18nLangValue;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 通过过期时间获取发放记录请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 假期类型 ID（必填，路径参数，可通过获取假期类型列表接口获得）
    leave_id: String,
    /// 员工 ID（必填，与 user_id_type 保持一致）
    employment_id: String,
    /// 假期类型 ID（必填，请求体过滤条件）
    leave_type_id: String,
    /// 失效最早日期（必填，格式 yyyy-MM-dd）
    start_expiration_date: String,
    /// 失效最晚日期（必填，格式 yyyy-MM-dd）
    end_expiration_date: String,
    /// 时区偏移（可选，单位分钟，如东八区 480；缺省按东八区）
    time_offset: Option<i32>,
    /// 用户 ID 类型（可选，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl GetRequest {
    /// 创建请求
    ///
    /// - `leave_id`: 假期类型 ID（路径参数）
    /// - `employment_id`: 员工 ID
    /// - `leave_type_id`: 假期类型 ID（请求体过滤条件）
    /// - `start_expiration_date`: 失效最早日期（yyyy-MM-dd）
    /// - `end_expiration_date`: 失效最晚日期（yyyy-MM-dd）
    pub fn new(
        config: Config,
        leave_id: String,
        employment_id: String,
        leave_type_id: String,
        start_expiration_date: String,
        end_expiration_date: String,
    ) -> Self {
        Self {
            leave_id,
            employment_id,
            leave_type_id,
            start_expiration_date,
            end_expiration_date,
            time_offset: None,
            user_id_type: None,
            config,
        }
    }

    /// 设置时区偏移（可选，单位分钟，如东八区 480）
    pub fn time_offset(mut self, time_offset: i32) -> Self {
        self.time_offset = Some(time_offset);
        self
    }

    /// 设置用户 ID 类型（可选，open_id/union_id/user_id/people_corehr_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.leave_id.trim(), "leave_id");
        validate_required!(self.employment_id.trim(), "employment_id");
        validate_required!(self.leave_type_id.trim(), "leave_type_id");
        validate_required!(self.start_expiration_date.trim(), "start_expiration_date");
        validate_required!(self.end_expiration_date.trim(), "end_expiration_date");
        // yyyy-MM-dd 字符串按字典序比较即时间序
        if self.end_expiration_date < self.start_expiration_date {
            return Err(openlark_core::error::validation_error(
                "日期范围无效",
                "end_expiration_date 不能早于 start_expiration_date",
            ));
        }
        Ok(())
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        self.validate()?;

        // 1. 构建端点（路径参数为假期类型 ID）
        let api_endpoint = AttendanceApiV1::LeaveEmployExpireRecordGet(self.leave_id.clone());
        let mut request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(ref user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        // 3. 构建请求体（官方过滤条件在 GET 请求体中）
        let request_body = GetRequestBody {
            employment_id: self.employment_id,
            leave_type_id: self.leave_type_id,
            start_expiration_date: self.start_expiration_date,
            end_expiration_date: self.end_expiration_date,
            time_offset: self.time_offset,
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
            "通过过期时间获取发放记录响应数据为空",
        )
        .await
    }
}

/// 通过过期时间获取发放记录请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRequestBody {
    /// 员工 ID，与 user_id_type 保持一致
    pub employment_id: String,
    /// 假期类型 ID
    pub leave_type_id: String,
    /// 失效最早日期，格式 yyyy-MM-dd
    pub start_expiration_date: String,
    /// 失效最晚日期，格式 yyyy-MM-dd
    pub end_expiration_date: String,
    /// 时区偏移，单位分钟（如东八区 480），缺省按东八区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_offset: Option<i32>,
}

/// 通过过期时间获取发放记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 员工过期日期的发放记录（leave_employ_expire_record[]）
    #[serde(default)]
    pub records: Vec<LeaveEmployExpireRecord>,
}

/// 假期发放过期记录（官方 `leave_employ_expire_record`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeaveEmployExpireRecord {
    /// 发放记录 ID
    pub id: String,
    /// 员工 ID，与 user_id_type 保持一致
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 假期类型 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    /// 发放数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granting_quantity: Option<String>,
    /// 剩余数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left_granting_quantity: Option<String>,
    /// 发放单位：1 天、2 小时
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granting_unit: Option<i32>,
    /// 生效日期，格式 yyyy-MM-dd
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 失效日期，格式 yyyy-MM-dd
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// 发放原因（lang_text[]）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Vec<I18nLangValue>>,
    /// 是否已经被外部系统更改过
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_update_by_external: Option<bool>,
    /// 发放来源：1 系统发放、2 人工发放、3 外部系统发放、4 虚拟发放、5 旧系统导入、6 加班转入
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accrual_source: Option<i32>,
    /// 假期子类型 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_sub_type_id: Option<String>,
    /// 是否参与折算：1 不参与折算、2 参与折算（默认不折算）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section_type: Option<i32>,
}

impl ApiResponseTrait for GetResponse {
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
    fn test_get_request_builder_new() {
        let request = GetRequest::new(
            TestConfigBuilder::new().build(),
            "7111688079785723436".to_string(),
            "6982509313466189342".to_string(),
            "7111688079785723436".to_string(),
            "2023-04-10".to_string(),
            "2023-05-10".to_string(),
        );
        let _ = request;
    }

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_leave_employ_expire_record_get_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.records[]（完整 13 字段）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"records": [{"id": "6893014062142064135", "employment_id": "6982509313466189342", "leave_type_id": "6893014062142064135", "granting_quantity": "1", "left_granting_quantity": "1", "granting_unit": 1, "effective_date": "2020-01-01", "expiration_date": "2020-01-01", "reason": [{"lang": "zh_CN", "value": "test"}], "is_update_by_external": true, "accrual_source": 1, "leave_sub_type_id": "6893014062142064135", "section_type": 1}]}"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/attendance/v1/leave_employ_expire_records/7111688079785723436",
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

        let data = GetRequest::new(
            config,
            "7111688079785723436".to_string(),
            "6982509313466189342".to_string(),
            "7111688079785723436".to_string(),
            "2023-04-10".to_string(),
            "2023-05-10".to_string(),
        )
        .time_offset(480)
        .execute()
        .await
        .expect("attendance_v1_leave_employ_expire_record_get 应成功");

        assert_eq!(data.records.len(), 1);
        let record = &data.records[0];
        assert_eq!(record.id, "6893014062142064135");
        assert_eq!(record.granting_quantity.as_deref(), Some("1"));
        assert_eq!(record.granting_unit, Some(1));
        assert_eq!(record.effective_date.as_deref(), Some("2020-01-01"));
        assert_eq!(record.reason.as_ref().map(|r| r.len()), Some(1));
        assert_eq!(record.is_update_by_external, Some(true));
        assert_eq!(record.accrual_source, Some(1));
        assert_eq!(record.section_type, Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/leave_employ_expire_records/7111688079785723436"
        );
        // 官方过滤条件在 GET 请求体中
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(
            body.contains("\"employment_id\""),
            "请求体缺 employment_id: {body}"
        );
        assert!(
            body.contains("\"start_expiration_date\""),
            "请求体缺 start_expiration_date: {body}"
        );
        assert!(
            body.contains("\"time_offset\""),
            "请求体缺 time_offset: {body}"
        );
        assert!(
            !body.contains("expire_time_start"),
            "请求体不应含编造字段 expire_time_start: {body}"
        );
    }
}
