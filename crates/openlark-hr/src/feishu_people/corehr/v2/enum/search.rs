//! 查询枚举信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/enum/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询枚举信息请求
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: Option<Value>,
}

impl SearchRequest {
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
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        let mut request = ApiRequest::<SearchResponse>::post("/open-apis/corehr/v2/enums/search");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 查询枚举信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 查询的枚举信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enums: Option<Vec<EnumInfo>>,
}

/// 枚举信息（官方 enums[] 元素结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumInfo {
    /// 枚举名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_apiname: Option<String>,
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_items: Option<Vec<EnumField>>,
}

/// 枚举值（官方 enum_field 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumField {
    /// ApiName
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    /// 枚举值名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    /// 枚举值描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
    /// 所属枚举常量 ApiName
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_api_name: Option<String>,
    /// 顺序
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// 状态：1 生效，0 失效
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
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

    /// 端到端：POST /open-apis/corehr/v2/enums/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/enums/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "enums": [
                        {
                            "enum_apiname": "overtime_date_type",
                            "enum_items": [
                                {
                                    "api_name": "special_day",
                                    "name": [
                                        { "lang": "zh-CN", "value": "中文示例" }
                                    ],
                                    "description": [
                                        { "lang": "zh-CN", "value": "中文示例" }
                                    ],
                                    "enum_api_name": "overtime_date_type",
                                    "order": 1,
                                    "status": 1
                                }
                            ]
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

        let resp = SearchRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let enums = resp.enums.expect("enums 应存在");
        assert_eq!(enums.len(), 1);
        assert_eq!(
            enums[0].enum_apiname,
            Some("overtime_date_type".to_string())
        );
        let enum_items = enums[0].enum_items.clone().expect("enum_items 应存在");
        assert_eq!(enum_items.len(), 1);
        assert_eq!(enum_items[0].api_name, Some("special_day".to_string()));
        assert_eq!(enum_items[0].order, Some(1));
        assert_eq!(enum_items[0].status, Some(1));
        assert_eq!(
            enum_items[0].name,
            Some(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("中文示例".to_string()),
            }])
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
