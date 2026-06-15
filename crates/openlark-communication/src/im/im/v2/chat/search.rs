//! 搜索群组
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/group/im-v2/chat/search

use crate::endpoints::IM_V2_CHATS_SEARCH;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 搜索群组请求。
#[derive(Debug, Clone)]
pub struct SearchChatsRequest {
    config: Arc<Config>,
    /// 查询参数
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
    /// 请求体
    body: SearchChatsRequestBody,
}

/// 搜索群组请求体。
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct SearchChatsRequestBody {
    /// 搜索关键词（0-50 字符）。
    pub query: Option<String>,
    /// 过滤条件。
    pub filter: Option<ChatSearchFilter>,
    /// 排序方式：create_time_desc / update_time_desc / member_count_desc。
    pub sorter: Option<String>,
}

/// 群组搜索过滤条件。
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct ChatSearchFilter {
    /// 群组类型：private / external / public_joined / public_not_joined（0-4）。
    pub search_types: Option<Vec<String>>,
    /// 群成员ID（0-500）。
    pub member_ids: Option<Vec<String>>,
    /// 是否自己创建或者管理的群组。
    pub is_manager: Option<bool>,
    /// 是否关闭以人搜群功能（默认开启）。
    pub disable_search_by_user: Option<bool>,
    /// 群模式筛选器：default（普通群）/ thread（话题群）（0-2）。
    pub chat_modes: Option<Vec<String>>,
}

/// 搜索群组响应。
#[derive(Debug, Clone, Deserialize)]
pub struct SearchChatsResponse {
    /// 错误码，非 0 表示失败。
    pub code: i32,
    /// 错误描述。
    pub msg: String,
    /// 响应数据。
    pub data: Option<SearchChatsData>,
}

impl ApiResponseTrait for SearchChatsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索群组响应数据。
#[derive(Debug, Clone, Deserialize)]
pub struct SearchChatsData {
    /// 搜索结果列表。
    pub items: Option<Vec<ChatSearchItem>>,
    /// 搜索命中结果数。
    pub total: Option<i32>,
    /// 是否还有更多项。
    pub has_more: Option<bool>,
    /// 分页标记。
    pub page_token: Option<String>,
    /// 搜索补充提示信息。
    pub notice: Option<String>,
}

/// 群组搜索结果项。
#[derive(Debug, Clone, Deserialize)]
pub struct ChatSearchItem {
    /// 群组ID。
    pub id: Option<String>,
    /// 包含群组基本信息的卡片，关键词命中的文本片段用 <h></h> 包裹标注。
    pub display_info: Option<String>,
    /// 群组元信息。
    pub meta_data: Option<ChatSearchMeta>,
}

/// 群组元信息。
#[derive(Debug, Clone, Deserialize)]
pub struct ChatSearchMeta {
    /// 群组 ID。
    pub chat_id: Option<String>,
    /// 创建时间（iso8601）。
    pub create_time: Option<String>,
    /// 更新时间（iso8601）。
    pub update_time: Option<String>,
    /// 是否是外部群。
    pub external: Option<bool>,
    /// 群模式：group / topic。
    pub chat_mode: Option<String>,
    /// 群描述。
    pub description: Option<String>,
    /// 群头像URL。
    pub avatar: Option<String>,
    /// 群名称。
    pub name: Option<String>,
    /// 群主ID。
    pub owner_id: Option<String>,
    /// 群主ID类型。
    pub owner_id_type: Option<String>,
    /// tenant key。
    pub tenant_key: Option<String>,
    /// 群状态：normal / dissolved / dissolved_save。
    pub chat_status: Option<String>,
}

impl SearchChatsRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
            body: SearchChatsRequestBody::default(),
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

    /// 设置用户 ID 类型（查询参数）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置搜索关键词。
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.body.query = Some(query.into());
        self
    }

    /// 设置过滤条件。
    pub fn filter(mut self, filter: ChatSearchFilter) -> Self {
        self.body.filter = Some(filter);
        self
    }

    /// 设置排序方式。
    pub fn sorter(mut self, sorter: impl Into<String>) -> Self {
        self.body.sorter = Some(sorter.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<SearchChatsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SearchChatsResponse> {
        let mut req: ApiRequest<SearchChatsResponse> = ApiRequest::post(IM_V2_CHATS_SEARCH);

        if let Some(ps) = self.page_size {
            req = req.query_param("page_size", ps.to_string());
        }
        if let Some(pt) = &self.page_token {
            req = req.query_param("page_token", pt);
        }
        if let Some(uit) = &self.user_id_type {
            req = req.query_param("user_id_type", uit);
        }

        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error("搜索群组", format!("请求体序列化失败: {e}"))
        })?;
        req = req.body(body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("搜索群组", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_sets_params() {
        let config = Arc::new(Config::default());
        let request = SearchChatsRequest::new(config)
            .query("部门群")
            .page_size(10);
        assert_eq!(request.body.query, Some("部门群".to_string()));
        assert_eq!(request.page_size, Some(10));
    }

    #[test]
    fn response_deserializes_from_json() {
        let json = r#"{
            "code": 0,
            "msg": "success",
            "data": {
                "items": [{
                    "id": "7890123456abcdef",
                    "display_info": "飞书<h>搜索</h>",
                    "meta_data": {
                        "chat_id": "7890123456abcdef",
                        "external": true,
                        "chat_mode": "group",
                        "name": "研发讨论群",
                        "chat_status": "normal"
                    }
                }],
                "total": 10,
                "has_more": true
            }
        }"#;
        let resp: SearchChatsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.code, 0);
        assert_eq!(resp.data.as_ref().unwrap().total, Some(10));
        let item = resp.data.unwrap().items.unwrap().pop().unwrap();
        assert_eq!(item.meta_data.unwrap().name, Some("研发讨论群".to_string()));
    }
}
