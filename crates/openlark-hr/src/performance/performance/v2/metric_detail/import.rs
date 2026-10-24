//! 录入被评估人关键指标数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/performance-v2/metric_detail/import>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 录入被评估人关键指标数据请求
#[derive(Debug, Clone)]
pub struct ImportRequest {
    /// 周期 ID（必填，可通过获取周期接口获取）
    semester_id: String,
    /// 幂等标记（必填 query 参数 client_token，同一请求多次调用将被拦截，0~64 字符）
    client_token: String,
    /// 数据源录入人/录入记录名称（可选，默认 "API录入"）
    import_record_name: Option<String>,
    /// 指标明细列表（必填，1~50 条）
    imported_metrics: Vec<ImportedMetric>,
    /// 用户 ID 类型（可选 query 参数，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl ImportRequest {
    /// 创建请求
    pub fn new(config: Config, semester_id: String, client_token: String) -> Self {
        Self {
            semester_id,
            client_token,
            import_record_name: None,
            imported_metrics: Vec::new(),
            user_id_type: None,
            config,
        }
    }

    /// 追加一条指标明细
    pub fn add_metric_detail(mut self, metric: ImportedMetric) -> Self {
        self.imported_metrics.push(metric);
        self
    }

    /// 设置数据源录入人/录入记录名称（可选）
    pub fn import_record_name(mut self, import_record_name: String) -> Self {
        self.import_record_name = Some(import_record_name);
        self
    }

    /// 设置用户 ID 类型（可选 query 参数，open_id/union_id/user_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ImportResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ImportResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.semester_id.trim(), "semester_id");
        validate_required!(self.client_token.trim(), "client_token");
        validate_required_list!(
            self.imported_metrics,
            50,
            "imported_metrics 不能为空且不能超过 50 条"
        );

        // 2. 构建端点
        let api_endpoint = PerformanceApiV1::MetricDetailImport;
        let request = ApiRequest::<ImportResponse>::post(api_endpoint.to_url())
            .query("client_token", self.client_token)
            .query_opt("user_id_type", self.user_id_type);

        // 3. 构建请求体
        let request_body = ImportRequestBody {
            semester_id: self.semester_id,
            import_record_name: self.import_record_name,
            imported_metrics: self.imported_metrics,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "录入关键指标数据响应数据为空",
        )
        .await
    }
}

/// 录入请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportRequestBody {
    /// 周期 ID
    pub semester_id: String,
    /// 数据源录入人/录入记录名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_record_name: Option<String>,
    /// 指标明细列表
    pub imported_metrics: Vec<ImportedMetric>,
}

/// 指标明细（imported_metric）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportedMetric {
    /// 被评估人 ID（必填，与入参 user_id_type 类型一致）
    pub reviewee_user_id: String,
    /// 指标 ID（必填，可通过获取指标列表接口获取）
    pub metric_id: String,
    /// 指标字段信息（必填，1~99 条）
    pub fields: Vec<ImportedMetricField>,
}

/// 指标字段（imported_metric_field）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportedMetricField {
    /// 指标字段 ID（必填，可通过获取指标字段列表接口获取）
    pub field_id: String,
    /// 字段值（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
    /// 字段值，当字段为人员信息时必填（可选，与入参 user_id_type 类型一致）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_value_person: Option<String>,
}

/// 录入被评估人关键指标数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportResponse {
    /// 导入记录 ID
    pub import_record_id: String,
}

impl ApiResponseTrait for ImportResponse {
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

    fn sample_metric() -> ImportedMetric {
        ImportedMetric {
            reviewee_user_id: "ou_3245842393d09e9428ad4655da6e30b3".to_string(),
            metric_id: "7272580325522276372".to_string(),
            fields: vec![ImportedMetricField {
                field_id: "7283776005142675476".to_string(),
                field_value: Some("100".to_string()),
                field_value_person: Some("ou_3245842393d09e9428ad4655da6e30b3".to_string()),
            }],
        }
    }

    #[test]
    fn test_metric_detail_import_request_builder() {
        let request = ImportRequest::new(
            create_test_config(),
            "7293040702907514899".to_string(),
            "12454646".to_string(),
        )
        .add_metric_detail(sample_metric());

        assert_eq!(request.semester_id, "7293040702907514899");
        assert_eq!(request.imported_metrics.len(), 1);
        assert_eq!(request.imported_metrics[0].metric_id, "7272580325522276372");
    }

    #[test]
    fn test_metric_detail_import_request_body_serialize() {
        let body = ImportRequestBody {
            semester_id: "7293040702907514899".to_string(),
            import_record_name: Some("API录入".to_string()),
            imported_metrics: vec![sample_metric()],
        };

        let value = serde_json::to_value(body).expect("序列化请求体失败");
        assert_eq!(value["semester_id"], json!("7293040702907514899"));
        assert_eq!(value["import_record_name"], json!("API录入"));
        assert_eq!(
            value["imported_metrics"][0]["reviewee_user_id"],
            json!("ou_3245842393d09e9428ad4655da6e30b3")
        );
        assert_eq!(
            value["imported_metrics"][0]["fields"][0]["field_id"],
            json!("7283776005142675476")
        );
        assert!(value.get("cycle_id").is_none());
        assert!(value.get("metric_details").is_none());
    }

    #[test]
    fn test_metric_detail_import_response_deserialize() {
        let value = json!({"import_record_id": "7241404194141224979"});
        let response: ImportResponse = serde_json::from_value(value).expect("反序列化响应失败");
        assert_eq!(response.import_record_id, "7241404194141224979");
    }

    #[test]
    fn test_metric_detail_import_validation() {
        let request = ImportRequest::new(create_test_config(), "   ".to_string(), "tk".to_string());
        let result: SDKResult<()> = (|| {
            validate_required!(request.semester_id.trim(), "semester_id");
            Ok(())
        })();
        assert!(result.is_err());
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_performance_v2_metric_detail_import_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"import_record_id": "7241404194141224979"}"#).unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v2/metric_details/import"))
            .and(query_param("client_token", "12454646"))
            .and(body_json(json!({
                "semester_id": "7293040702907514899",
                "import_record_name": "API录入",
                "imported_metrics": [{
                    "reviewee_user_id": "ou_3245842393d09e9428ad4655da6e30b3",
                    "metric_id": "7272580325522276372",
                    "fields": [{
                        "field_id": "7283776005142675476",
                        "field_value": "100",
                        "field_value_person": "ou_3245842393d09e9428ad4655da6e30b3"
                    }]
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

        let data = ImportRequest::new(
            config,
            "7293040702907514899".to_string(),
            "12454646".to_string(),
        )
        .import_record_name("API录入".to_string())
        .add_metric_detail(sample_metric())
        .execute()
        .await
        .expect("performance_v2_metric_detail_import 应成功");

        assert_eq!(data.import_record_id, "7241404194141224979");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/metric_details/import"
        );
    }
}
