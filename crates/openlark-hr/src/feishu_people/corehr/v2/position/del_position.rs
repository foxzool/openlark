//! 删除岗位
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/position/del_position>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 删除岗位请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DelPositionBody {
    /// 岗位 ID（必填），可通过查询岗位信息接口获得
    pub position_id: String,
}

/// 删除岗位请求
#[derive(Debug, Clone)]
pub struct DelPositionRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: DelPositionBody,
}

impl DelPositionRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: DelPositionBody::default(),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: DelPositionBody) -> Self {
        self.body = body;
        self
    }

    /// 设置岗位 ID（必填）。
    pub fn position_id(mut self, position_id: impl Into<String>) -> Self {
        self.body.position_id = position_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DelPositionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DelPositionResponse> {
        validate_required!(self.body.position_id, "position_id 不能为空");

        let request =
            ApiRequest::<DelPositionResponse>::post("/open-apis/corehr/v2/positions/del_position")
                .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 删除岗位响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DelPositionResponse {
    /// 响应数据
    pub data: Value,
}

impl ApiResponseTrait for DelPositionResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/positions/del_position
    /// （官方请求体为 {position_id}，必填）
    #[tokio::test]
    async fn test_del_position_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/positions/del_position"))
            .and(body_json(json!({
                "position_id": "6862995757234914824"
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

        DelPositionRequest::new(config)
            .position_id("6862995757234914824")
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
