//! 上传勋章图片 API
//! docPath: https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/create

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 上传勋章图片的请求构建器。
pub struct CreateBadgeImageBuilder {
    image: String,
    config: Config,
}

impl CreateBadgeImageBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            image: String::new(),
            config,
        }
    }

    /// 设置图片内容。
    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.image = image.into();
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<CreateBadgeImageResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateBadgeImageResponse> {
        validate_required!(self.image, "图片不能为空");

        let request_body = CreateBadgeImageRequest { image: self.image };
        let api_request: ApiRequest<CreateBadgeImageResponse> =
            ApiRequest::post("/open-apis/admin/v1/badge_images")
                .body(serde_json::to_value(&request_body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("上传勋章图片", "响应数据为空"))
    }
}

#[derive(Debug, Serialize)]
struct CreateBadgeImageRequest {
    image: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 上传勋章图片的响应。
pub struct CreateBadgeImageResponse {
    /// 图片 ID。
    pub image_id: String,
    /// 图片访问地址。
    pub image_url: String,
}

impl ApiResponseTrait for CreateBadgeImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateBadgeImageBuilder::new(config.clone()).image("test".to_string());
        let _ = request;
    }
}
