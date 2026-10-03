//! 写入归档报表结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/archive_rule/upload_report>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 写入归档报表结果请求
#[derive(Debug, Clone)]
pub struct UploadReportRequest {
    /// 月份，格式 `yyyyMM`（必填）
    month: String,
    /// 操作者 ID（必填）
    operator_id: String,
    /// 归档规则 ID（必填）
    archive_rule_id: String,
    /// 归档报表内容，不超过 50 个（官网 schema 未标 required，但语义上是上传核心数据）
    archive_report_datas: Option<Vec<ArchiveReportData>>,
    /// 配置信息
    config: Config,
}

impl UploadReportRequest {
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
            archive_report_datas: None,
            config,
        }
    }

    /// 设置归档报表内容（不超过 50 个）
    pub fn archive_report_datas(mut self, datas: Vec<ArchiveReportData>) -> Self {
        self.archive_report_datas = Some(datas);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UploadReportResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UploadReportResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.month.trim(), "month");
        validate_required!(self.operator_id.trim(), "operator_id");
        validate_required!(self.archive_rule_id.trim(), "archive_rule_id");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::ArchiveRuleUploadReport;
        let request = ApiRequest::<UploadReportResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = UploadReportRequestBody {
            month: self.month,
            operator_id: self.operator_id,
            archive_rule_id: self.archive_rule_id,
            archive_report_datas: self.archive_report_datas,
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
            "写入归档报表结果响应数据为空",
        )
        .await
    }
}

/// 写入归档报表结果请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadReportRequestBody {
    /// 月份，格式 `yyyyMM`
    pub month: String,
    /// 操作者 ID
    pub operator_id: String,
    /// 归档规则 ID
    pub archive_rule_id: String,
    /// 归档报表内容，不超过 50 个
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_report_datas: Option<Vec<ArchiveReportData>>,
}

/// 归档报表数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchiveReportData {
    /// 用户 ID，对应 employee_type
    pub member_id: String,
    /// 考勤开始时间，格式 `yyyyMMdd`
    pub start_time: String,
    /// 考勤结束时间，格式 `yyyyMMdd`
    pub end_time: String,
    /// 字段结果，不超过 200 个
    pub field_datas: Vec<ArchiveFieldData>,
}

/// 字段结果数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchiveFieldData {
    /// 字段编码
    pub code: String,
    /// 字段结果值
    pub value: String,
}

/// 写入归档报表结果响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UploadReportResponse {
    /// 无效的字段编码列表
    #[serde(default)]
    pub invalid_code: Vec<String>,
    /// 无效的用户 ID 列表，对应 employee_type
    #[serde(default)]
    pub invalid_member_id: Vec<String>,
}

impl ApiResponseTrait for UploadReportResponse {
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
    fn test_upload_report_request_builder_new() {
        let request = UploadReportRequest::new(
            TestConfigBuilder::new().build(),
            "202409".to_string(),
            "ax11d".to_string(),
            "1".to_string(),
        )
        .archive_report_datas(vec![ArchiveReportData {
            member_id: "1aaxxd".to_string(),
            start_time: "20210109".to_string(),
            end_time: "20210109".to_string(),
            field_datas: vec![ArchiveFieldData {
                code: "abd754f7".to_string(),
                value: "1".to_string(),
            }],
        }]);
        let _ = request;
    }

    /// 端到端：Builder→execute→Transport→mock→assert 请求体字段对齐飞书官网 schema。
    #[tokio::test]
    async fn test_attendance_v1_archive_rule_upload_report_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/archive_rule/upload_report"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "invalid_code": ["1"],
                    "invalid_member_id": ["a1xud"]
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = UploadReportRequest::new(
            config,
            "202409".to_string(),
            "ax11d".to_string(),
            "1".to_string(),
        )
        .archive_report_datas(vec![ArchiveReportData {
            member_id: "1aaxxd".to_string(),
            start_time: "20210109".to_string(),
            end_time: "20210109".to_string(),
            field_datas: vec![ArchiveFieldData {
                code: "abd754f7".to_string(),
                value: "1".to_string(),
            }],
        }])
        .execute()
        .await
        .expect("attendance_v1_archive_rule_upload_report 应成功");

        // 响应解析对齐官网 schema
        assert_eq!(data.invalid_code, vec!["1".to_string()]);
        assert_eq!(data.invalid_member_id, vec!["a1xud".to_string()]);

        // 请求体对齐官网字段，且不含旧错误字段
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/archive_rule/upload_report"
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
            body.contains("\"archive_report_datas\""),
            "请求体缺 archive_report_datas: {body}"
        );
        assert!(
            body.contains("\"member_id\""),
            "请求体缺 archive_report_data.member_id: {body}"
        );
        assert!(
            !body.contains("\"reports\""),
            "请求体不应含旧字段 reports: {body}"
        );
    }
}
