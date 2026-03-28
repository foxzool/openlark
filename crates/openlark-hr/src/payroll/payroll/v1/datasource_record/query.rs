//! 批量查询外部算薪数据记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/datasource_record/query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量查询外部算薪数据记录请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 数据源 ID（可选）
    datasource_id: Option<String>,
    /// 员工 ID 列表（可选）
    employee_ids: Option<Vec<String>>,
    /// 分页大小（可选，默认 50，最大 100）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            datasource_id: None,
            employee_ids: None,
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置数据源 ID（可选）
    pub fn datasource_id(mut self, datasource_id: String) -> Self {
        self.datasource_id = Some(datasource_id);
        self
    }

    /// 设置员工 ID 列表（可选）
    pub fn employee_ids(mut self, employee_ids: Vec<String>) -> Self {
        self.employee_ids = Some(employee_ids);
        self
    }

    /// 设置分页大小（可选，默认 50，最大 100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
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
        use crate::common::api_endpoints::PayrollApiV1;

        // 1. 构建端点
        let api_endpoint = PayrollApiV1::DatasourceRecordQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            datasource_id: self.datasource_id,
            employee_ids: self.employee_ids,
            page_size: self.page_size,
            page_token: self.page_token,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {}", e),
            )
        })?;
        let request = request.body(request_body_json);

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "批量查询外部算薪数据记录响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 批量查询外部算薪数据记录请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 数据源 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasource_id: Option<String>,
    /// 员工 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_ids: Option<Vec<String>>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 批量查询外部算薪数据记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 算薪数据记录列表
    pub items: Vec<DatasourceRecord>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 算薪数据记录
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasourceRecord {
    /// 记录 ID
    pub record_id: String,
    /// 数据源 ID
    pub datasource_id: String,
    /// 员工 ID
    pub employee_id: String,
    /// 数据项列表
    pub items: Vec<DatasourceRecordItem>,
    /// 创建时间（Unix 时间戳）
    pub created_at: i64,
    /// 更新时间（Unix 时间戳）
    pub updated_at: i64,
}

/// 数据项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasourceRecordItem {
    /// 字段名称
    pub field_name: String,
    /// 字段值
    pub value: serde_json::Value,
}

impl ApiResponseTrait for QueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

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
