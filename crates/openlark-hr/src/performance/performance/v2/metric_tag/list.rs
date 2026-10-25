//! 获取指标标签列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/performance-v2/metric_tag/list>

use crate::common::shared_models::I18nText;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取指标标签列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        // 1. 构建端点
        let api_endpoint = PerformanceApiV1::MetricTagList;
        let request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 2. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取指标标签列表响应数据为空",
        )
        .await
    }
}

/// 获取指标标签列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 指标标签列表
    pub items: Vec<MetricTag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 分页标记，当 has_more 为 true 时返回
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否还有更多项
    pub has_more: Option<bool>,
}

/// 指标标签信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetricTag {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 标签 ID
    pub tag_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 标签名称（i18n）
    pub tag_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 标签顺序
    pub index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间戳（单位：毫秒）
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 更新时间戳（单位：毫秒）
    pub update_time: Option<String>,
}

impl ApiResponseTrait for ListResponse {
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
    async fn test_performance_v2_metric_tag_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 使用官方响应示例的真实 shape（tag_id/tag_name/i18n），防止字段名回归
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "items": [
                    {
                        "tag_id": "7302271694582841364",
                        "tag_name": { "zh_cn": "体验", "en_us": "Interactive experience" },
                        "index": 100,
                        "create_time": "1678870702466",
                        "update_time": "1605529625257"
                    }
                ],
                "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
                "has_more": true
            }"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/performance/v2/metric_tags"))
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

        let data = ListRequest::new(config)
            .execute()
            .await
            .expect("performance_v2_metric_tag_list 应成功");

        assert_eq!(data.items.len(), 1);
        assert_eq!(data.items[0].tag_id.as_deref(), Some("7302271694582841364"));
        assert_eq!(
            data.items[0]
                .tag_name
                .as_ref()
                .and_then(|n| n.zh_cn.as_deref()),
            Some("体验")
        );
        assert_eq!(data.items[0].index, Some(100));
        assert_eq!(data.items[0].create_time.as_deref(), Some("1678870702466"));
        assert_eq!(data.items[0].update_time.as_deref(), Some("1605529625257"));
        assert_eq!(data.has_more, Some(true));
        assert!(data.page_token.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/metric_tags"
        );
    }
}
