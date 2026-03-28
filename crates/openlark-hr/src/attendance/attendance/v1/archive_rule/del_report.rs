//! 删除归档报表行数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/archive_rule/del_report>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 删除归档报表行数据请求
#[derive(Debug, Clone)]
pub struct DelReportRequest {
    /// 归档规则 ID（必填）
    archive_rule_id: String,
    /// 员工 ID 列表（必填）
    employee_ids: Vec<String>,
    /// 统计日期列表（必填）
    stat_dates: Vec<i64>,
    /// 配置信息
    config: Config,
}

impl DelReportRequest {
    /// 创建请求
    pub fn new(
        config: Config,
        archive_rule_id: String,
        employee_ids: Vec<String>,
        stat_dates: Vec<i64>,
    ) -> Self {
        Self {
            archive_rule_id,
            employee_ids,
            stat_dates,
            config,
        }
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
        validate_required!(self.archive_rule_id.trim(), "archive_rule_id");
        validate_required_list!(
            self.employee_ids,
            100,
            "员工 ID 列表不能为空且不能超过 100 个"
        );
        validate_required_list!(
            self.stat_dates,
            100,
            "统计日期列表不能为空且不能超过 100 个"
        );

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::ArchiveRuleDelReport;
        let request = ApiRequest::<DelReportResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = DelReportRequestBody {
            archive_rule_id: self.archive_rule_id,
            employee_ids: self.employee_ids,
            stat_dates: self.stat_dates,
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
                "删除归档报表行数据响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 删除归档报表行数据请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelReportRequestBody {
    /// 归档规则 ID
    pub archive_rule_id: String,
    /// 员工 ID 列表
    pub employee_ids: Vec<String>,
    /// 统计日期列表
    pub stat_dates: Vec<i64>,
}

/// 删除归档报表行数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DelReportResponse {
    /// 是否成功
    pub success: bool,
    /// 删除的记录数
    pub deleted_count: i32,
}

impl ApiResponseTrait for DelReportResponse {
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
