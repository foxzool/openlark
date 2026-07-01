//! 词条高亮
//!
//! docPath: <https://open.feishu.cn/document/server-docs/baike-v1/entity/highlight>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 词条高亮请求体。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HighlightEntityReqBody {
    /// 待高亮文本。
    pub text: String,
}

/// 词条高亮响应 data。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HighlightEntityResponse {
    /// 高亮短语列表。
    #[serde(default)]
    pub phrases: Vec<Phrase>,
}

/// 高亮短语信息。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Phrase {
    /// 词条名称。
    pub name: String,
    /// 命中的词条 ID 列表。
    pub entity_ids: Vec<String>,
    /// 文本位置区间。
    pub span: Span,
}

/// 文本高亮位置区间。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Span {
    /// 起始位置。
    pub start: i32,
    /// 结束位置。
    pub end: i32,
}

impl ApiResponseTrait for HighlightEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 词条高亮请求
pub struct HighlightEntityRequest {
    config: Config,
    req: HighlightEntityReqBody,
}

impl HighlightEntityRequest {
    /// 创建新的词条高亮请求。
    pub fn new(config: Config, text: impl Into<String>) -> Self {
        Self {
            config,
            req: HighlightEntityReqBody { text: text.into() },
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<HighlightEntityResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<HighlightEntityResponse> {
        use crate::common::api_endpoints::BaikeApiV1;
        // ===== 参数校验 =====
        validate_required!(self.req.text, "text 不能为空");
        let len = self.req.text.chars().count();
        if !(1..=1000).contains(&len) {
            return Err(openlark_core::error::validation_error(
                "text",
                "text 长度必须在 1~1000 字符之间",
            ));
        }

        // ===== 构建请求 =====
        let api_request: ApiRequest<HighlightEntityResponse> =
            ApiRequest::post(&BaikeApiV1::EntityHighlight.to_url())
                .body(serde_json::to_value(&self.req)?);

        // ===== 发送请求 =====
        let response: Response<HighlightEntityResponse> =
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
    fn test_highlight_entity_request_builder() {
        let config = Config::default();
        let request = HighlightEntityRequest::new(config, "测试文本");
        assert_eq!(request.req.text, "测试文本");
    }

    /// 测试响应数据结构
    #[test]
    fn test_highlight_entity_response() {
        let response = HighlightEntityResponse {
            phrases: vec![Phrase {
                name: "关键词1".to_string(),
                entity_ids: vec!["id1".to_string(), "id2".to_string()],
                span: Span { start: 0, end: 3 },
            }],
        };

        assert_eq!(response.phrases.len(), 1);
        assert_eq!(response.phrases[0].name, "关键词1");
    }

    /// 测试Span结构
    #[test]
    fn test_span_structure() {
        let span = Span { start: 0, end: 10 };

        assert_eq!(span.start, 0);
        assert_eq!(span.end, 10);
    }

    /// 测试响应trait实现
    #[test]
    fn test_response_trait() {
        assert_eq!(HighlightEntityResponse::data_format(), ResponseFormat::Data);
    }

    /// 测试边界长度
    #[test]
    fn test_text_length_boundaries() {
        let config = Config::default();

        // 最小长度 1
        let min_request = HighlightEntityRequest::new(config.clone(), "a");
        assert_eq!(min_request.req.text.chars().count(), 1);

        // 最大长度 1000
        let max_text = "a".repeat(1000);
        let max_request = HighlightEntityRequest::new(config, max_text);
        assert_eq!(max_request.req.text.chars().count(), 1000);

        // 测试空文本（会触发验证错误）
        let config2 = Config::default();
        let _empty_request = HighlightEntityRequest::new(config2, "");
    }

    /// 测试多个实体ID
    #[test]
    fn test_multiple_entity_ids() {
        let phrase = Phrase {
            name: "多义词".to_string(),
            entity_ids: vec!["id1".to_string(), "id2".to_string(), "id3".to_string()],
            span: Span { start: 0, end: 3 },
        };

        assert_eq!(phrase.entity_ids.len(), 3);
    }

    /// 测试空短语列表
    #[test]
    fn test_empty_phrases() {
        let response = HighlightEntityResponse { phrases: vec![] };

        assert!(response.phrases.is_empty());
    }
}
