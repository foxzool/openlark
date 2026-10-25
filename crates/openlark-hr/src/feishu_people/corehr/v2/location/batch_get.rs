//! 通过地点 ID 批量获取地点信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/location/batch_get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// BatchGetResponse
///
/// 响应 `data` 子树形状：`{items: [...]}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// 查询的地点信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<LocationItem>>,
}

/// 地点信息（官方 `location` 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocationItem {
    /// 地点 ID（飞书人事组织管理模块维护的地点记录 ID）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    /// 地点基本信息（名称、编码、启用状态、上级地点等）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hiberarchy_common: Option<HiberarchyCommon>,
    /// 地点用途
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_usage_list: Option<Vec<EnumValue>>,
    /// 地址
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<Address>>,
    /// 工时制度 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    /// 版本生效时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 版本失效时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
    /// 区域设置 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<EnumValue>,
    /// 时区
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    /// 默认显示语言
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_language_id: Option<String>,
}

/// 地点基本信息（官方 `hiberarchy_common` 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HiberarchyCommon {
    /// 上级地点 ID（一级地点不返回该字段）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 地点名称（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    /// 组织类型（固定返回 location）
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<EnumValue>,
    /// 启用状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 当前版本生效日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 当前版本失效日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 地点编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 描述（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
    /// 树形排序（该功能暂不支持，可忽略）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree_order: Option<String>,
    /// 列表排序（该功能暂不支持，可忽略）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_order: Option<String>,
    /// 自定义字段（该功能暂不支持，可忽略；官方 `object_field_data` 结构）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

/// 地址（官方 `address` 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address {
    /// 完整地址（本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_address_local_script: Option<String>,
    /// 完整地址（西方文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_address_western_script: Option<String>,
    /// 地址 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// 国家 / 地区 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    /// 主要行政区 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 地址行 1
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    /// 地址行 2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    /// 地址行 3
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line3: Option<String>,
    /// 地址行 4
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line4: Option<String>,
    /// 地址行 5
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line5: Option<String>,
    /// 地址行 6
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line6: Option<String>,
    /// 地址行 7
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line7: Option<String>,
    /// 地址行 8
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line8: Option<String>,
    /// 地址行 9
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line9: Option<String>,
    /// 地址行 1（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line1: Option<String>,
    /// 地址行 2（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line2: Option<String>,
    /// 地址行 3（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line3: Option<String>,
    /// 地址行 4（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line4: Option<String>,
    /// 地址行 5（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line5: Option<String>,
    /// 地址行 6（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line6: Option<String>,
    /// 地址行 7（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line7: Option<String>,
    /// 地址行 8（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line8: Option<String>,
    /// 地址行 9（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line9: Option<String>,
    /// 邮政编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// 地址类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_type_list: Option<Vec<EnumValue>>,
    /// 主要地址
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    /// 公开地址
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    /// 自定义字段（该功能暂不支持，可忽略）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
}

/// 官方 i18n 多语文本结构：`{lang, value}`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言（如 zh-CN / en-US）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 enum 类型字段结构：`{enum_name, display: i18n[]}`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
}

