//! 查询归档报表表头
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/archive_rule/user_stats_fields_query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 查询归档报表表头请求
#[derive(Debug, Clone)]
pub struct UserStatsFieldsQueryRequest {
    /// 归档规则 ID（可选）
    archive_rule_id: Option<String>,
    /// 配置信息
    config: Config,
}

impl UserStatsFieldsQueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            archive_rule_id: None,
            config,
        }
    }

    /// 设置归档规则 ID（可选）
    pub fn archive_rule_id(mut self, archive_rule_id: String) -> Self {
        self.archive_rule_id = Some(archive_rule_id);
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

        // 1. 构建端点
        let api_endpoint = AttendanceApiV1::ArchiveRuleUserStatsFieldsQuery;
        let mut request = ApiRequest::<UserStatsFieldsQueryResponse>::post(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(ref archive_rule_id) = self.archive_rule_id {
            request = request.query("archive_rule_id", archive_rule_id);
        }

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询归档报表表头响应数据为空",
        )
        .await
    }
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
        let request = UserStatsFieldsQueryRequest::new(TestConfigBuilder::new().build())
            .archive_rule_id("test".to_string());
        let _ = request;
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_archive_rule_user_stats_fields_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
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

        let data = UserStatsFieldsQueryRequest::new(config)
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
    }
}
