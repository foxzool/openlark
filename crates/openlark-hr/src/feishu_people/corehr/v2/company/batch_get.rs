//! 通过公司 ID 批量获取公司信息
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/company/batch_get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 通过公司 ID 批量获取公司信息请求
#[derive(Debug, Clone)]
pub struct BatchGetRequest {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl BatchGetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体。
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
            ApiRequest::<BatchGetResponse>::post("/open-apis/corehr/v2/companies/batch_get");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 通过公司 ID 批量获取公司信息响应
///
/// 响应 `data` 子树形状：`{items: company[]}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// 查询的公司信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CompanyItem>>,
}

/// 公司信息（官方 `company` 结构，除 `company_id` / `hiberarchy_common` 外
/// 字段经 `extra` 透传）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompanyItem {
    /// 公司 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// 公司基本信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hiberarchy_common: Option<HiberarchyCommon>,
    /// 兼容保留字段
    #[serde(flatten)]
    pub extra: Value,
}

/// 公司基本信息（官方 `hiberarchy_common` 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HiberarchyCommon {
    /// 上级公司 ID（一级公司不返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 公司名称（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    /// 公司性质类型
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<EnumValue>,
    /// 是否启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 当前版本生效日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 当前版本失效日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 描述（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
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

    /// 端到端：POST /open-apis/corehr/v2/companies/batch_get
    #[tokio::test]
    async fn test_batch_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/companies/batch_get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "company_id": "4692472714243080020",
                            "hiberarchy_common": {
                                "parent_id": "4719168654814483759",
                                "name": [{ "lang": "zh-CN", "value": "中文示例" }],
                                "type": {
                                    "enum_name": "phone_type",
                                    "display": [{ "lang": "zh-CN", "value": "中文示例" }]
                                },
                                "active": true,
                                "effective_time": "2020-05-01 00:00:00",
                                "expiration_time": "2020-05-02 00:00:00",
                                "code": "12456",
                                "description": [{ "lang": "zh-CN", "value": "中文示例" }]
                            }
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
            .expect("items 应包含公司");
        assert_eq!(item.company_id.as_deref(), Some("4692472714243080020"));
        let common = item
            .hiberarchy_common
            .as_ref()
            .expect("应包含 hiberarchy_common");
        assert_eq!(
            common
                .name
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("中文示例")
        );
        assert_eq!(
            common.r#type.as_ref().and_then(|t| t.enum_name.as_deref()),
            Some("phone_type")
        );
        assert_eq!(common.active, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
