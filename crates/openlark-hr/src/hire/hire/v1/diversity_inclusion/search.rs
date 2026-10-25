//! 获取申请表附加信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/diversity_inclusion/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::DiversityInclusionRecord;

/// 获取申请表附加信息请求
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 配置信息
    config: Config,
}

impl SearchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        use crate::common::api_endpoints::HireApiV1;

        let api_endpoint = HireApiV1::DiversityInclusionSearch;
        let request = ApiRequest::<SearchResponse>::post(api_endpoint.to_url());

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取申请表附加信息响应数据为空",
        )
        .await
    }
}

/// 获取申请表附加信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SearchResponse {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<DiversityInclusionRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for SearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/hire/v1/applications/diversity_inclusions/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/hire/v1/applications/diversity_inclusions/search",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "id": "6784008015948283904",
                            "application_id": "6784008015948283905",
                            "talent_id": "6784008015948283906",
                            "source_type": 1,
                            "create_time": "1681301987000",
                            "update_time": "1681301987000",
                            "di_data": [
                                {
                                    "value": "1",
                                    "object_attribute": {
                                        "title": { "zh_cn": "测试", "en_us": "test" },
                                        "description": { "zh_cn": "测试", "en_us": "test" },
                                        "data_type": 1,
                                        "tags": [1],
                                        "is_fcf_data": true,
                                        "is_di_data": true
                                    }
                                }
                            ]
                        }
                    ],
                    "has_more": false
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

        let resp = SearchRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");
        let item = resp.items.first().expect("应返回 items 列表");
        assert_eq!(item.id.as_deref(), Some("6784008015948283904"));
        // 官方响应体含 source_type 与 di_data 嵌套（.md 字段表 + 响应示例）
        assert_eq!(item.source_type, Some(1));
        let di = item
            .di_data
            .as_ref()
            .and_then(|l| l.first())
            .expect("di_data 应反序列化");
        assert_eq!(di.value.as_deref(), Some("1"));
        let attr = di
            .object_attribute
            .as_ref()
            .expect("object_attribute 应反序列化");
        assert_eq!(attr.data_type, Some(1));
        assert_eq!(attr.tags.as_deref(), Some(&[1][..]));
        assert_eq!(attr.is_fcf_data, Some(true));
        assert_eq!(attr.is_di_data, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
