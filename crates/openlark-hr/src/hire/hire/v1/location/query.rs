//! 查询地点列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/location/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 地点名称信息聚合（官方 `location_name_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocationNameInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 中文名。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 英文名。
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 拼音名。
    pub py_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 国家信息（官方 `location_country`，仅当 location_type=1 时返回）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocationCountry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 国家 Code 码。
    pub country_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 国家名称信息聚合。
    pub country_name_info: Option<LocationNameInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 省份/州信息（官方 `location_state`，仅当 location_type=2 时返回）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocationState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 省份/州 Code 码。
    pub state_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 国家 Code 码。
    pub country_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 省份/州名称信息聚合。
    pub state_name_info: Option<LocationNameInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 市信息（官方 `location_city`，仅当 location_type=3 时返回）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocationCity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 市 Code 码。
    pub city_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 省份/州 Code 码。
    pub state_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 国家 Code 码。
    pub country_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 市名称信息聚合。
    pub city_name_info: Option<LocationNameInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 区/县信息（官方 `location_district`，仅当 location_type=4 时返回）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocationDistrict {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 区/县 Code 码。
    pub district_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 市 Code 码。
    pub city_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 省份/州 Code 码。
    pub state_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 国家 Code 码。
    pub country_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 区/县名称信息聚合。
    pub district_name_info: Option<LocationNameInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 查询地点列表项（官方 `location_dto`，按 location_type 条件返回对应容器）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryLocationItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 国家信息，仅当 location_type=1 时返回。
    pub country: Option<LocationCountry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 省份/州信息，仅当 location_type=2 时返回。
    pub state: Option<LocationState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 市信息，仅当 location_type=3 时返回。
    pub city: Option<LocationCity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 区/县信息，仅当 location_type=4 时返回。
    pub district: Option<LocationDistrict>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 查询地点列表请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        let mut request = ApiRequest::<QueryResponse>::post("/open-apis/hire/v1/locations/query");
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询地点列表响应数据为空",
        )
        .await
    }
}

/// 查询地点列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryResponse {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<QueryLocationItem>,
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

impl ApiResponseTrait for QueryResponse {
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

    /// 端到端：POST /open-apis/hire/v1/locations/query
    #[tokio::test]
    async fn test_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/locations/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [{
                        "country": {
                            "country_code": "CN_1",
                            "country_name_info": {
                                "zh_name": "成都",
                                "en_name": "Chinese Mainland",
                                "py_name": "zhongguodalu"
                            }
                        },
                        "state": {
                            "state_code": "ST_1",
                            "country_code": "CN_1",
                            "state_name_info": {
                                "zh_name": "成都",
                                "en_name": "Liaoning",
                                "py_name": "liaoning"
                            }
                        }
                    }],
                    "has_more": true
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

        let resp = QueryRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        // 官方 query item 为 country/state/city/district 嵌套容器（.md L61-91）
        let item = resp.items.first().expect("items 应反序列化");
        assert_eq!(
            item.country
                .as_ref()
                .and_then(|v| v.country_code.as_deref()),
            Some("CN_1")
        );
        assert_eq!(
            item.country
                .as_ref()
                .and_then(|v| v.country_name_info.as_ref())
                .and_then(|v| v.py_name.as_deref()),
            Some("zhongguodalu")
        );
        assert_eq!(
            item.state.as_ref().and_then(|v| v.state_code.as_deref()),
            Some("ST_1")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
