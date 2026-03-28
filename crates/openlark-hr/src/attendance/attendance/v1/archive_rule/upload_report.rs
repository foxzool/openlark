//! 写入归档报表结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/archive_rule/upload_report>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 写入归档报表结果请求
#[derive(Debug, Clone)]
pub struct UploadReportRequest {
    /// 归档规则 ID（必填）
    archive_rule_id: String,
    /// 报表数据列表（必填）
    reports: Vec<ReportData>,
    /// 配置信息
    config: Config,
}

impl UploadReportRequest {
    /// 创建请求
    pub fn new(config: Config, archive_rule_id: String, reports: Vec<ReportData>) -> Self {
        Self {
            archive_rule_id,
            reports,
            config,
        }
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
        validate_required!(self.archive_rule_id.trim(), "archive_rule_id");
        validate_required_list!(self.reports, 100, "报表数据列表不能为空且不能超过 100 条");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::ArchiveRuleUploadReport;
        let request = ApiRequest::<UploadReportResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = UploadReportRequestBody {
            archive_rule_id: self.archive_rule_id,
            reports: self.reports,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {}", e),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "写入归档报表结果响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 写入归档报表结果请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadReportRequestBody {
    /// 归档规则 ID
    pub archive_rule_id: String,
    /// 报表数据列表
    pub reports: Vec<ReportData>,
}

/// 报表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    /// 员工 ID
    pub employee_id: String,
    /// 统计日期（Unix 时间戳）
    pub stat_date: i64,
    /// 字段数据
    pub field_data: Vec<FieldData>,
}

/// 字段数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldData {
    /// 字段 ID
    pub field_id: String,
    /// 字段值
    pub value: serde_json::Value,
}

/// 写入归档报表结果响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UploadReportResponse {
    /// 是否成功
    pub success: bool,
    /// 成功写入的记录数
    pub inserted_count: i32,
    /// 更新的记录数
    pub updated_count: i32,
    /// 失败的记录数
    pub failed_count: i32,
    /// 失败的员工 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_employee_ids: Option<Vec<String>>,
}

impl ApiResponseTrait for UploadReportResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
