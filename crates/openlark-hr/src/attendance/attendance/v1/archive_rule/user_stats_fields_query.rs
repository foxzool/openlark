//! 查询归档报表表头
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/archive_rule/user_stats_fields_query>
//!
//! 必填 query 参数 `employee_type`；请求体为 month/archive_rule_id/operator_id（locale 可选）。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询归档报表表头请求
#[derive(Debug, Clone)]
pub struct UserStatsFieldsQueryRequest {
    /// 员工 ID 类型（必填，query 参数）：employee_id / employee_no
    employee_type: String,
    /// 月份，格式 `yyyyMM`（必填）
    month: String,
    /// 归档规则 ID（必填），可通过「查询所有归档规则」获得
    archive_rule_id: String,
    /// 操作者 ID，对应 employee_type（必填）
    operator_id: String,
    /// 语言类型（可选，默认 zh）：en / ja / zh
    locale: Option<String>,
    /// 配置信息
    config: Config,
}

impl UserStatsFieldsQueryRequest {
    /// 创建请求
    ///
    /// - `employee_type`: 员工 ID 类型（query 参数，必填）
    /// - `month`: 月份，格式 `yyyyMM`
    /// - `archive_rule_id`: 归档规则 ID
    /// - `operator_id`: 操作者 ID，对应 employee_type
    pub fn new(
        config: Config,
        employee_type: String,
        month: String,
        archive_rule_id: String,
        operator_id: String,
    ) -> Self {
        Self {
            employee_type,
            month,
            archive_rule_id,
            operator_id,
            locale: None,
            config,
        }
    }

    /// 设置语言类型（可选，默认 zh）：en / ja / zh
    pub fn locale(mut self, locale: String) -> Self {
        self.locale = Some(locale);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UserStatsFieldsQueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserStatsFieldsQueryResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.employee_type, "employee_type 不能为空");
        validate_required!(self.month, "month 不能为空");
        validate_required!(self.archive_rule_id, "archive_rule_id 不能为空");
        validate_required!(self.operator_id, "operator_id 不能为空");

        // 2. 构建端点并附加必填 query 参数 employee_type
        let api_endpoint = AttendanceApiV1::ArchiveRuleUserStatsFieldsQuery;
        let request = ApiRequest::<UserStatsFieldsQueryResponse>::post(api_endpoint.to_url())
            .query("employee_type", &self.employee_type);

        // 3. 构建请求体
        let request_body = UserStatsFieldsQueryRequestBody {
            month: self.month,
            archive_rule_id: self.archive_rule_id,
            operator_id: self.operator_id,
            locale: self.locale,
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
            "查询归档报表表头响应数据为空",
        )
        .await
    }
}

/// 查询归档报表表头请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatsFieldsQueryRequestBody {
    /// 月份，格式 `yyyyMM`
    pub month: String,
    /// 归档规则 ID
    pub archive_rule_id: String,
    /// 操作者 ID，对应 employee_type
    pub operator_id: String,
    /// 语言类型（默认 zh）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

/// 查询归档报表表头响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserStatsFieldsQueryResponse {
    /// 统计数据表头（archive_field[]）
    #[serde(default)]
    pub archive_report_fields: Vec<ArchiveField>,
}

/// 归档报表表头字段（官方 `archive_field`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchiveField {
    /// 字段编号
    pub code: String,
    /// 字段名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 表头名称层级路径
    #[serde(default)]
    pub upper_titles: Vec<String>,
}

impl ApiResponseTrait for UserStatsFieldsQueryResponse {
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
    fn test_user_stats_fields_query_request_builder_new() {
        let request = UserStatsFieldsQueryRequest::new(
            TestConfigBuilder::new().build(),
            "employee_id".to_string(),
            "202409".to_string(),
            "1".to_string(),
            "ax8ud".to_string(),
        )
        .locale("zh".to_string());
        let _ = request;
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_archive_rule_user_stats_fields_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.archive_report_fields[]（code/title/upper_titles）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"archive_report_fields": [{"code": "1", "title": "工号", "upper_titles": ["基本信息"]}]}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/attendance/v1/archive_rule/user_stats_fields_query",
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

        let data = UserStatsFieldsQueryRequest::new(
            config,
            "employee_id".to_string(),
            "202409".to_string(),
            "1".to_string(),
            "ax8ud".to_string(),
        )
        .execute()
        .await
        .expect("attendance_v1_archive_rule_user_stats_fields_query 应成功");

        assert_eq!(data.archive_report_fields.len(), 1);
        assert_eq!(data.archive_report_fields[0].code, "1");
        assert_eq!(data.archive_report_fields[0].title.as_deref(), Some("工号"));
        assert_eq!(data.archive_report_fields[0].upper_titles, vec!["基本信息"]);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/archive_rule/user_stats_fields_query"
        );
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("employee_type=employee_id"),
            "缺必填 query 参数 employee_type: {query}"
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(body.contains("\"month\""), "请求体缺 month: {body}");
        assert!(
            body.contains("\"archive_rule_id\""),
            "请求体缺 archive_rule_id: {body}"
        );
        assert!(
            body.contains("\"operator_id\""),
            "请求体缺 operator_id: {body}"
        );
    }
}
