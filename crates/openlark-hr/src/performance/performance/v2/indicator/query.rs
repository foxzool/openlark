//! 获取评估项列表
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/indicator/query>

use crate::common::shared_models::I18nText;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取评估项列表请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 评估项 ID 列表（可选，≤50，不填返回所有）
    indicator_ids: Option<Vec<String>>,
    /// 分页标记（可选 query 参数）
    page_token: Option<String>,
    /// 分页大小（可选 query 参数，默认 20，0～50）
    page_size: Option<i32>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            indicator_ids: None,
            page_token: None,
            page_size: None,
            config,
        }
    }

    /// 设置评估项 ID 列表（可选，≤50）
    pub fn indicator_ids(mut self, indicator_ids: Vec<String>) -> Self {
        self.indicator_ids = Some(indicator_ids);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置分页大小（可选，默认 20，0～50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
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
        let api_endpoint = PerformanceApiV1::IndicatorQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url())
            .query_opt("page_token", self.page_token)
            .query_opt("page_size", self.page_size.map(|v| v.to_string()));

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            indicator_ids: self.indicator_ids,
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
            "获取评估项列表响应数据为空",
        )
        .await
    }
}

/// 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 评估项 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indicator_ids: Option<Vec<String>>,
}

/// 获取评估项列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryResponse {
    /// 评估项列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicators: Option<Vec<Indicator>>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记（当 has_more 为 true 时返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 评估项（官方 indicator 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Indicator {
    /// 评估项 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 评估项名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 评估项类型：general_review_item / review_item_based_on_key_metric / okr_review_item / plus / minus
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 评估项等级列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<IndicatorOption>>,
}

/// 评估项等级（官方 indicator_option 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct IndicatorOption {
    /// 评估项等级 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 评估项等级名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 评估项等级代号（官方文档字段名即为 lable）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lable: Option<String>,
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
    async fn test_performance_v2_indicator_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 使用官方响应示例的真实 shape（indicators 容器 + i18n name + lable）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "indicators": [
                    {
                        "id": "7343513161666707459",
                        "name": { "zh_cn": "完成度", "en_us": "Completeness" },
                        "type": "plus",
                        "options": [
                            {
                                "id": "7343513161666707459",
                                "name": { "zh_cn": "等级 1", "en_us": "Rank 1" },
                                "lable": "qwerty"
                            }
                        ]
                    }
                ],
                "has_more": true,
                "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0="
            }"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v2/indicators/query"))
            .and(body_json(json!({
                "indicator_ids": ["7343513161666707459"]
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
            .indicator_ids(vec!["7343513161666707459".to_string()])
            .page_size(30)
            .execute()
            .await
            .expect("performance_v2_indicator_query 应成功");

        let indicators = data.indicators.as_ref().expect("indicators 应存在");
        assert_eq!(indicators.len(), 1);
        assert_eq!(indicators[0].id.as_deref(), Some("7343513161666707459"));
        assert_eq!(
            indicators[0].name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("完成度")
        );
        assert_eq!(indicators[0].r#type.as_deref(), Some("plus"));
        let options = indicators[0].options.as_ref().expect("options 应存在");
        assert_eq!(options[0].lable.as_deref(), Some("qwerty"));
        assert_eq!(
            options[0].name.as_ref().and_then(|n| n.en_us.as_deref()),
            Some("Rank 1")
        );
        assert_eq!(data.has_more, Some(true));
        assert!(data.page_token.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/indicators/query"
        );
    }
}
