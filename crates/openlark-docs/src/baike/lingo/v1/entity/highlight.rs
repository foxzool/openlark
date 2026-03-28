//! 词条高亮
//!
//! docPath: <https://open.feishu.cn/document/lingo-v1/entity/highlight>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::LingoApiV1;

/// 词条所在位置（span）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    /// 关键词开始位置，从 0 开始计数（utf-8）
    pub start: i32,
    /// 关键词结束位置，从 0 开始计数（utf-8）
    pub end: i32,
}

/// 识别到的词条信息（phrase）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phrase {
    /// 识别到的关键词
    pub name: String,
    /// 对应的词条 ID
    pub entity_ids: Vec<String>,
    /// 词条所在位置
    pub span: Span,
}

/// 词条高亮响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightEntityResp {
    /// 识别到的词条信息
    #[serde(default)]
    pub phrases: Vec<Phrase>,
}

impl ApiResponseTrait for HighlightEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 词条高亮请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightEntityBody {
    /// 需要识别词条的内容（不超过 1000 字）
    pub text: String,
}

/// 词条高亮请求
pub struct HighlightEntityRequest {
    config: Config,
    body: HighlightEntityBody,
}

impl HighlightEntityRequest {
    pub fn new(config: Config, text: impl Into<String>) -> Self {
        Self {
            config,
            body: HighlightEntityBody { text: text.into() },
        }
    }

    pub async fn execute(self) -> SDKResult<HighlightEntityResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<HighlightEntityResp> {
        // ===== 参数校验 =====
        validate_required!(self.body.text, "text 不能为空");

        // ===== 构建请求 =====
        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::serialization_error("序列化词条高亮请求体失败", Some(e))
        })?;

        let api_request: ApiRequest<HighlightEntityResp> =
            ApiRequest::post(&LingoApiV1::EntityHighlight.to_url()).body(body);

        // ===== 发送请求 =====
        let response: Response<HighlightEntityResp> =
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
    fn test_highlight_entity_builder() {
        let config = Config::default();
        let request = HighlightEntityRequest::new(config, "测试文本");

        assert_eq!(request.body.text, "测试文本");
    }

    /// 测试响应数据结构
    #[test]
    fn test_highlight_entity_response() {
        let response = HighlightEntityResp {
            phrases: vec![Phrase {
                name: "词条1".to_string(),
                entity_ids: vec!["id1".to_string()],
                span: Span { start: 0, end: 2 },
            }],
        };

        assert_eq!(response.phrases.len(), 1);
        assert_eq!(response.phrases[0].name, "词条1");
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
        assert_eq!(HighlightEntityResp::data_format(), ResponseFormat::Data);
    }

    /// 测试多个词条ID
    #[test]
    fn test_multiple_entity_ids() {
        let phrase = Phrase {
            name: "多义词".to_string(),
            entity_ids: vec!["id1".to_string(), "id2".to_string()],
            span: Span { start: 0, end: 3 },
        };

        assert_eq!(phrase.entity_ids.len(), 2);
    }

    /// 测试空短语列表
    #[test]
    fn test_empty_phrases() {
        let response = HighlightEntityResp { phrases: vec![] };

        assert!(response.phrases.is_empty());
    }

    /// 测试长文本
    #[test]
    fn test_long_text() {
        let config = Config::default();
        let long_text = "这是一个很长的文本，".repeat(20);
        let request = HighlightEntityRequest::new(config, &long_text);

        assert!(request.body.text.len() > 100);
    }
}
