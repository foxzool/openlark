//! 更新预约
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/update>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::validate_required_field;

/// 更新预约请求

#[derive(Debug, Clone)]
pub struct UpdateReserveRequest {
    /// 配置信息
    config: Config,
    /// 预约 ID（路径参数）
    reserve_id: String,
}

/// 更新预约响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateReserveResponse {
    /// 更新状态
    pub success: bool,
}

impl ApiResponseTrait for UpdateReserveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateReserveRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            reserve_id: String::new(),
        }
    }

    /// 设置预约 ID（路径参数）
    pub fn reserve_id(mut self, reserve_id: impl Into<String>) -> Self {
        self.reserve_id = reserve_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/update>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<UpdateReserveResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<UpdateReserveResponse> {
        validate_required_field("reserve_id", Some(&self.reserve_id), "预约 ID 不能为空")?;

        let api_endpoint = format!("/open-apis/vc/v1/reserves/{}", self.reserve_id);
        let api_request: ApiRequest<UpdateReserveResponse> =
            ApiRequest::put(&api_endpoint).body(serde_json::to_vec(&body)?);

        Transport::request_typed(api_request, &self.config, Some(option), "更新预约").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PUT .../vc/v1/reserves/{reserve_id} → 强类型 UpdateReserveResponse 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_update_reserve_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/vc/v1/reserves/r_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "success": true }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = UpdateReserveRequest::new(config)
            .reserve_id("r_001")
            .execute(json!({ "topic": "更新后的主题" }))
            .await
            .expect("更新预约应成功");
        assert!(resp.success);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/reserves/r_001");
    }
}
