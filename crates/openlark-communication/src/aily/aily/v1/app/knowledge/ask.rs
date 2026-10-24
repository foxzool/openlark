//! 执行数据知识问答
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/data-knowledge/ask>

use crate::endpoints::AILY_V1_KNOWLEDGE_ASK;
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};

/// 数据知识问答输入消息（官方 aily_knowledge_message）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AilyKnowledgeMessage {
    /// 消息内容，当前仅支持纯文本输入，长度范围 1~65535 字符。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl AilyKnowledgeMessage {
    /// 创建新的输入消息。
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
        }
    }
}

/// 执行数据知识问答请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AskKnowledgeBody {
    /// 输入消息（必填，当前仅支持纯文本输入）。
    pub message: AilyKnowledgeMessage,
    /// 控制知识问答所依据的数据知识范围，默认值为空，此时不限制数据知识范围。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_asset_ids: Option<Vec<String>>,
    /// 控制知识问答所依据的数据知识分类范围，默认值为空，此时不限制数据知识分类范围。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_asset_tag_ids: Option<Vec<String>>,
}

impl AskKnowledgeBody {
    /// 创建新的请求构建器。
    pub fn new(message: AilyKnowledgeMessage) -> Self {
        Self {
            message,
            data_asset_ids: None,
            data_asset_tag_ids: None,
        }
    }

    /// 设置数据知识范围。
    pub fn data_asset_ids(mut self, data_asset_ids: Vec<String>) -> Self {
        self.data_asset_ids = Some(data_asset_ids);
        self
    }

    /// 设置数据知识分类范围。
    pub fn data_asset_tag_ids(mut self, data_asset_tag_ids: Vec<String>) -> Self {
        self.data_asset_tag_ids = Some(data_asset_tag_ids);
        self
    }
}

/// 执行数据知识问答请求
///
/// 用于对数据知识执行问答操作。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `app_id`: 应用 ID，必填
///
/// # 请求体字段
///
/// - `message`: 输入消息，必填（含 `content` 消息内容）
/// - `data_asset_ids`: 数据知识范围（可选）
/// - `data_asset_tag_ids`: 数据知识分类范围（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let body = AskKnowledgeBody::new(AilyKnowledgeMessage::new("推荐一部电影"))
///     .data_asset_ids(vec!["asset_xxx".to_string()]);
/// let request = AskKnowledgeRequest::new(config)
///     .app_id("app_xxx")
///     .execute(body).await?;
/// ```
pub struct AskKnowledgeRequest {
    config: Config,
    app_id: String,
}

impl AskKnowledgeRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_id: String::new(),
        }
    }

    /// 应用 ID（路径参数）
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/aily-v1/data-knowledge/ask>
    pub async fn execute(self, body: AskKnowledgeBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: AskKnowledgeBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.app_id, "app_id 不能为空");

        let url = AILY_V1_KNOWLEDGE_ASK.replace("{app_id}", &self.app_id);
        let req: ApiRequest<AskKnowledgeBody> = ApiRequest::post(&url).json_body(&body);

        Transport::request_typed(req, &self.config, Some(option), "执行数据知识问答").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_ask_knowledge_request_builder() {
        let config = Config::default();
        let request = AskKnowledgeRequest::new(config).app_id("app_xxx");
        assert_eq!(request.app_id, "app_xxx");
    }

    #[test]
    fn test_ask_knowledge_body_builder() {
        let body = AskKnowledgeBody::new(AilyKnowledgeMessage::new("推荐一部电影"));
        assert_eq!(body.message.content, Some("推荐一部电影".to_string()));
        assert_eq!(body.data_asset_ids, None);
        assert_eq!(body.data_asset_tag_ids, None);
    }

    #[test]
    fn test_ask_knowledge_body_with_asset_scope() {
        let body = AskKnowledgeBody::new(AilyKnowledgeMessage::new("测试问题"))
            .data_asset_ids(vec!["asset_aadg2b5os5wjg".to_string()])
            .data_asset_tag_ids(vec![
                "spring_5862e4fea8__c__asset_tag_aadg2b5ql4gbs".to_string(),
            ]);
        assert_eq!(body.data_asset_ids.as_ref().map(|v| v.len()), Some(1));
        assert_eq!(body.data_asset_tag_ids.as_ref().map(|v| v.len()), Some(1));
    }

    #[test]
    fn test_ask_knowledge_request_default_values() {
        let config = Config::default();
        let request = AskKnowledgeRequest::new(config);
        assert_eq!(request.app_id, "");
    }

    #[test]
    fn test_ask_knowledge_body_serialization() {
        // 官方请求示例形状：{"message":{"content":"..."},"data_asset_ids":[...]}
        let body = AskKnowledgeBody::new(AilyKnowledgeMessage::new("推荐一部电影"))
            .data_asset_ids(vec!["asset_aadg2b5os5wjg".to_string()]);
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(json["message"]["content"], "推荐一部电影");
        assert_eq!(json["data_asset_ids"][0], "asset_aadg2b5os5wjg");
        assert!(json.get("data_asset_tag_ids").is_none());
        assert!(json.get("question").is_none());
        assert!(json.get("stream").is_none());
    }

    #[test]
    fn test_ask_knowledge_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_KNOWLEDGE_ASK;
        let url = AILY_V1_KNOWLEDGE_ASK.replace("{app_id}", "app_1");
        assert_eq!(url, "/open-apis/aily/v1/apps/app_1/knowledges/ask");
        assert!(
            !url.contains("{"),
            "URL should not contain unreplaced placeholders"
        );
    }
}
