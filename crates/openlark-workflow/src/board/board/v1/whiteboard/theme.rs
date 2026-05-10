//! 获取白板主题（v1）
//! docPath: 

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
/// 白板主题信息。
pub struct WhiteboardTheme {
    /// 主题 ID。
    pub theme_id: String,
    /// 主题名称。
    pub name: String,
    /// 背景颜色。
    pub background_color: String,
    #[serde(default)]
    /// 主题配置。
    pub theme_config: serde_json::Value,
    /// 创建时间。
    pub create_time: i64,
    #[serde(default)]
    /// 更新时间。
    pub update_time: i64,
}

#[derive(Debug, Clone, Deserialize)]
/// 获取白板主题响应。
pub struct GetWhiteboardThemeResponseV1 {
    /// 主题详情。
    pub theme: WhiteboardTheme,
}

#[derive(Debug, Clone)]
/// 获取白板主题请求构建器。
pub struct GetWhiteboardThemeRequestV1 {
    config: Arc<Config>,
    board_id: String,
}

impl GetWhiteboardThemeRequestV1 {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, board_id: impl Into<String>) -> Self {
        Self {
            config,
            board_id: board_id.into(),
        }
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<GetWhiteboardThemeResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetWhiteboardThemeResponseV1> {
        validate_required!(self.board_id.trim(), "白板 ID 不能为空");

        let api_endpoint = crate::common::api_endpoints::BoardApiV1::WhiteboardTheme(self.board_id);
        let request = ApiRequest::<GetWhiteboardThemeResponseV1>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for GetWhiteboardThemeResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_whiteboard_theme_v1_url() {
        let endpoint =
            crate::common::api_endpoints::BoardApiV1::WhiteboardTheme("test_board_id".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/board/v1/whiteboards/test_board_id/theme"
        );
    }
}
