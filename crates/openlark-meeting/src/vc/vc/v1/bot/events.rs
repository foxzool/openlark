//! 获取会议事件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/bot/events>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_endpoints::VcApiV1;

/// 获取会议事件请求。
#[derive(Debug, Clone)]
pub struct GetBotEventsRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl GetBotEventsRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数。
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(VcApiV1::BotEvents.to_url());

        for (key, value) in self.query_params {
            req = req.query(key, value);
        }

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
}
