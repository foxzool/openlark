//! 批量查询外部算薪数据记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/datasource_record/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 批量查询外部算薪数据记录请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 数据源 code（必填）
    source_code: Option<String>,
    /// 指定查询的数据源字段 code（可选，不传默认返回所有数据源字段）
    selected_fields: Option<Vec<String>>,
    /// 查询条件列表（可选，多个条件之间为 And 关系）
    field_filters: Option<Vec<DatasourceRecordFieldFilter>>,
    /// 分页大小（必填，取值范围 1~100，URL query 参数）
    page_size: Option<i32>,
    /// 分页标记（可选，URL query 参数）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            source_code: None,
            selected_fields: None,
            field_filters: None,
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置数据源 code（必填）
    pub fn source_code(mut self, source_code: String) -> Self {
        self.source_code = Some(source_code);
        self
    }

    /// 设置指定查询的数据源字段 code（可选）
    pub fn selected_fields(mut self, selected_fields: Vec<String>) -> Self {
        self.selected_fields = Some(selected_fields);
        self
    }

    /// 设置查询条件列表（可选，多个条件之间为 And 关系）
    pub fn field_filters(mut self, field_filters: Vec<DatasourceRecordFieldFilter>) -> Self {
        self.field_filters = Some(field_filters);
        self
    }

    /// 设置分页大小（必填，取值范围 1~100）
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

        // 1. 验证必填字段
        let source_code = self
            .source_code
            .ok_or_else(|| validation_error("source_code 不能为空", "数据源 code 必填"))?;
        validate_required!(source_code, "source_code 不能为空");
        let page_size = self.page_size.ok_or_else(|| {
            validation_error("page_size 不能为空", "分页大小必填，取值范围 1~100")
        })?;
        if !(1..=100).contains(&page_size) {
            return Err(validation_error(
                "page_size 超出范围",
                "分页大小取值范围为 1~100",
            ));
        }

        // 2. 构建端点与 query 参数（官方分页参数为 URL query 参数）
        let api_endpoint = PayrollApiV1::DatasourceRecordQuery;
        let mut request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());
        request = request.query("page_size", page_size.to_string());
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 3. 构建请求体
        let request_body = QueryRequestBody {
            source_code,
            selected_fields: self.selected_fields,
            field_filters: self.field_filters,
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
            "批量查询外部算薪数据记录响应数据为空",
        )
        .await
    }
}

/// 批量查询外部算薪数据记录请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 数据源 code
    pub source_code: String,
    /// 指定查询的数据源字段 code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_fields: Option<Vec<String>>,
    /// 查询条件列表，多个条件之间为 And 关系
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_filters: Option<Vec<DatasourceRecordFieldFilter>>,
}

/// 数据源记录查询条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasourceRecordFieldFilter {
    /// 字段编码（如 employment_id、payroll_period）
    pub field_code: String,
    /// 包含的字段值列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_values: Option<Vec<String>>,
    /// 查询操作符：1-IsAnyOf 包含查询（默认） 2-InDataRange
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<i32>,
}

/// 批量查询外部算薪数据记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 分页标记，当 has_more 为 true 时返回，否则不返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 数据记录列表
    pub records: Vec<DatasourceRecord>,
}

/// 外部算薪数据记录
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasourceRecord {
    /// 记录的启停用状态：1-启用 2-停用
    pub active_status: i32,
    /// 记录的字段值列表
    pub field_values: Vec<DatasourceRecordFieldValue>,
}

/// 数据源字段值
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasourceRecordFieldValue {
    /// 数据源字段编码
    pub field_code: String,
    /// 字段值，值为 API 写入的值，通过 string 传输
    pub value: String,
    /// 字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<i32>,
}

impl ApiResponseTrait for QueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_query_datasource_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应形状：records + 分页字段
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "page_token": "7413047003142619148",
                "has_more": true,
                "records": [{
                    "active_status": 1,
                    "field_values": [
                        {"field_code": "yache41_8680__c", "value": "2024-12-01"},
                        {"field_code": "yache42_8680__c", "value": "100", "field_type": 2}
                    ]
                }]
            }"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/payroll/v1/datasource_records/query"))
            .and(query_param("page_size", "10"))
            .and(body_json(json!({
                "source_code": "yache19_8680__c",
                "selected_fields": ["yache41_8680__c"],
                "field_filters": [{
                    "field_code": "employment_id",
                    "field_values": ["7134995965475620396"],
                    "operator": 1
                }]
            })))
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

        let data = QueryRequest::new(config)
            .source_code("yache19_8680__c".to_string())
            .selected_fields(vec!["yache41_8680__c".to_string()])
            .field_filters(vec![DatasourceRecordFieldFilter {
                field_code: "employment_id".to_string(),
                field_values: Some(vec!["7134995965475620396".to_string()]),
                operator: Some(1),
            }])
            .page_size(10)
            .execute()
            .await
            .expect("查询数据源记录应成功");

        assert!(data.has_more);
        assert_eq!(data.records.len(), 1);
        assert_eq!(data.records[0].active_status, 1);
        assert_eq!(
            data.records[0].field_values[0].field_code,
            "yache41_8680__c"
        );
        assert_eq!(data.records[0].field_values[0].value, "2024-12-01");
        assert_eq!(data.records[0].field_values[1].field_type, Some(2));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/payroll/v1/datasource_records/query"
        );
    }

    /// source_code / page_size 为必填，缺失时应校验失败
    #[tokio::test]
    async fn test_query_datasource_records_requires_source_code_and_page_size() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url("https://127.0.0.1:9")
            .enable_token_cache(false)
            .build();

        let result = QueryRequest::new(config.clone())
            .page_size(10)
            .execute()
            .await;
        assert!(result.is_err());

        let result = QueryRequest::new(config)
            .source_code("test__c".to_string())
            .execute()
            .await;
        assert!(result.is_err());
    }
}
