//! 搜索机器人
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bot-v4/bot/search

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 搜索机器人请求。
#[derive(Debug, Clone)]
pub struct SearchBotRequest {
    config: Arc<Config>,
    /// 查询参数
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
    /// 请求体
    body: SearchBotRequestBody,
}

/// 搜索机器人请求体。
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct SearchBotRequestBody {
    /// 搜索关键词（0-50 字符）。
    pub query: Option<String>,
    /// 过滤条件。
    pub filter: Option<BotSearchFilter>,
}

/// 机器人搜索过滤条件。
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct BotSearchFilter {
    /// 群聊ID列表，查询指定群聊内的机器人（0-100）。
    pub chat_ids: Option<Vec<String>>,
    /// 是否和机器人聊过天，true 只返回有聊天的机器人。
    pub has_chatter: Option<bool>,
}

/// 搜索机器人响应。
#[derive(Debug, Clone, Deserialize)]
pub struct SearchBotResponse {
    /// 错误码，非 0 表示失败。
    pub code: i32,
    /// 错误描述。
    pub msg: String,
    /// 响应数据。
    pub data: Option<SearchBotData>,
}

impl ApiResponseTrait for SearchBotResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索机器人响应数据。
#[derive(Debug, Clone, Deserialize)]
pub struct SearchBotData {
    /// 搜索结果列表。
    pub items: Option<Vec<BotSearchItem>>,
    /// 是否还有更多项。
    pub has_more: Option<bool>,
    /// 分页标记。
    pub page_token: Option<String>,
    /// 搜索补充提示信息。
    pub notice: Option<String>,
}

/// 机器人搜索结果项。
#[derive(Debug, Clone, Deserialize)]
pub struct BotSearchItem {
    /// 机器人ID。
    pub id: Option<String>,
    /// 包含机器人基本信息的卡片，关键词命中的文本片段用 <h></h> 包裹标注。
    pub display_info: Option<String>,
    /// 机器人元信息。
    pub meta_data: Option<BotSearchMeta>,
}

/// 机器人元信息。
#[derive(Debug, Clone, Deserialize)]
pub struct BotSearchMeta {
    /// 租户ID。
    pub tenant_id: Option<String>,
    /// 是否允许加入群聊。
    pub enable_join_group: Option<bool>,
    /// 机器人所属的群聊ID。
    pub chat_id: Option<String>,
    /// 是否是智能体。
    pub is_agent: Option<bool>,
}

impl SearchBotRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
            body: SearchBotRequestBody::default(),
        }
    }

    /// 设置分页大小（查询参数）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（查询参数）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置用户 ID 类型（查询参数）：open_id / union_id / user_id。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置搜索关键词（请求体，0-50 字符）。
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.body.query = Some(query.into());
        self
    }

    /// 设置过滤条件（请求体）。
    pub fn filter(mut self, filter: BotSearchFilter) -> Self {
        self.body.filter = Some(filter);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<SearchBotResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SearchBotResponse> {
        let mut req: ApiRequest<SearchBotResponse> = ApiRequest::post("/open-apis/bot/v4/bot/search");

        // 查询参数
        if let Some(ps) = self.page_size {
            req = req.query_param("page_size", ps.to_string());
        }
        if let Some(pt) = &self.page_token {
            req = req.query_param("page_token", pt);
        }
        if let Some(uit) = &self.user_id_type {
            req = req.query_param("user_id_type", uit);
        }

        // 请求体
        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error(
                "搜索机器人",
                format!("请求体序列化失败: {e}"),
            )
        })?;
        req = req.body(body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("搜索机器人", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let request = SearchBotRequest::new(config);
        assert!(request.body.query.is_none());
    }

    #[test]
    fn builder_sets_params() {
        let config = Arc::new(Config::default());
        let request = SearchBotRequest::new(config)
            .query("会议助手")
            .page_size(10)
            .user_id_type("open_id");
        assert_eq!(request.body.query, Some("会议助手".to_string()));
        assert_eq!(request.page_size, Some(10));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn body_serializes_correctly() {
        let body = SearchBotRequestBody {
            query: Some("会议助手".to_string()),
            filter: Some(BotSearchFilter {
                chat_ids: Some(vec!["oc-123".to_string()]),
                has_chatter: Some(false),
            }),
        };
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(json["query"], "会议助手");
        assert_eq!(json["filter"]["chat_ids"][0], "oc-123");
    }

    #[test]
    fn response_deserializes_from_json() {
        let json = r#"{
            "code": 0,
            "msg": "success",
            "data": {
                "items": [{
                    "id": "7890123456abcdef",
                    "display_info": "飞书<h>搜索</h>助手",
                    "meta_data": {
                        "tenant_id": "7010970696222244883",
                        "enable_join_group": false,
                        "chat_id": "oc-7890123456abcdef",
                        "is_agent": false
                    }
                }],
                "has_more": true,
                "page_token": "token123"
            }
        }"#;
        let resp: SearchBotResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.code, 0);
        let item = resp.data.unwrap().items.unwrap().pop().unwrap();
        assert_eq!(item.id, Some("7890123456abcdef".to_string()));
        assert_eq!(item.meta_data.unwrap().is_agent, Some(false));
    }
}
