//! 提取潜在的词条
//!
//! docPath: <https://open.feishu.cn/document/server-docs/baike-v1/entity/extract>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 潜在词条提取请求体。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtractEntityReqBody {
    /// 待提取词条的原始文本。
    pub text: String,
}

/// 潜在词条提取响应 data。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtractEntityResponse {
    /// 提取到的候选词条列表。
    #[serde(default)]
    pub entity_word: Vec<ExtractedWord>,
}

/// 提取出的词条候选项。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtractedWord {
    /// 词条名称。
    pub name: String,
    /// 别名列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
}

impl ApiResponseTrait for ExtractEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 提取潜在词条请求
pub struct ExtractEntityRequest {
    config: Config,
    req: ExtractEntityReqBody,
}

impl ExtractEntityRequest {
    /// 创建新的候选词条提取请求。
    pub fn new(config: Config, text: impl Into<String>) -> Self {
        Self {
            config,
            req: ExtractEntityReqBody { text: text.into() },
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ExtractEntityResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ExtractEntityResponse> {
        use crate::common::api_endpoints::BaikeApiV1;
        // ===== 参数校验 =====
        let len = self.req.text.chars().count();
        if len > 128 {
            return Err(openlark_core::error::validation_error(
                "text",
                "text 最大长度不能超过 128 字符",
            ));
        }

        // ===== 构建请求 =====
        // 使用 catalog 提供 method + path + auth（#443）
        let api_request: ApiRequest<ExtractEntityResponse> =
            BaikeApiV1::EntityExtract.to_request()
                .body(serde_json::to_value(&self.req)?);

        // ===== 发送请求 =====
        let response: Response<ExtractEntityResponse> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_extract_entity_request_builder() {
        let config = Config::default();
        let request = ExtractEntityRequest::new(config, "测试文本");
        assert_eq!(request.req.text, "测试文本");
    }

    /// 测试空文本
    #[test]
    fn test_empty_text() {
        let config = Config::default();
        let request = ExtractEntityRequest::new(config, "");
        assert_eq!(request.req.text, "");
    }

    /// 测试 Unicode 字符计数
    #[test]
    fn test_unicode_character_count() {
        let config = Config::default();
        let text = "🎉🎊🎈"; // 3 个 Unicode 码点
        let request = ExtractEntityRequest::new(config, text);
        assert_eq!(request.req.text.chars().count(), 3);
    }

    /// 测试响应数据结构
    #[test]
    fn test_extract_entity_response() {
        let response = ExtractEntityResponse {
            entity_word: vec![
                ExtractedWord {
                    name: "词条1".to_string(),
                    aliases: Some(vec!["别名1".to_string()]),
                },
                ExtractedWord {
                    name: "词条2".to_string(),
                    aliases: None,
                },
            ],
        };

        assert_eq!(response.entity_word.len(), 2);
        assert_eq!(response.entity_word[0].name, "词条1");
    }

    /// 测试响应trait实现
    #[test]
    fn test_response_trait() {
        assert_eq!(ExtractEntityResponse::data_format(), ResponseFormat::Data);
    }

    /// 测试ExtractedWord结构
    #[test]
    fn test_extracted_word_structure() {
        let word = ExtractedWord {
            name: "测试词条".to_string(),
            aliases: Some(vec!["别名A".to_string(), "别名B".to_string()]),
        };

        assert_eq!(word.name, "测试词条");
        assert_eq!(word.aliases.unwrap().len(), 2);
    }

    /// 测试无别名场景
    #[test]
    fn test_extracted_word_no_aliases() {
        let word = ExtractedWord {
            name: "无别名词条".to_string(),
            aliases: None,
        };

        assert!(word.aliases.is_none());
    }

    /// 端到端：POST .../baike/v1/entities/extract → 强类型 ExtractEntityResponse（单层 data 信封）。
    #[tokio::test]
    async fn test_extract_entity_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/baike/v1/entities/extract"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "entity_word": [
                        { "name": "词条1", "aliases": ["别名1"] },
                        { "name": "词条2" }
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

        let resp = ExtractEntityRequest::new(config, "提取这段文本的词条")
            .execute()
            .await
            .expect("提取词条应成功");
        assert_eq!(resp.entity_word.len(), 2);
        assert_eq!(resp.entity_word[0].name, "词条1");
        assert_eq!(resp.entity_word[0].aliases.as_ref().unwrap().len(), 1);
        assert_eq!(resp.entity_word[1].aliases, None);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/baike/v1/entities/extract"
        );
    }
}
