//! 获取指标字段列表
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/metric_field/query>

use crate::common::shared_models::I18nText;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取指标字段列表请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 指标的字段 ID 列表（可选，≤50，不填获取全部指标字段）
    field_ids: Option<Vec<String>>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            field_ids: None,
            config,
        }
    }

    /// 设置指标的字段 ID 列表（可选，≤50）
    pub fn field_ids(mut self, field_ids: Vec<String>) -> Self {
        self.field_ids = Some(field_ids);
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
        use crate::common::api_endpoints::PerformanceApiV1;

        // 1. 构建端点
        let api_endpoint = PerformanceApiV1::MetricFieldQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            field_ids: self.field_ids,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取指标字段列表响应数据为空",
        )
        .await
    }
}

/// 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 指标的字段 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ids: Option<Vec<String>>,
}

/// 获取指标字段列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryResponse {
    /// 指标字段信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<MetricField>>,
}

/// 指标字段信息（官方 metric_field 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetricField {
    /// 指标字段 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    /// 指标字段名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 指标字段类型：text 文本 / number 数字 / pencentage 百分比 / person 人员单选
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
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
    async fn test_performance_v2_metric_field_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 使用官方响应示例的真实 shape（field_id/i18n name/type）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "items": [
                    {
                        "field_id": "7272581996315099155",
                        "name": { "zh_cn": "销量", "en_us": "Sales" },
                        "type": "text"
                    }
                ]
            }"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v2/metric_fields/query"))
            .and(body_json(json!({
                "field_ids": ["7293169069640630291"]
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
            .field_ids(vec!["7293169069640630291".to_string()])
            .execute()
            .await
            .expect("performance_v2_metric_field_query 应成功");

        let items = data.items.as_ref().expect("items 应存在");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].field_id.as_deref(), Some("7272581996315099155"));
        assert_eq!(
            items[0].name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("销量")
        );
        assert_eq!(
            items[0].name.as_ref().and_then(|n| n.en_us.as_deref()),
            Some("Sales")
        );
        assert_eq!(items[0].r#type.as_deref(), Some("text"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/metric_fields/query"
        );
    }
}
