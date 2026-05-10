//! 更新白板主题（v1）
//! docPath: 

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Default)]
/// 更新白板主题请求体。
pub struct UpdateWhiteboardThemeBodyV1 {
    #[serde(skip_serializing_if = "String::is_empty")]
    /// 主题名称。
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    /// 背景颜色。
    pub background_color: String,
    #[serde(default)]
    /// 主题配置。
    pub theme_config: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
/// 更新白板主题响应。
pub struct UpdateWhiteboardThemeResponseV1 {
    /// 主题 ID。
    pub theme_id: String,
    /// 主题名称。
    pub name: String,
    /// 背景颜色。
    pub background_color: String,
    /// 更新时间。
    pub update_time: i64,
}

#[derive(Debug, Clone)]
/// 更新白板主题请求构建器。
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

    /// 设置主题名称。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置背景颜色。
    pub fn background_color(mut self, background_color: impl Into<String>) -> Self {
        self.body.background_color = background_color.into();
        self
    }

    /// 设置主题配置。
    pub fn theme_config(mut self, theme_config: serde_json::Value) -> Self {
        self.body.theme_config = theme_config;
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

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for UpdateWhiteboardThemeResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
}
