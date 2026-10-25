//! 更新外部 Offer
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/external_offer/update>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::ExternalOffer;

/// 更新外部 Offer请求
#[derive(Debug, Clone)]
pub struct UpdateRequest {
    /// 配置信息
    config: Config,
    external_offer_id: String,
    request_body: Option<Value>,
}

impl UpdateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            external_offer_id: String::new(),
            request_body: None,
        }
    }

    /// 设置 `external_offer_id`。
    pub fn external_offer_id(mut self, external_offer_id: impl Into<String>) -> Self {
        self.external_offer_id = external_offer_id.into();
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateResponse> {
        validate_required!(self.external_offer_id.trim(), "external_offer_id 不能为空");

        let mut request = ApiRequest::<UpdateResponse>::put(format!(
            "/open-apis/hire/v1/external_offers/{}",
            self.external_offer_id
        ));
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "更新外部 Offer 响应数据为空",
        )
        .await
    }
}

/// 更新外部 Offer响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// `external_offer` 字段（官方响应 data 下唯一容器）。
    pub external_offer: Option<ExternalOffer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_offer_id` 字段（旧版平铺字段，官方现行响应中不存在，保留以兼容既有消费者）。
    pub external_offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_id` 字段（旧版平铺字段，官方现行响应中不存在，保留以兼容既有消费者）。
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段（旧版平铺字段，官方现行响应中不存在，保留以兼容既有消费者）。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段（旧版平铺字段，官方现行响应中不存在，保留以兼容既有消费者）。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for UpdateResponse {
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

    /// 端到端：PUT /open-apis/hire/v1/external_offers/test001
    #[tokio::test]
    async fn test_update_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/hire/v1/external_offers/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "SUCCESS",
                "data": {
                    "external_offer": {
                        "id": "6989202908470446380",
                        "external_application_id": "7395015673275697419",
                        "biz_create_time": "1721899352428",
                        "owner": "张三",
                        "offer_status": "已发送",
                        "attachment_list": [
                            { "id": "6987954043925432620", "name": "test_resume.pdf", "size": 126371 }
                        ]
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = UpdateRequest::new(config)
            .external_offer_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");
        let offer = resp.external_offer.expect("应返回 external_offer 容器");
        assert_eq!(offer.id.as_deref(), Some("6989202908470446380"));
        assert_eq!(
            offer.external_application_id.as_deref(),
            Some("7395015673275697419")
        );
        assert_eq!(offer.owner.as_deref(), Some("张三"));
        assert_eq!(offer.offer_status.as_deref(), Some("已发送"));
        let attachment = offer
            .attachment_list
            .as_ref()
            .and_then(|l| l.first())
            .expect("应返回附件列表");
        assert_eq!(attachment.size, Some(126371));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
