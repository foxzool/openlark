//! 获取妙记AI产物
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

use crate::common::api_endpoints::minutes::MinutesExtraApiV1;

/// 获取妙记AI产物请求。
#[derive(Debug, Clone)]
pub struct MinuteArtifactsRequest {
    config: Arc<Config>,
    minute_token: String,
}

impl MinuteArtifactsRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            minute_token: String::new(),
        }
    }

    /// 设置路径参数 `minute_token`。
    pub fn minute_token(mut self, minute_token: impl Into<String>) -> Self {
        self.minute_token = minute_token.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.minute_token, "minute_token 不能为空");
        let req: ApiRequest<serde_json::Value> =
            MinutesExtraApiV1::Artifacts(self.minute_token).to_request();
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("获取妙记AI产物", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = MinuteArtifactsRequest::new(config);
    }
}
