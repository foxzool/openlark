//! 下载白板为图片（v1）
//! docPath: https://open.feishu.cn/document/docs/board-v1/whiteboard/download_as_image

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Default)]
/// 下载白板图片请求体。
pub struct DownloadWhiteboardAsImageBodyV1 {
    #[serde(rename = "format")]
    /// 输出图片格式。
    pub format: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    /// 输出质量。
    pub quality: String,
    #[serde(default)]
    /// 是否包含缩略图。
    pub include_thumbnail: bool,
}

#[derive(Debug, Clone, Deserialize)]
/// 下载白板图片响应。
pub struct DownloadWhiteboardAsImageResponseV1 {
    /// 下载地址。
    pub image_url: String,
    /// 图片宽度。
    pub width: u32,
    /// 图片高度。
    pub height: u32,
    #[serde(default)]
    /// 文件大小。
    pub file_size: u64,
    #[serde(default)]
    /// 缩略图地址。
    pub thumbnail_url: String,
}

#[derive(Debug, Clone)]
/// 下载白板图片请求构建器。
pub struct DownloadWhiteboardAsImageRequestV1 {
    config: Arc<Config>,
    board_id: String,
    body: DownloadWhiteboardAsImageBodyV1,
}

impl DownloadWhiteboardAsImageRequestV1 {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, board_id: impl Into<String>) -> Self {
        Self {
            config,
            board_id: board_id.into(),
            body: DownloadWhiteboardAsImageBodyV1::default(),
        }
    }

    /// 设置输出图片格式。
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.body.format = format.into();
        self
    }

    /// 设置输出质量。
    pub fn quality(mut self, quality: impl Into<String>) -> Self {
        self.body.quality = quality.into();
        self
    }

    /// 设置是否包含缩略图。
    pub fn include_thumbnail(mut self, include_thumbnail: bool) -> Self {
        self.body.include_thumbnail = include_thumbnail;
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<DownloadWhiteboardAsImageResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DownloadWhiteboardAsImageResponseV1> {
        validate_required!(self.board_id.trim(), "白板 ID 不能为空");
        validate_required!(self.body.format.trim(), "图片格式不能为空");

        let api_endpoint =
            crate::common::api_endpoints::BoardApiV1::WhiteboardDownloadAsImage(self.board_id);
        let mut request =
            ApiRequest::<DownloadWhiteboardAsImageResponseV1>::get(api_endpoint.to_url());

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

impl ApiResponseTrait for DownloadWhiteboardAsImageResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_whiteboard_download_as_image_v1_url() {
        let endpoint = crate::common::api_endpoints::BoardApiV1::WhiteboardDownloadAsImage(
            "test_board_id".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/board/v1/whiteboards/test_board_id/download_as_image"
        );
    }
}
