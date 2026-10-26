//! 启停用岗位
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/position/active>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 启停用岗位请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActiveBody {
    /// 岗位 ID（必填），可通过查询岗位信息接口获得
    pub position_id: String,
    /// 可选值：true（启用）、false（停用）
    pub active: bool,
    /// 版本生效时间（必填，格式：YYYY-MM-DD）
    pub effective_time: String,
}

/// 启停用岗位请求
#[derive(Debug, Clone)]
pub struct ActiveRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: ActiveBody,
}

impl ActiveRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: ActiveBody::default(),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: ActiveBody) -> Self {
        self.body = body;
        self
    }

    /// 设置岗位 ID（必填）。
    pub fn position_id(mut self, position_id: impl Into<String>) -> Self {
        self.body.position_id = position_id.into();
        self
    }

    /// 设置启用/停用（true=启用，false=停用）。
    pub fn active(mut self, active: bool) -> Self {
        self.body.active = active;
        self
    }

    /// 设置版本生效时间（必填，格式：YYYY-MM-DD）。
    pub fn effective_time(mut self, effective_time: impl Into<String>) -> Self {
        self.body.effective_time = effective_time.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ActiveResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ActiveResponse> {
        validate_required!(self.body.position_id, "position_id 不能为空");
        validate_required!(self.body.effective_time, "effective_time 不能为空");

        let request = ApiRequest::<ActiveResponse>::post("/open-apis/corehr/v2/positions/active")
            .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 启停用岗位响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActiveResponse {
    /// 响应数据
    pub data: Value,
}

impl ApiResponseTrait for ActiveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/positions/active
    /// （官方请求体必填 position_id/active/effective_time）
    #[tokio::test]
    async fn test_active_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/positions/active"))
            .and(body_json(json!({
                "position_id": "6862995757234914823",
                "active": true,
                "effective_time": "2020-01-01"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "data": {} }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        ActiveRequest::new(config)
            .position_id("6862995757234914823")
            .active(true)
            .effective_time("2020-01-01")
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
