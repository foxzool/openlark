//! 获取电子签文件列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/signature_file/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取电子签文件列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl ListRequest {
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
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        let mut request = ApiRequest::<ListResponse>::get("/open-apis/corehr/v2/signature_files");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 获取电子签文件列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 电子签文件信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<SignatureFileItem>>,
    /// 分页标记
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 电子签文件（官方 signature_file 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureFileItem {
    /// 电子签文件 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_file_id: Option<String>,
    /// 名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<I18nText>>,
    /// 创建时间，格式：YYYY-MM-DD 00:00:00
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间，格式：YYYY-MM-DD 00:00:00
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 雇员 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 电子签文件状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_file_state: Option<EnumValue>,
    /// 供应商侧的合同编号，作为幂等 key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_code: Option<String>,
    /// 电子签文件生效日期，格式：YYYY-MM-DD
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 电子签模板 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

/// 多语文本（官方 i18n 结构：{lang, value}）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
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

impl ApiResponseTrait for ListResponse {
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

    /// 端到端：GET /open-apis/corehr/v2/signature_files
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/signature_files"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "signature_file_id": "7147527056140813828",
                            "names": [
                                { "lang": "zh-CN", "value": "张三" }
                            ],
                            "create_time": "2023-01-01 00:00:00",
                            "update_time": "2023-01-01 00:00:00",
                            "employment_id": "ou_a294793e8fa21529f2a60e3e9de45520",
                            "signature_file_state": {
                                "enum_name": "sign_finished",
                                "display": [
                                    { "lang": "zh-CN", "value": "张三" }
                                ]
                            },
                            "contract_code": "48793beea04f4e2583d50732e481d507",
                            "effective_date": "2024-12-01",
                            "template_id": "7147527056140813828"
                        }
                    ],
                    "page_token": "{\"cn\":\"[\\\"7371387495822411308\\\"]\"}",
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

        let resp = ListRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应解码");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].signature_file_id.as_deref(),
            Some("7147527056140813828")
        );
        assert_eq!(
            items[0]
                .names
                .as_ref()
                .and_then(|names| names.first())
                .and_then(|n| n.value.as_deref()),
            Some("张三")
        );
        assert_eq!(
            items[0]
                .signature_file_state
                .as_ref()
                .and_then(|s| s.enum_name.as_deref()),
            Some("sign_finished")
        );
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
