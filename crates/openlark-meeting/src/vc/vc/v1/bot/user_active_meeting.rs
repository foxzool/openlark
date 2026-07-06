//! 获取用户活跃会议
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/bot/user_active_meeting>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::extract_response_data;

/// 获取用户活跃会议请求。
#[derive(Debug, Clone)]
pub struct GetUserActiveMeetingRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl GetUserActiveMeetingRequest {
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
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(VcApiV1::BotUserActiveMeeting.to_url());

        for (key, value) in self.query_params {
            req = req.query(key, value);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取用户活跃会议")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_active_meeting_url() {
        assert_eq!(
            VcApiV1::BotUserActiveMeeting.to_url(),
            "/open-apis/vc/v1/bots/user_active_meeting"
        );
    }
}
