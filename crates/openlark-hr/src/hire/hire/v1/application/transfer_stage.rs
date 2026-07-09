//! 转移投递阶段
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/application/transfer_stage>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hire::hire::common_models::ApplicationOperationResult;

/// 转移投递阶段请求
#[derive(Debug, Clone)]
pub struct TransferStageRequest {
    /// 配置信息
    config: Config,
    application_id: String,
    request_body: Value,
}

impl TransferStageRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            application_id: String::new(),
            request_body: Value::Null,
        }
    }

    /// 设置 `application_id`。
    pub fn application_id(mut self, application_id: String) -> Self {
        self.application_id = application_id;
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TransferStageResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<TransferStageResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.application_id.trim(), "投递 ID 不能为空");
        if self.request_body.is_null() {
            return Err(openlark_core::error::validation_error(
                "请求体不能为空",
                "转移投递阶段时 request_body 为必填参数",
            ));
        }

        let api_endpoint = HireApiV1::ApplicationTransferStage(self.application_id.clone());
        let request = ApiRequest::<TransferStageResponse>::post(api_endpoint.to_url())
            .body(self.request_body);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "转移投递阶段响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 转移投递阶段响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TransferStageResponse {
    #[serde(flatten)]
    /// `operation` 字段。
    pub operation: ApplicationOperationResult,
}

impl ApiResponseTrait for TransferStageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/hire/v1/applications/test001/transfer_stage
    #[tokio::test]
    async fn test_transfer_stage_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/hire/v1/applications/test001/transfer_stage",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "operation": {  } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        TransferStageRequest::new(config)
            .application_id("test001".to_string())
            .request_body(json!({"k": "v"}))
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
