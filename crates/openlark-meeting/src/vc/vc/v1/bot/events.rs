//! 获取会议事件列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/bot/events>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;

/// 获取会议事件列表查询参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBotEventsQuery {
    /// 会议唯一标识，可通过创建会议接口或会议列表查询接口获取（必填）
    pub meeting_id: String,
    /// 分页标记，第一次请求不填，表示从头开始遍历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 待查询历史信息的起始时间，秒级 Unix 时间戳字符串（10 位）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 待查询历史信息的结束时间，秒级 Unix 时间戳字符串（10 位）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 分页大小，取值范围 20～100，默认 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 用户 ID 类型：open_id / union_id / user_id，默认 open_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

/// 获取会议事件列表请求。
#[derive(Debug, Clone)]
pub struct GetBotEventsRequest {
    config: Config,
    query: GetBotEventsQuery,
}

impl GetBotEventsRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: GetBotEventsQuery::default(),
        }
    }

    /// 设置会议唯一标识（必填）。
    pub fn meeting_id(mut self, meeting_id: impl Into<String>) -> Self {
        self.query.meeting_id = meeting_id.into();
        self
    }

    /// 设置分页标记，第一次请求不填，表示从头开始遍历。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.query.page_token = Some(page_token.into());
        self
    }

    /// 设置待查询历史信息的起始时间（秒级 Unix 时间戳字符串，10 位）。
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.query.start_time = Some(start_time.into());
        self
    }

    /// 设置待查询历史信息的结束时间（秒级 Unix 时间戳字符串，10 位）。
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.query.end_time = Some(end_time.into());
        self
    }

    /// 设置分页大小（取值范围 20～100，默认 20）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.query.page_size = Some(page_size);
        self
    }

    /// 设置用户 ID 类型：`open_id` / `union_id` / `user_id`。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.query.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.query.meeting_id, "meeting_id 不能为空");

        let req: ApiRequest<serde_json::Value> = ApiRequest::get(VcApiV1::BotEvents.to_url())
            .query("meeting_id", self.query.meeting_id)
            .query_opt("page_token", self.query.page_token)
            .query_opt("start_time", self.query.start_time)
            .query_opt("end_time", self.query.end_time)
            .query_opt("page_size", self.query.page_size.map(|v| v.to_string()))
            .query_opt("user_id_type", self.query.user_id_type);

        Transport::request_typed(req, &self.config, Some(option), "获取会议事件").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_events_url() {
        assert_eq!(VcApiV1::BotEvents.to_url(), "/open-apis/vc/v1/bots/events");
    }

    #[test]
    fn test_query_serializes_meeting_id_and_skips_optional() {
        let query = GetBotEventsQuery {
            meeting_id: "7628568141510692381".to_string(),
            page_token: None,
            start_time: None,
            end_time: None,
            page_size: None,
            user_id_type: None,
        };
        let value = serde_json::to_value(&query).expect("序列化查询参数应成功");
        assert_eq!(
            value,
            serde_json::json!({"meeting_id": "7628568141510692381"})
        );
    }
}
