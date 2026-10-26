//! 查询统计字段定义
//!
//! docPath: <https://open.feishu.cn/document/attendance-v1/user_stats_field/query>
//!
//! 必填 query 参数 `employee_type`；请求体为 locale/stats_type/start_date/end_date。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::{QueryRequestBody, QueryResponse};

/// 查询统计字段定义请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 员工 ID 类型（必填，query 参数）：employee_id / employee_no
    employee_type: String,
    /// 语言类型（必填）：en / ja / zh
    locale: String,
    /// 统计类型（必填）：daily-日度统计，month-月度统计
    stats_type: String,
    /// 开始时间，格式 yyyyMMdd（官方标注「无效字段」，仍必填）
    start_date: i32,
    /// 结束时间，格式 yyyyMMdd（官方标注「无效字段」，仍必填）
    end_date: i32,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建查询统计字段定义请求
    ///
    /// - `employee_type`: 员工 ID 类型（query 参数，必填）
    /// - `locale`: 语言类型（必填）
    /// - `stats_type`: 统计类型（必填）
    /// - `start_date` / `end_date`: 起止时间，格式 yyyyMMdd
    pub fn new(
        config: Config,
        employee_type: String,
        locale: String,
        stats_type: String,
        start_date: i32,
        end_date: i32,
    ) -> Self {
        Self {
            employee_type,
            locale,
            stats_type,
            start_date,
            end_date,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.employee_type, "employee_type 不能为空");
        validate_required!(self.locale, "locale 不能为空");
        validate_required!(self.stats_type, "stats_type 不能为空");

        // 2. 构建端点并附加必填 query 参数 employee_type
        let api_endpoint = AttendanceApiV1::UserStatsFieldQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url())
            .query("employee_type", &self.employee_type);

        // 3. 序列化请求体
        let request_body = QueryRequestBody {
            locale: self.locale,
            stats_type: self.stats_type,
            start_date: self.start_date,
            end_date: self.end_date,
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
            "查询统计字段定义响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for QueryResponse {
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
    fn test_query_request_builder_new() {
        let request = QueryRequest::new(
            TestConfigBuilder::new().build(),
            "employee_id".to_string(),
            "zh".to_string(),
            "daily".to_string(),
            20_210_316,
            20_210_323,
        );
        let _ = request;
    }

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_user_stats_field_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.user_stats_field.fields[].child_fields[]
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/user_stats_fields/query"))
            .and(query_param("employee_type", "employee_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "user_stats_field": {
                    "stats_type": "month",
                    "user_id": "",
                    "fields": [
                        {"code": "501", "title": "Basic info", "child_fields": [
                            {"code": "50103", "title": "Employee ID"}
                        ]},
                        {"code": "521", "title": "Attendance statistics", "child_fields": [
                            {"code": "52104", "title": "Required attendance duration", "time_unit": "Minute"}
                        ]}
                    ]
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

        let data = QueryRequest::new(
            config,
            "employee_id".to_string(),
            "zh".to_string(),
            "daily".to_string(),
            20_210_316,
            20_210_323,
        )
        .execute()
        .await
        .expect("attendance_v1_user_stats_field_query 应成功");

        let stats_field = data.user_stats_field.expect("user_stats_field 应存在");
        assert_eq!(stats_field.stats_type.as_deref(), Some("month"));
        assert_eq!(stats_field.fields.len(), 2);
        assert_eq!(stats_field.fields[1].child_fields[0].code, "52104");
        assert_eq!(
            stats_field.fields[1].child_fields[0].time_unit.as_deref(),
            Some("Minute")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_stats_fields/query"
        );
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("employee_type=employee_id"),
            "缺必填 query 参数 employee_type: {query}"
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(body.contains("\"locale\""), "请求体缺 locale: {body}");
        assert!(
            body.contains("\"stats_type\""),
            "请求体缺 stats_type: {body}"
        );
        assert!(
            !body.contains("\"unit_id\""),
            "请求体不应含旧字段 unit_id: {body}"
        );
    }
}
