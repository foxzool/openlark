//! 创建卡片实体
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::{common::api_utils::serialize_params, endpoints::CARDKIT_V1_CARDS};

/// 创建卡片实体请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCardBody {
    /// 卡片内容
    pub card_content: serde_json::Value,
    /// 卡片类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 卡片类型。
    pub card_type: Option<String>,
    /// 模板ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 模板 ID。
    pub template_id: Option<String>,
    /// 临时卡片标记（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 临时卡片标记。
    pub temp: Option<bool>,
    /// 临时卡片过期时间（可选，秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 临时卡片过期时间。
    pub temp_expire_time: Option<i32>,
}

impl CreateCardBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        if self.card_content.is_null() {
            return Err(openlark_core::CoreError::validation_msg(
                "card_content 不能为空",
            ));
        }
        if !self.card_content.is_object() {
            return Err(openlark_core::CoreError::validation_msg(
                "card_content 必须是 JSON 对象",
            ));
        }

        if let Some(temp_expire_time) = self.temp_expire_time
            && (temp_expire_time <= 0 || temp_expire_time > 86_400)
        {
            return Err(openlark_core::CoreError::validation_msg(
                "temp_expire_time 取值范围为 1~86400（秒）",
            ));
        }

        Ok(())
    }
}

/// 创建卡片实体响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 卡片 ID。
    pub card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 应用 ID。
    pub app_id: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for CreateCardResponse {}

/// 创建卡片实体请求
#[derive(Debug, Clone)]
pub struct CreateCardRequest {
    config: Config,
}

impl CreateCardRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/create>
    pub async fn execute(self, body: CreateCardBody) -> SDKResult<CreateCardResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/create>
    pub async fn execute_with_options(
        self,
        body: CreateCardBody,
        option: RequestOption,
    ) -> SDKResult<CreateCardResponse> {
        body.validate()?;

        // url: POST:/open-apis/cardkit/v1/cards
        let req: ApiRequest<CreateCardResponse> =
            ApiRequest::post(CARDKIT_V1_CARDS).body(serialize_params(&body, "创建卡片实体")?);

        Transport::request_typed(req, &self.config, Some(option), "创建卡片实体").await
    }
}

/// 创建卡片实体请求构建器
#[derive(Debug, Clone)]
pub struct CreateCardRequestBuilder {
    request: CreateCardRequest,
    card_content: Option<serde_json::Value>,
    card_type: Option<String>,
    template_id: Option<String>,
    temp: Option<bool>,
    temp_expire_time: Option<i32>,
}

impl CreateCardRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateCardRequest::new(config),
            card_content: None,
            card_type: None,
            template_id: None,
            temp: None,
            temp_expire_time: None,
        }
    }

    /// 设置卡片内容
    pub fn card_content(mut self, card_content: impl Into<serde_json::Value>) -> Self {
        self.card_content = Some(card_content.into());
        self
    }

    /// 设置卡片类型
    pub fn card_type(mut self, card_type: impl Into<String>) -> Self {
        self.card_type = Some(card_type.into());
        self
    }

    /// 设置模板 ID
    pub fn template_id(mut self, template_id: impl Into<String>) -> Self {
        self.template_id = Some(template_id.into());
        self
    }

    /// 设置临时卡片标记
    pub fn temp(mut self, temp: impl Into<bool>) -> Self {
        self.temp = Some(temp.into());
        self
    }

    /// 设置临时卡片过期时间
    pub fn temp_expire_time(mut self, temp_expire_time: impl Into<i32>) -> Self {
        self.temp_expire_time = Some(temp_expire_time.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> CreateCardRequest {
        CreateCardRequest {
            config: self.request.config,
        }
    }
}

/// 执行创建卡片实体请求
///
/// docPath: <https://open.feishu.cn/document/cardkit-v1/card/create>
pub async fn create(config: &Config, body: CreateCardBody) -> SDKResult<CreateCardResponse> {
    create_with_options(config, body, RequestOption::default()).await
}

/// 执行创建卡片实体请求（支持自定义选项）
///
/// docPath: <https://open.feishu.cn/document/cardkit-v1/card/create>
pub async fn create_with_options(
    config: &Config,
    body: CreateCardBody,
    option: RequestOption,
) -> SDKResult<CreateCardResponse> {
    body.validate()?;

    // url: POST:/open-apis/cardkit/v1/cards
    let req: ApiRequest<CreateCardResponse> =
        ApiRequest::post(CARDKIT_V1_CARDS).body(serialize_params(&body, "创建卡片实体")?);

    Transport::request_typed(req, config, Some(option), "创建卡片实体").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/cardkit/v1/cards + body 序列化 → 强类型 CreateCardResponse。
    #[tokio::test]
    async fn test_create_card_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/cardkit/v1/cards"))
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

        let body = CreateCardBody {
            card_content: json!({ "schema": "2.0" }),
            card_type: None,
            template_id: None,
            temp: None,
            temp_expire_time: None,
        };
        let resp = CreateCardRequest::new(config)
            .execute(body)
            .await
            .expect("创建卡片实体应成功");
        assert_eq!(resp.card_id.as_deref(), Some("card_001"));
        assert_eq!(resp.app_id.as_deref(), Some("app_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["card_content"]["schema"], "2.0");
    }
}
