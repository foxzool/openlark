//! 删除归档报表行数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/archive_rule/del_report>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 删除归档报表行数据请求
#[derive(Debug, Clone)]
pub struct DelReportRequest {
    /// 月份，格式 `yyyyMM`（必填）
    month: String,
    /// 操作者 ID（必填）
    operator_id: String,
    /// 归档规则 ID（必填）
    archive_rule_id: String,
    /// 用户 ID 列表（可选）
    user_ids: Option<Vec<String>>,
    /// 配置信息
    config: Config,
}

impl DelReportRequest {
    /// 创建请求
    ///
    /// - `month`: 月份，格式 `yyyyMM`
    /// - `operator_id`: 操作者 ID
    /// - `archive_rule_id`: 归档规则 ID
    pub fn new(
        config: Config,
        month: String,
        operator_id: String,
        archive_rule_id: String,
    ) -> Self {
        Self {
            month,
            operator_id,
            archive_rule_id,
            user_ids: None,
            config,
        }
    }

    /// 设置用户 ID 列表（可选）
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DelReportResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DelReportResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.month.trim(), "month");
        validate_required!(self.operator_id.trim(), "operator_id");
        validate_required!(self.archive_rule_id.trim(), "archive_rule_id");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::ArchiveRuleDelReport;
        let request = ApiRequest::<DelReportResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = DelReportRequestBody {
            month: self.month,
            operator_id: self.operator_id,
            archive_rule_id: self.archive_rule_id,
            user_ids: self.user_ids,
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
            "删除归档报表行数据响应数据为空",
        )
        .await
    }
}

/// 删除归档报表行数据请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelReportRequestBody {
    /// 月份，格式 `yyyyMM`
    pub month: String,
    /// 操作者 ID
    pub operator_id: String,
    /// 归档规则 ID
    pub archive_rule_id: String,
    /// 用户 ID 列表（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

/// 删除归档报表行数据响应
///
/// 飞书官网 response `data` 为空对象（无返回字段）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DelReportResponse {}

impl ApiResponseTrait for DelReportResponse {
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
    fn test_del_report_request_builder_new() {
        let request = DelReportRequest::new(
            TestConfigBuilder::new().build(),
            "202409".to_string(),
            "a111xd".to_string(),
            "1".to_string(),
        )
        .user_ids(vec!["xx1uad".to_string()]);
        let _ = request;
    }

    /// 端到端：Builder→execute→Transport→mock→assert 请求体字段对齐飞书官网 schema。
    #[tokio::test]
    async fn test_attendance_v1_archive_rule_del_report_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 飞书官网 response data 为空对象
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/archive_rule/del_report"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let _data = DelReportRequest::new(
            config,
            "202409".to_string(),
            "a111xd".to_string(),
            "1".to_string(),
        )
        .user_ids(vec!["xx1uad".to_string()])
        .execute()
        .await
        .expect("attendance_v1_archive_rule_del_report 应成功");

        // 请求体对齐官网必填字段，且不含旧错误字段
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/archive_rule/del_report"
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(body.contains("\"month\""), "请求体缺 month: {body}");
        assert!(
            body.contains("\"operator_id\""),
            "请求体缺 operator_id: {body}"
        );
        assert!(
            body.contains("\"archive_rule_id\""),
            "请求体缺 archive_rule_id: {body}"
        );
        assert!(
            !body.contains("employee_ids"),
            "请求体不应含旧字段 employee_ids: {body}"
        );
        assert!(
            !body.contains("stat_dates"),
            "请求体不应含旧字段 stat_dates: {body}"
        );
    }
}
