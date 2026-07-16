//! 精准搜索词条
//!
//! docPath: <https://open.feishu.cn/document/server-docs/baike-v1/entity/match>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::BaikeApiV1;

/// 精准匹配请求体。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchEntityReq {
    /// 搜索关键词，将与词条名、别名进行精准匹配
    pub word: String,
}

/// 精准搜索词条响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchEntityResp {
    /// 匹配结果列表。
    #[serde(default)]
    pub results: Vec<MatchEntityResult>,
}

/// 精准匹配结果项。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchEntityResult {
    /// 词条 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// 匹配类型（文档示例为 int，如 0）
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,
}

impl ApiResponseTrait for MatchEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 精准搜索词条请求
pub struct MatchEntityRequest {
    config: Config,
    req: MatchEntityReq,
}

impl MatchEntityRequest {
    /// 创建新的词条精准匹配请求。
    pub fn new(config: Config, word: impl Into<String>) -> Self {
        Self {
            config,
            req: MatchEntityReq { word: word.into() },
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<MatchEntityResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<MatchEntityResp> {
        // ===== 验证必填字段 =====
        validate_required!(self.req.word, "word 不能为空");
        // ===== 验证字段长度 =====
        let len = self.req.word.chars().count();
        if !(1..=100).contains(&len) {
            return Err(openlark_core::error::validation_error(
                "word",
                "word 长度必须在 1~100 字符之间",
            ));
        }

        // 使用 catalog 提供 method + path + auth（#443）
        let api_request: ApiRequest<MatchEntityResp> =
            BaikeApiV1::EntityMatch.to_request()
                .body(serde_json::to_value(&self.req)?);

        let response: Response<MatchEntityResp> =
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
    fn test_match_entity_request_builder() {
        let config = Config::default();
        let request = MatchEntityRequest::new(config, "搜索词");
        assert_eq!(request.req.word, "搜索词");
    }

    /// 测试 Unicode 字符计数
    #[test]
    fn test_unicode_character_count() {
        let config = Config::default();
        let word = "🎉🎊🎈"; // 3 个 Unicode 码点
        let request = MatchEntityRequest::new(config, word);
        assert_eq!(request.req.word.chars().count(), 3);
    }

    /// 端到端：POST .../baike/v1/entities/match → 强类型 MatchEntityResp（单层 data 信封）。
    #[tokio::test]
    async fn test_match_entity_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/baike/v1/entities/match"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "results": [
                        { "entity_id": "en001", "type": 0 }
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

        let resp = MatchEntityRequest::new(config, "飞书")
            .execute()
            .await
            .expect("精准搜索词条应成功");
        assert_eq!(resp.results.len(), 1);
        assert_eq!(resp.results[0].entity_id, Some("en001".to_string()));
        assert_eq!(resp.results[0].type_, Some(0));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/baike/v1/entities/match");
    }
}
