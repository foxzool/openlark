//! 更新地点
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/location/patch>

use crate::common::shared_models::I18nLangValue;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use serde::{Deserialize, Serialize};

/// 枚举引用（官方 `location_usages` / `locale` 结构：`{enum_name}`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocationEnumValue {
    /// 枚举值（必填）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
}

/// 更新地点请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchBody {
    /// 版本生效日期（必填，格式：YYYY-MM-DD，范围 1900-01-01 ～ 9999-12-31）
    pub effective_time: String,
    /// 上级地点，在创建场景下该字段必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 地点名称（i18n 多语，最多 5 条；不能包含「/」「；」「;」「\」「'」字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<I18nLangValue>>,
    /// 地点启用/停用状态（true=启用，false=停用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 地点编码（不能与其他记录的编码重复；开启自动编码时该字段会失效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 地点描述（i18n 多语，最多 5 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<I18nLangValue>>,
    /// 地点用途（最多 10 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_usages: Option<Vec<LocationEnumValue>>,
    /// 工时制度 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    /// 区域设置 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<LocationEnumValue>,
    /// 时区 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    /// 默认显示语言 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_language_id: Option<String>,
}

/// 更新地点响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 地点信息
    pub data: Option<PatchResponseData>,
}

/// `PatchResponseData`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponseData {
    /// 地点 ID
    pub id: String,
    /// 地点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 地点描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 地址信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl ApiResponseTrait for PatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新地点请求构建器
#[derive(Debug, Clone)]
pub struct PatchRequestBuilder {
    /// 配置信息
    config: Config,
    /// 地点 ID（路径参数）
    location_id: String,
    /// 请求体
    body: PatchBody,
    /// 根据 client_token 是否一致来判断是否为同一请求
    client_token: Option<String>,
}

impl PatchRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config, location_id: String) -> Self {
        Self {
            config,
            location_id,
            body: PatchBody::default(),
            client_token: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: PatchBody) -> Self {
        self.body = body;
        self
    }

    /// 设置版本生效日期（必填，格式：YYYY-MM-DD）。
    pub fn effective_time(mut self, effective_time: impl Into<String>) -> Self {
        self.body.effective_time = effective_time.into();
        self
    }

    /// 设置上级地点。
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.body.parent_id = Some(parent_id.into());
        self
    }

    /// 设置地点名称（i18n 多语）。
    pub fn names(mut self, names: Vec<I18nLangValue>) -> Self {
        self.body.names = Some(names);
        self
    }

    /// 设置启用/停用状态（true=启用，false=停用）。
    pub fn active(mut self, active: bool) -> Self {
        self.body.active = Some(active);
        self
    }

    /// 设置地点编码。
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.body.code = Some(code.into());
        self
    }

    /// 设置地点描述（i18n 多语）。
    pub fn descriptions(mut self, descriptions: Vec<I18nLangValue>) -> Self {
        self.body.descriptions = Some(descriptions);
        self
    }

    /// 设置地点用途。
    pub fn location_usages(mut self, location_usages: Vec<LocationEnumValue>) -> Self {
        self.body.location_usages = Some(location_usages);
        self
    }

    /// 设置工时制度 ID。
    pub fn working_hours_type_id(mut self, working_hours_type_id: impl Into<String>) -> Self {
        self.body.working_hours_type_id = Some(working_hours_type_id.into());
        self
    }

    /// 设置区域设置。
    pub fn locale(mut self, locale: LocationEnumValue) -> Self {
        self.body.locale = Some(locale);
        self
    }

    /// 设置时区 ID。
    pub fn time_zone_id(mut self, time_zone_id: impl Into<String>) -> Self {
        self.body.time_zone_id = Some(time_zone_id.into());
        self
    }

    /// 设置默认显示语言 ID。
    pub fn display_language_id(mut self, display_language_id: impl Into<String>) -> Self {
        self.body.display_language_id = Some(display_language_id.into());
        self
    }

    /// 设置幂等 client_token。
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        validate_required!(self.body.effective_time, "effective_time 不能为空");

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::LocationPatch(self.location_id.clone());
        let request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url())
            .query_opt("client_token", self.client_token)
            .body(serde_json::to_value(&self.body).map_err(|e| {
                openlark_core::error::validation_error(
                    "请求体序列化失败",
                    format!("无法序列化请求参数: {e}"),
                )
            })?);

        // 发送请求
        Transport::request_typed(request, &self.config, Some(option), "更新地点响应数据为空").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH /open-apis/corehr/v2/locations/test001
    /// （官方请求体必填 effective_time，可选 parent_id/names/active/code 等）
    #[tokio::test]
    async fn test_patch_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/corehr/v2/locations/test001"))
            .and(query_param("client_token", "12454646"))
            .and(body_json(json!({
                "parent_id": "4719168654814483759",
                "names": [
                    { "lang": "zh-CN", "value": "中文示例" }
                ],
                "active": true,
                "effective_time": "2020-05-02",
                "code": "12456",
                "descriptions": [
                    { "lang": "zh-CN", "value": "中文示例" }
                ],
                "location_usages": [
                    { "enum_name": "phone_type" }
                ],
                "working_hours_type_id": "4690238309151997779",
                "locale": { "enum_name": "phone_type" },
                "time_zone_id": "123456789",
                "display_language_id": "123456789"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        PatchRequestBuilder::new(config, "test001".to_string())
            .parent_id("4719168654814483759")
            .names(vec![I18nLangValue {
                lang: Some("zh-CN".to_string()),
                value: Some("中文示例".to_string()),
            }])
            .active(true)
            .effective_time("2020-05-02")
            .code("12456")
            .descriptions(vec![I18nLangValue {
                lang: Some("zh-CN".to_string()),
                value: Some("中文示例".to_string()),
            }])
            .location_usages(vec![LocationEnumValue {
                enum_name: Some("phone_type".to_string()),
            }])
            .working_hours_type_id("4690238309151997779")
            .locale(LocationEnumValue {
                enum_name: Some("phone_type".to_string()),
            })
            .time_zone_id("123456789")
            .display_language_id("123456789")
            .client_token("12454646")
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
