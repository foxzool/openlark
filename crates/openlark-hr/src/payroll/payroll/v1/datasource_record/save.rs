//! 创建 / 更新外部算薪数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/datasource_record/save>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 创建 / 更新外部算薪数据请求
#[derive(Debug, Clone)]
pub struct SaveRequest {
    /// 数据源 code（必填，可从「获取外部数据源配置信息」API 或
    /// 「飞书人事后台-设置-算薪数据设置-外部数据源配置」页面获取）
    source_code: String,
    /// 需保存的记录列表（必填，1~50 条）
    records: Vec<DatasourceRecord>,
    /// 配置信息
    config: Config,
}

impl SaveRequest {
    /// 创建请求
    pub fn new(config: Config, source_code: String, records: Vec<DatasourceRecord>) -> Self {
        Self {
            source_code,
            records,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SaveResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SaveResponse> {
        use crate::common::api_endpoints::PayrollApiV1;

        // 1. 验证必填字段
        validate_required!(self.source_code, "source_code 不能为空");
        validate_required_list!(self.records, 50, "records 不能为空且不能超过 50 条");

        // 2. 构建端点
        let api_endpoint = PayrollApiV1::DatasourceRecordSave;
        let request = ApiRequest::<SaveResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = SaveRequestBody {
            source_code: self.source_code,
            records: self.records,
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
            "创建/更新外部算薪数据响应数据为空",
        )
        .await
    }
}

/// 创建 / 更新外部算薪数据请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveRequestBody {
    /// 数据源 code
    pub source_code: String,
    /// 需保存的记录列表（1~50 条）
    pub records: Vec<DatasourceRecord>,
}

/// 外部算薪数据记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasourceRecord {
    /// 记录的启停用状态：1-启用 2-停用（停用后依旧可被 API 保存、查询，但无法被算薪使用）
    pub active_status: i32,
    /// 记录的字段值列表
    pub field_values: Vec<DatasourceRecordFieldValue>,
}

/// 数据源字段值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasourceRecordFieldValue {
    /// 数据源字段编码，请确保字段存在且是启用的
    pub field_code: String,
    /// 字段值，通过 string 传输，不允许空字符串
    pub value: String,
    /// 字段类型（无需传入，仅文档说明用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<i32>,
}

/// 创建 / 更新外部算薪数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SaveResponse {
    /// 变更的记录条数（新建或更新），官方以 string 传输的数字
    pub affect_counts: String,
}

impl ApiResponseTrait for SaveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use serde_json::json;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    fn build_records() -> Vec<DatasourceRecord> {
        vec![DatasourceRecord {
            active_status: 1,
            field_values: vec![DatasourceRecordFieldValue {
                field_code: "base_salary__c".to_string(),
                value: "10000".to_string(),
                field_type: None,
            }],
        }]
    }

    #[test]
    fn test_payroll_save_request_builder() {
        let request =
            SaveRequest::new(create_test_config(), "test__c".to_string(), build_records());

        assert_eq!(request.source_code, "test__c");
        assert_eq!(request.records.len(), 1);
    }

    #[test]
    fn test_payroll_save_request_body_serialize() {
        let body = SaveRequestBody {
            source_code: "yache19_8680__c".to_string(),
            records: vec![DatasourceRecord {
                active_status: 1,
                field_values: vec![DatasourceRecordFieldValue {
                    field_code: "bonus__c".to_string(),
                    value: "888".to_string(),
                    field_type: None,
                }],
            }],
        };

        let value = serde_json::to_value(body).expect("序列化请求体失败");
        assert_eq!(value["source_code"], json!("yache19_8680__c"));
        assert_eq!(value["records"][0]["active_status"], json!(1));
        assert_eq!(
            value["records"][0]["field_values"][0]["field_code"],
            json!("bonus__c")
        );
        assert_eq!(
            value["records"][0]["field_values"][0]["value"],
            json!("888")
        );
    }

    #[test]
    fn test_payroll_save_response_deserialize() {
        let value = json!({"affect_counts": "120"});
        let response: SaveResponse = serde_json::from_value(value).expect("反序列化响应失败");

        assert_eq!(response.affect_counts, "120");
    }

    #[test]
    fn test_payroll_save_validation() {
        let request = SaveRequest::new(create_test_config(), "  ".to_string(), build_records());
        let result: SDKResult<()> = (|| {
            validate_required!(request.source_code, "source_code");
            validate_required_list!(request.records, 50, "records 不能为空且不能超过 50 条");
            Ok(())
        })();

        assert!(result.is_err());
    }

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_save_datasource_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应：data 仅 affect_counts
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"affect_counts": "1"}"#).unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/payroll/v1/datasource_records/save"))
            .and(body_json(json!({
                "source_code": "test__c",
                "records": [{
                    "active_status": 1,
                    "field_values": [{"field_code": "amount__c", "value": "100"}]
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

        let data = SaveRequest::new(
            config,
            "test__c".to_string(),
            vec![DatasourceRecord {
                active_status: 1,
                field_values: vec![DatasourceRecordFieldValue {
                    field_code: "amount__c".to_string(),
                    value: "100".to_string(),
                    field_type: None,
                }],
            }],
        )
        .execute()
        .await
        .expect("保存数据源记录应成功");

        assert_eq!(data.affect_counts, "1");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/payroll/v1/datasource_records/save"
        );
    }
}
