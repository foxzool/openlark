//! 局部更新卡片实体
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card/batch_update>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        validation::validate_card_id,
    },
    endpoints::cardkit_v1_card_batch_update,
};

/// 局部更新卡片实体请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateCardBody {
    /// 卡片 ID
    pub card_id: String,
    /// 操作列表（结构以官方文档为准）
    pub operations: Vec<serde_json::Value>,
}

/// 局部更新卡片实体响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchUpdateCardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 卡片 ID。
    pub card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 应用 ID。
    pub app_id: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for BatchUpdateCardResponse {}

/// 局部更新卡片实体请求
#[derive(Debug, Clone)]
pub struct BatchUpdateCardRequest {
    config: Config,
    card_id: Option<String>,
    operations: Option<Vec<serde_json::Value>>,
}

impl BatchUpdateCardRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            operations: None,
        }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/batch_update>
    pub async fn execute(self, body: BatchUpdateCardBody) -> SDKResult<BatchUpdateCardResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/batch_update>
    pub async fn execute_with_options(
        self,
        body: BatchUpdateCardBody,
        option: RequestOption,
    ) -> SDKResult<BatchUpdateCardResponse> {
        let mut body = body;
        if let Some(card_id) = self.card_id {
            body.card_id = card_id;
        }
        if let Some(operations) = self.operations {
            body.operations = operations;
        }

        validate_card_id(&body.card_id)?;
        if body.operations.is_empty() {
            return Err(openlark_core::error::validation_error(
                "operations 不能为空",
                "operations 不能为空",
            ));
        }

        // url: POST:/open-apis/cardkit/v1/cards/:card_id/batch_update
        let url = cardkit_v1_card_batch_update(&body.card_id);
        let req: ApiRequest<BatchUpdateCardResponse> =
            ApiRequest::post(url).body(serialize_params(&body, "局部更新卡片实体")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "局部更新卡片实体")
    }
}

/// 局部更新卡片实体请求构建器
#[derive(Debug, Clone)]
pub struct BatchUpdateCardRequestBuilder {
    request: BatchUpdateCardRequest,
    card_id: Option<String>,
    operations: Option<Vec<serde_json::Value>>,
}

impl BatchUpdateCardRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchUpdateCardRequest::new(config),
            card_id: None,
            operations: None,
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.card_id = Some(card_id.into());
        self
    }

    /// 设置操作列表
    pub fn operations(mut self, operations: impl Into<Vec<serde_json::Value>>) -> Self {
        self.operations = Some(operations.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchUpdateCardRequest {
        BatchUpdateCardRequest {
            config: self.request.config,
            card_id: self.card_id,
            operations: self.operations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../cards/{card_id}/batch_update + body 序列化 → BatchUpdateCardResponse。
    #[tokio::test]
    async fn test_batch_update_card_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/cardkit/v1/cards/card_001/batch_update"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "card_id": "card_001", "app_id": "app_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body = BatchUpdateCardBody {
            card_id: "card_001".into(),
            operations: vec![json!({ "op": "replace", "path": "/title" })],
        };
        let resp = BatchUpdateCardRequest::new(config)
            .execute(body)
            .await
            .expect("局部更新卡片实体应成功");
        assert_eq!(resp.card_id.as_deref(), Some("card_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["operations"][0]["op"], "replace");
    }
}