/// 官方 `object_field_data` 结构（hiberarchy_common.custom_fields）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObjectFieldData {
    /// 自定义字段 API Name，即自定义字段的唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段值，是 json 转义后的字符串
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 自定义字段名称：`{zh_cn, en_us}`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldName {
    /// 中文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 官方 `custom_field_data` 结构。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldData {
    /// 自定义字段 apiname，即自定义字段的唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    /// 自定义字段名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CustomFieldName>,
    /// 自定义字段类型
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    /// 字段值，是 json 转义后的字符串
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// BatchGetRequest
#[derive(Debug, Clone)]
pub struct BatchGetRequest {
    /// 配置信息
    config: Config,
    /// 请求体（可选）
    body: Option<Value>,
}

impl BatchGetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchGetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetResponse> {
        let mut request =
            ApiRequest::<BatchGetResponse>::post("/open-apis/corehr/v2/locations/batch_get");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

impl ApiResponseTrait for BatchGetResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/locations/batch_get
    #[tokio::test]
    async fn test_batch_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/locations/batch_get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "location_id": "4718803945687580505",
                            "hiberarchy_common": {
                                "parent_id": "4719168654814483759",
                                "name": [
                                    { "lang": "zh-CN", "value": "中文示例" }
                                ],
                                "type": {
                                    "enum_name": "location",
                                    "display": [
                                        { "lang": "zh-CN", "value": " 地点" }
                                    ]
                                },
                                "active": true,
                                "effective_time": "2020-05-01 00:00:00",
                                "expiration_time": "2020-05-02 00:00:00",
                                "code": "12456",
                                "description": [
                                    { "lang": "zh-CN", "value": "中文示例" }
                                ],
                                "tree_order": "123",
                                "list_order": "123",
                                "custom_fields": [
                                    { "field_name": "name", "value": "\"Sandy\"" }
                                ]
                            },
                            "location_usage_list": [
                                {
                                    "enum_name": "work_location",
                                    "display": [
                                        { "lang": "zh-CN", "value": "中文示例" }
                                    ]
                                }
                            ],
                            "address": [
                                {
                                    "full_address_local_script": "中国北京北京",
                                    "full_address_western_script": "Beijing, Beijing, China,",
                                    "address_id": "6989822217869624863",
                                    "country_region_id": "6862995757234914824",
                                    "region_id": "6863326815667095047",
                                    "address_line1": "丹佛测试地址-纽埃时区",
                                    "local_address_line1": "丹佛测试地址-纽埃时区",
                                    "postal_code": "611530",
                                    "address_type_list": [
                                        {
                                            "enum_name": "home_address",
                                            "display": [
                                                { "lang": "zh-CN", "value": "中文示例" }
                                            ]
                                        }
                                    ],
                                    "is_primary": true,
                                    "is_public": true
                                }
                            ],
                            "working_hours_type_id": "4690238309151997779",
                            "effective_time": "2020-05-01 00:00:00",
                            "expiration_time": "2020-05-02 00:00:00",
                            "custom_fields": [
                                {
                                    "custom_api_name": "name",
                                    "name": { "zh_cn": "自定义姓名", "en_us": "Custom Name" },
                                    "type": 1,
                                    "value": "\"231\""
                                }
                            ],
                            "locale": {
                                "enum_name": "zh_cn",
                                "display": [
                                    { "lang": "zh-CN", "value": "中文示例" }
                                ]
                            },
                            "time_zone_id": "123456789",
                            "display_language_id": "123456789"
                        }
                    ]
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

        let resp = BatchGetRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let item = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("items 应包含地点");
        assert_eq!(item.location_id.as_deref(), Some("4718803945687580505"));
        let common = item
            .hiberarchy_common
            .as_ref()
            .expect("hiberarchy_common 应存在");
        assert_eq!(common.parent_id.as_deref(), Some("4719168654814483759"));
        assert_eq!(common.code.as_deref(), Some("12456"));
        assert_eq!(common.active, Some(true));
        assert_eq!(
            common.r#type.as_ref().and_then(|t| t.enum_name.as_deref()),
            Some("location")
        );
        assert_eq!(
            common
                .custom_fields
                .as_ref()
                .and_then(|f| f.first())
                .and_then(|f| f.field_name.as_deref()),
            Some("name")
        );
        let address = item
            .address
            .as_ref()
            .and_then(|a| a.first())
            .expect("address 应包含地址");
        assert_eq!(address.address_id.as_deref(), Some("6989822217869624863"));
        assert_eq!(address.is_primary, Some(true));
        assert_eq!(
            item.working_hours_type_id.as_deref(),
            Some("4690238309151997779")
        );
        assert_eq!(
            item.locale.as_ref().and_then(|l| l.enum_name.as_deref()),
            Some("zh_cn")
        );
        assert_eq!(item.time_zone_id.as_deref(), Some("123456789"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
