//! 更新卡片实体配置
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card/settings>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::serialize_params,
        validation::{validate_card_id, validate_sequence, validate_uuid},
    },
    endpoints::cardkit_v1_card_settings,
};

/// 更新卡片实体配置请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCardSettingsBody {
    /// 卡片 ID（路径参数，不进入 JSON body）
    #[serde(skip_serializing)]
    pub card_id: String,
    /// 卡片配置（含 `config` / `card_link` 的 JSON 序列化字符串）
    pub settings: String,
    /// 幂等 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// 流式更新序号（必填，严格递增）
    pub sequence: i32,
}

impl UpdateCardSettingsBody {
    /// 校验请求体。
    pub fn validate(&self) -> SDKResult<()> {
        validate_card_id(&self.card_id)?;
        validate_required!(self.settings, "settings 不能为空");
        validate_uuid(&self.uuid)?;
        validate_sequence(self.sequence)?;
        Ok(())
    }
}

/// 更新卡片实体配置响应（官方 `data` 为空对象）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardSettingsResponse {}

impl openlark_core::api::ApiResponseTrait for UpdateCardSettingsResponse {}

/// 更新卡片实体配置请求
#[derive(Debug, Clone)]
pub struct UpdateCardSettingsRequest {
    config: Config,
    card_id: Option<String>,
    settings: Option<String>,
    uuid: Option<String>,
    sequence: Option<i32>,
}

impl UpdateCardSettingsRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            settings: None,
            uuid: None,
            sequence: None,
        }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/settings>
    pub async fn execute(
        self,
        body: UpdateCardSettingsBody,
    ) -> SDKResult<UpdateCardSettingsResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/settings>
    pub async fn execute_with_options(
        self,
        body: UpdateCardSettingsBody,
        option: RequestOption,
    ) -> SDKResult<UpdateCardSettingsResponse> {
        let mut body = body;
        if let Some(card_id) = self.card_id {
            body.card_id = card_id;
        }
        if let Some(settings) = self.settings {
            body.settings = settings;
        }
        if let Some(uuid) = self.uuid {
            body.uuid = Some(uuid);
        }
        if let Some(sequence) = self.sequence {
            body.sequence = sequence;
        }

        body.validate()?;

        // url: PATCH:/open-apis/cardkit/v1/cards/:card_id/settings
        let url = cardkit_v1_card_settings(&body.card_id);
        let req: ApiRequest<UpdateCardSettingsResponse> =
            ApiRequest::patch(url).body(serialize_params(&body, "更新卡片实体配置")?);

        Transport::request_typed(req, &self.config, Some(option), "更新卡片实体配置").await
    }
}

/// 更新卡片实体配置请求构建器
#[derive(Debug, Clone)]
pub struct UpdateCardSettingsRequestBuilder {
    request: UpdateCardSettingsRequest,
}

impl UpdateCardSettingsRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateCardSettingsRequest::new(config),
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.request.card_id = Some(card_id.into());
        self
    }

    /// 设置配置
    pub fn settings(mut self, settings: impl Into<String>) -> Self {
        self.request.settings = Some(settings.into());
        self
    }

    /// 设置幂等 ID
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.request.uuid = Some(uuid.into());
        self
    }

    /// 设置流式更新序号
    pub fn sequence(mut self, sequence: i32) -> Self {
        self.request.sequence = Some(sequence);
        self
    }

    /// 构建请求
    pub fn build(self) -> UpdateCardSettingsRequest {
        self.request
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH .../cards/{card_id}/settings + body 序列化 → UpdateCardSettingsResponse。
    #[tokio::test]
    async fn test_update_card_settings_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/cardkit/v1/cards/card_001/settings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body = UpdateCardSettingsBody {
            card_id: "card_001".into(),
            settings: r#"{"config":{"streaming_mode":true}}"#.into(),
            uuid: None,
            sequence: 1,
        };
        UpdateCardSettingsRequest::new(config)
            .execute(body)
            .await
            .expect("更新卡片实体配置应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert!(sent.get("card_id").is_none());
        assert_eq!(sent["settings"], r#"{"config":{"streaming_mode":true}}"#);
        assert_eq!(sent["sequence"], 1);
    }
}
