//! 搜索待入职信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/pre_hire/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `SearchRequest` 请求。
#[derive(Debug, Clone)]
pub struct SearchRequest {
    config: Config,
    request_body: Option<Value>,
}

impl SearchRequest {
    /// 创建新的请求实例。
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

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let api_endpoint = FeishuPeopleApiV2::PreHireSearch;
        let mut request = ApiRequest::<SearchResponse>::post(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "搜索待入职信息响应数据为空",
        )
        .await
    }
}

/// `SearchResponse` 响应。
///
/// 响应 `data` 子树形状：`{items: pre_hire[], page_token, has_more}`
/// （由 decode 层解包）。请求 `fields` 参数为空时仅返回 `pre_hire_id`，
/// 故条目字段均为 Option。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 查询的待入职信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PreHireItem>>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 待入职信息（官方 `pre_hire` 结构；本期先落 `pre_hire_id` / `person_info`
/// 骨架，employment_info / onboarding_info / probation_info / contract_info
/// 等容器后续按文档全表补齐）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreHireItem {
    /// 待入职 ID（`fields` 为空时唯一返回字段）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_hire_id: Option<String>,
    /// 个人信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_info: Option<PersonInfo>,
}

/// 个人信息（官方 `person_info` 结构的核心字段子集）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersonInfo {
    /// 个人信息 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    /// 个人电话（需 corehr:person.phone:read / write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// 法定姓名（需 corehr:person.legal_name:read / write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    /// 常用名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_name: Option<String>,
    /// 常用本地全名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_local_full_name: Option<String>,
    /// 常用英文全名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_english_full_name: Option<String>,
    /// 姓名列表（需 corehr:person.legal_name:read / write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_list: Option<Vec<PersonName>>,
}

/// 姓名信息（官方 `person_name` 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersonName {
    /// 姓 - 本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_primary: Option<String>,
    /// 名 - 本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_first_name: Option<String>,
    /// 国家 / 地区 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    /// 姓名类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_type: Option<EnumValue>,
    /// 名 - 第二本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_first_name_2: Option<String>,
    /// 姓 - 第二本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_primary_2: Option<String>,
    /// 别名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<String>,
    /// 补充姓名类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_name_type: Option<EnumValue>,
    /// 名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// 全名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// 姓氏称谓
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hereditary: Option<String>,
    /// 自定义姓名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// 本地文字的自定义姓名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_local_name: Option<String>,
    /// 中间名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// 姓
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_primary: Option<String>,
    /// 第二姓氏
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
    /// 婚后姓氏
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tertiary: Option<String>,
    /// 尊称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social: Option<EnumValue>,
    /// 头衔
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<EnumValue>,
    /// 本地中间名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_middle_name: Option<String>,
    /// 第二姓氏 - 本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_secondary: Option<String>,
    /// 展示姓名（本地和西方文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name_local_and_western_script: Option<String>,
    /// 展示姓名（本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name_local_script: Option<String>,
    /// 展示姓名（西方文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name_western_script: Option<String>,
    /// ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
}

/// 官方 i18n 多语文本结构：`{lang, value}`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 enum 类型字段结构：`{enum_name, display: i18n[]}`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
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

    /// 端到端：POST /open-apis/corehr/v2/pre_hires/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/pre_hires/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "person_info": {
                                "person_id": "6919733936050406926",
                                "phone_number": "13649211111",
                                "legal_name": "张三",
                                "preferred_name": "刘梓新(Henry)",
                                "preferred_local_full_name": "刘梓新",
                                "preferred_english_full_name": "Henry",
                                "name_list": [
                                    {
                                        "local_primary": "黄",
                                        "local_first_name": "四",
                                        "country_region_id": "6862995757234914824",
                                        "name_type": {
                                            "enum_name": "phone_type",
                                            "display": [{ "lang": "zh-CN", "value": "中文示例" }]
                                        },
                                        "local_first_name_2": "五",
                                        "local_primary_2": "王",
                                        "additional_name": "别名"
                                    }
                                ]
                            },
                            "pre_hire_id": "7023239238976141133"
                        }
                    ],
                    "page_token": "1234452132",
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

        let resp = SearchRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let item = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("items 应包含待入职信息");
        assert_eq!(item.pre_hire_id.as_deref(), Some("7023239238976141133"));
        let person = item.person_info.as_ref().expect("应包含 person_info");
        assert_eq!(person.person_id.as_deref(), Some("6919733936050406926"));
        assert_eq!(person.phone_number.as_deref(), Some("13649211111"));
        assert_eq!(person.legal_name.as_deref(), Some("张三"));
        assert_eq!(person.preferred_name.as_deref(), Some("刘梓新(Henry)"));
        assert_eq!(person.preferred_local_full_name.as_deref(), Some("刘梓新"));
        assert_eq!(person.preferred_english_full_name.as_deref(), Some("Henry"));
        let name = person
            .name_list
            .as_ref()
            .and_then(|n| n.first())
            .expect("name_list 应包含姓名");
        assert_eq!(name.local_primary.as_deref(), Some("黄"));
        assert_eq!(name.local_first_name.as_deref(), Some("四"));
        assert_eq!(
            name.country_region_id.as_deref(),
            Some("6862995757234914824")
        );
        assert_eq!(
            name.name_type.as_ref().and_then(|t| t.enum_name.as_deref()),
            Some("phone_type")
        );
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
