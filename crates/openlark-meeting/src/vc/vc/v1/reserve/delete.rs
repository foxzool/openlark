//! 删除预约
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::validate_required_field;

/// 删除预约请求

#[derive(Debug, Clone)]
pub struct DeleteReserveRequest {
    /// 配置信息
    config: Config,
    /// 预约 ID（路径参数）
    reserve_id: String,
}

/// 删除预约响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteReserveResponse {
    /// 删除状态
    pub success: bool,
}

impl ApiResponseTrait for DeleteReserveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteReserveRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/delete>
    pub async fn execute(self) -> SDKResult<DeleteReserveResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteReserveResponse> {
        validate_required_field("reserve_id", Some(&self.reserve_id), "预约 ID 不能为空")?;

        let api_endpoint = VcApiV1::ReserveDelete(self.reserve_id.clone());
        let api_request: ApiRequest<DeleteReserveResponse> =
            ApiRequest::delete(api_endpoint.to_url());

        Transport::request_typed(api_request, &self.config, Some(option), "删除预约").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../vc/v1/reserves/{reserve_id} → 强类型 DeleteReserveResponse 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_delete_reserve_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
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

        let resp = DeleteReserveRequest::new(config)
            .reserve_id("r_001")
            .execute()
            .await
            .expect("删除预约应成功");
        assert!(resp.success);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/reserves/r_001");
    }
}
