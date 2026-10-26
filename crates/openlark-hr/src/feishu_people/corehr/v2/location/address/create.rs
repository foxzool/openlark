//! 添加地点地址
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/location.address/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 地址类型（官方 `address_types` 数组元素：`{enum_name}`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AddressType {
    /// 枚举值（必填）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
}

/// 添加地点地址请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBody {
    /// 国家 / 地区 ID（必填），可通过查询国家/地区信息接口获取
    pub country_region_id: String,
    /// 主要行政区 ID（必填），可通过查询省份/行政区信息接口获取
    pub region_id: String,
    /// 城市 ID（必填），可通过查询城市信息接口获取
    pub city_id: String,
    /// 区/县 ID（必填），可通过查询区县信息接口获取
    pub distinct_id: String,
    /// 地址行 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    /// 地址行 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    /// 地址行 3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line3: Option<String>,
    /// 地址行 4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line4: Option<String>,
    /// 地址行 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line5: Option<String>,
    /// 地址行 6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line6: Option<String>,
    /// 地址行 7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line7: Option<String>,
    /// 地址行 8
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line8: Option<String>,
    /// 地址行 9
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line9: Option<String>,
    /// 地址行 1（非拉丁语系的本地文字）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_address_line1: Option<String>,
    /// 地址行 2（非拉丁语系的本地文字）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_address_line2: Option<String>,
    /// 地址行 3（非拉丁语系的本地文字）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_address_line3: Option<String>,
    /// 地址行 4（非拉丁语系的本地文字）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_address_line4: Option<String>,
    /// 地址行 5（非拉丁语系的本地文字）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_address_line5: Option<String>,
    /// 地址行 6（非拉丁语系的本地文字）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_address_line6: Option<String>,
    /// 地址行 7（非拉丁语系的本地文字）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_address_line7: Option<String>,
    /// 地址行 8（非拉丁语系的本地文字）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_address_line8: Option<String>,
    /// 地址行 9（非拉丁语系的本地文字）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_address_line9: Option<String>,
    /// 邮政编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// 地址类型（最多 5 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_types: Option<Vec<AddressType>>,
    /// 是否主要地址，一个地点只能存在一个主要地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    /// 是否公开地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

/// CreateRequest
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    location_id: Option<String>,
    /// 请求体
    body: CreateBody,
    /// 根据 client_token 是否一致来判断是否为同一请求
    client_token: Option<String>,
    /// 是否需要自定义传入国际化拉丁语系地址
    need_custom_latin_address: Option<bool>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            location_id: None,
            body: CreateBody::default(),
            client_token: None,
            need_custom_latin_address: None,
        }
    }

    /// 设置 `location_id`。
    pub fn location_id(mut self, location_id: impl Into<String>) -> Self {
        self.location_id = Some(location_id.into());
        self
    }

    /// 设置请求体。
    pub fn body(mut self, body: CreateBody) -> Self {
        self.body = body;
        self
    }

    /// 设置国家 / 地区 ID（必填）。
    pub fn country_region_id(mut self, country_region_id: impl Into<String>) -> Self {
        self.body.country_region_id = country_region_id.into();
        self
    }

    /// 设置主要行政区 ID（必填）。
    pub fn region_id(mut self, region_id: impl Into<String>) -> Self {
        self.body.region_id = region_id.into();
        self
    }

    /// 设置城市 ID（必填）。
    pub fn city_id(mut self, city_id: impl Into<String>) -> Self {
        self.body.city_id = city_id.into();
        self
    }

    /// 设置区/县 ID（必填）。
    pub fn distinct_id(mut self, distinct_id: impl Into<String>) -> Self {
        self.body.distinct_id = distinct_id.into();
        self
    }

    /// 设置地址类型。
    pub fn address_types(mut self, address_types: Vec<AddressType>) -> Self {
        self.body.address_types = Some(address_types);
        self
    }

    /// 设置是否主要地址。
    pub fn is_primary(mut self, is_primary: bool) -> Self {
        self.body.is_primary = Some(is_primary);
        self
    }

    /// 设置是否公开地址。
    pub fn is_public(mut self, is_public: bool) -> Self {
        self.body.is_public = Some(is_public);
        self
    }

    /// 设置幂等 client_token。
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    /// 设置是否需要自定义传入国际化拉丁语系地址。
    pub fn need_custom_latin_address(mut self, need_custom_latin_address: bool) -> Self {
        self.need_custom_latin_address = Some(need_custom_latin_address);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        let location_id = self.location_id.unwrap_or_default();
        validate_required!(location_id.trim(), "location_id 不能为空");
        validate_required!(self.body.country_region_id, "country_region_id 不能为空");
        validate_required!(self.body.region_id, "region_id 不能为空");
        validate_required!(self.body.city_id, "city_id 不能为空");
        validate_required!(self.body.distinct_id, "distinct_id 不能为空");

        let request = ApiRequest::<CreateResponse>::post(format!(
            "/open-apis/corehr/v2/locations/{location_id}/addresses"
        ))
        .query_opt("client_token", self.client_token)
        .query_opt(
            "need_custom_latin_address",
            self.need_custom_latin_address.map(|v| v.to_string()),
        )
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// CreateResponse
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 地址 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
}

impl ApiResponseTrait for CreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/locations/test001/addresses
    /// （官方请求体必填 country_region_id/region_id/city_id/distinct_id；data 内层键为 address_id）
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/locations/test001/addresses"))
            .and(query_param("client_token", "12454646"))
            .and(body_json(json!({
                "country_region_id": "6862995757234914824",
                "region_id": "6863326264296474119",
                "city_id": "6863333555859097096",
                "distinct_id": "6863333556291110408",
                "address_line1": "丹佛测试地址-纽埃时区",
                "local_address_line1": "丹佛测试地址-纽埃时区",
                "postal_code": "611530",
                "address_types": [
                    { "enum_name": "phone_type" }
                ],
                "is_primary": true,
                "is_public": true
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "address_id": "6862995757234914824"
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

        let resp = CreateRequest::new(config)
            .location_id("test001".to_string())
            .body(CreateBody {
                country_region_id: "6862995757234914824".to_string(),
                region_id: "6863326264296474119".to_string(),
                city_id: "6863333555859097096".to_string(),
                distinct_id: "6863333556291110408".to_string(),
                address_line1: Some("丹佛测试地址-纽埃时区".to_string()),
                local_address_line1: Some("丹佛测试地址-纽埃时区".to_string()),
                postal_code: Some("611530".to_string()),
                address_types: Some(vec![AddressType {
                    enum_name: Some("phone_type".to_string()),
                }]),
                is_primary: Some(true),
                is_public: Some(true),
                ..CreateBody::default()
            })
            .client_token("12454646")
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.address_id.as_deref(), Some("6862995757234914824"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
