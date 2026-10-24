//! 更新白板主题（v1）
//! docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/update_theme>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新白板主题请求体。
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateWhiteboardThemeBodyV1 {
    /// 主题名称，可选值有 classic、minimalist_gray、retro、vibrant_color、default。长度范围 1 ～ 100000 字符。
    #[serde(skip_serializing_if = "String::is_empty")]
    pub theme: String,
}

/// 更新白板主题响应（官方 data 为空对象）。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateWhiteboardThemeResponseV1 {}

/// 更新白板主题请求构建器。
#[derive(Debug, Clone)]
pub struct UpdateWhiteboardThemeRequestV1 {
    config: Arc<Config>,
    board_id: String,
    body: UpdateWhiteboardThemeBodyV1,
}

impl UpdateWhiteboardThemeRequestV1 {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, board_id: impl Into<String>) -> Self {
        Self {
            config,
            board_id: board_id.into(),
            body: UpdateWhiteboardThemeBodyV1::default(),
        }
    }

    /// 设置主题名称，可选值有 classic、minimalist_gray、retro、vibrant_color、default。
    pub fn theme(mut self, theme: impl Into<String>) -> Self {
        self.body.theme = theme.into();
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<UpdateWhiteboardThemeResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateWhiteboardThemeResponseV1> {
        validate_required!(self.board_id.trim(), "白板 ID 不能为空");

        let api_endpoint =
            crate::common::api_endpoints::BoardApiV1::WhiteboardUpdateTheme(self.board_id);
        let mut request =
            ApiRequest::<UpdateWhiteboardThemeResponseV1>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error("序列化请求体失败", e.to_string().as_str())
        })?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for UpdateWhiteboardThemeResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    /// 成功且响应体无 `data` 字段时按空成功处理（官方 data 为空对象）。
    fn empty_success() -> Option<Self> {
        Some(Self {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn test_whiteboard_update_theme_v1_url() {
        let endpoint = crate::common::api_endpoints::BoardApiV1::WhiteboardUpdateTheme(
            "test_board_id".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/board/v1/whiteboards/test_board_id/update_theme"
        );
    }

    #[test]
    fn test_update_theme_body_shape() {
        let body = UpdateWhiteboardThemeBodyV1 {
            theme: "classic".to_string(),
        };
        let value = serde_json::to_value(&body).expect("序列化应成功");
        assert_eq!(value, json!({"theme": "classic"}));
    }

    /// 端到端：POST /open-apis/board/v1/whiteboards/{whiteboard_id}/update_theme，
    /// 官方请求体仅 {"theme": "..."}，响应 data 为空对象。
    #[tokio::test]
    async fn test_update_whiteboard_theme_returns_empty_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/board/v1/whiteboards/wb_123/update_theme"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        UpdateWhiteboardThemeRequestV1::new(config, "wb_123")
            .theme("classic")
            .execute()
            .await
            .expect("更新主题应成功");

        let received = server.received_requests().await.expect("应收到请求");
        assert_eq!(received.len(), 1);
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(body, json!({"theme": "classic"}));
    }
}
