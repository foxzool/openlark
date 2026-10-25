//! 查询待入职信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/pre_hire/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `QueryRequest` 请求。
#[derive(Debug, Clone)]
pub struct QueryRequest {
    config: Config,
    request_body: Option<Value>,
}

impl QueryRequest {
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
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let api_endpoint = FeishuPeopleApiV2::PreHireQuery;
        let mut request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询待入职信息响应数据为空",
        )
        .await
    }
}

/// `QueryResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 查询待入职的信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PreHireItem>>,
    /// 分页标记
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 待入职信息（官方 pre_hire 结构；字段受请求 fields 参数控制按需返回，
/// 未 typed 的子对象以 Value 透传保留）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreHireItem {
    /// 待入职 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_hire_id: Option<String>,
    /// 个人信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_info: Option<PersonInfo>,
    /// 工作信息（官方 pre_hire_employment_info 结构，字段量大，暂以 Value 透传）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_info: Option<Value>,
    /// 入职信息（官方 pre_hire_onboarding_info 结构，暂以 Value 透传）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_info: Option<Value>,
    /// 试用期信息（官方 pre_hire_probation_info 结构，暂以 Value 透传）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_info: Option<Value>,
    /// 合同信息（官方 pre_hire_contract_info 结构，暂以 Value 透传）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_info: Option<Value>,
}

/// 个人信息（官方 person_info 结构的核心字段；其余字段经 extra 透传保留）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersonInfo {
    /// 个人信息 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    /// 个人电话
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// 法定姓名
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
    /// 姓名列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_list: Option<Vec<PersonName>>,
    /// 其余官方 person_info 字段透传保留
    #[serde(flatten)]
    pub extra: Value,
}

/// 姓名（官方 person_name 结构的核心字段；其余字段经 extra 透传保留）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersonName {
    /// 姓 - 本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_primary: Option<String>,
    /// 名 - 本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_first_name: Option<String>,
    /// 国家 / 地区
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    /// 姓名类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_type: Option<EnumValue>,
    /// 其余官方 person_name 字段透传保留
    #[serde(flatten)]
    pub extra: Value,
}

/// 官方 enum 类型字段结构：{enum_name, display: i18n[]}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
}

/// 多语文本（官方 i18n 结构：{lang, value}）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
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

    /// 端到端：POST /open-apis/corehr/v2/pre_hires/query
    #[tokio::test]
    async fn test_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/pre_hires/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "pre_hire_id": "7032210902531327521",
                            "person_info": {
                                "person_id": "6863326263210149383",
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
                                            "display": [
                                                { "lang": "zh-CN", "value": "中文示例" }
                                            ]
                                        }
                                    }
                                ]
                            },
                            "employment_info": { "department_id": "7094136522860922111" }
                        }
                    ],
                    "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
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

        let items = resp.items.expect("items 应解码");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].pre_hire_id.as_deref(), Some("7032210902531327521"));
        let person_info = items[0].person_info.as_ref().expect("person_info 应解码");
        assert_eq!(person_info.phone_number.as_deref(), Some("13649211111"));
        assert_eq!(person_info.legal_name.as_deref(), Some("张三"));
        assert_eq!(
            person_info.preferred_english_full_name.as_deref(),
            Some("Henry")
        );
        let name_list = person_info.name_list.as_ref().expect("name_list 应解码");
        assert_eq!(name_list[0].local_primary.as_deref(), Some("黄"));
        assert_eq!(name_list[0].local_first_name.as_deref(), Some("四"));
        assert_eq!(
            name_list[0].country_region_id.as_deref(),
            Some("6862995757234914824")
        );
        assert_eq!(
            name_list[0]
                .name_type
                .as_ref()
                .and_then(|t| t.enum_name.as_deref()),
            Some("phone_type")
        );
        assert_eq!(
            items[0]
                .employment_info
                .as_ref()
                .and_then(|v| v.get("department_id"))
                .and_then(|v| v.as_str()),
            Some("7094136522860922111")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
