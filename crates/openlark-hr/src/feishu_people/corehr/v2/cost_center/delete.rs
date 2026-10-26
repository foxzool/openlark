//! 删除成本中心
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/cost_center/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 删除成本中心请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteBody {
    /// 操作原因（必填）
    pub operation_reason: String,
}

/// DeleteRequest
#[derive(Debug, Clone)]
pub struct DeleteRequest {
    /// 配置信息
    config: Config,
    cost_center_id: Option<String>,
    /// 请求体
    body: DeleteBody,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            cost_center_id: None,
            body: DeleteBody::default(),
        }
    }

    /// 设置 `cost_center_id`。
    pub fn cost_center_id(mut self, cost_center_id: impl Into<String>) -> Self {
        self.cost_center_id = Some(cost_center_id.into());
        self
    }

    /// 设置请求体
    pub fn body(mut self, body: DeleteBody) -> Self {
        self.body = body;
        self
    }

    /// 设置操作原因（必填）。
    pub fn operation_reason(mut self, operation_reason: impl Into<String>) -> Self {
        self.body.operation_reason = operation_reason.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResponse> {
        let cost_center_id = self.cost_center_id.unwrap_or_default();
        validate_required!(cost_center_id.trim(), "cost_center_id 不能为空");
        validate_required!(self.body.operation_reason, "operation_reason 不能为空");

        let request = ApiRequest::<DeleteResponse>::delete(format!(
            "/open-apis/corehr/v2/cost_centers/{cost_center_id}"
        ))
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// DeleteResponse
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 响应数据
    pub data: Value,
}

impl ApiResponseTrait for DeleteResponse {
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

    /// 端到端：DELETE /open-apis/corehr/v2/cost_centers/test001
    #[tokio::test]
    async fn test_delete_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/corehr/v2/cost_centers/test001"))
            .and(body_json(json!({
                "operation_reason": "随着组织架构调整，该成本中心不再使用"
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

        DeleteRequest::new(config)
            .cost_center_id("test001".to_string())
            .operation_reason("随着组织架构调整，该成本中心不再使用")
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
