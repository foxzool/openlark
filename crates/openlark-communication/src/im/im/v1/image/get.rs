//! 下载图片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/image/get>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{common::api_utils::extract_response_data, endpoints::IM_V1_IMAGES};

/// 下载图片请求
///
/// 用于从飞书服务器下载图片。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `image_key`: 图片 key，必填
///
/// # 示例
///
/// ```rust,ignore
/// let request = GetImageRequest::new(config)
///     .image_key("img_key_xxx")
///     .execute().await?;
/// ```
pub struct GetImageRequest {
    config: Config,
    image_key: String,
}

impl GetImageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            image_key: String::new(),
        }
    }

    /// 图片 key（路径参数）
    pub fn image_key(mut self, image_key: impl Into<String>) -> Self {
        self.image_key = image_key.into();
        self
    }

    /// 执行请求（返回二进制内容）
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/image/get>
    pub async fn execute(self) -> SDKResult<Vec<u8>> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<Vec<u8>> {
        // === 必填字段验证 ===
        validate_required!(self.image_key, "image_key 不能为空");

        // url: GET:/open-apis/im/v1/images/:image_key
        let req: ApiRequest<Vec<u8>> =
            ApiRequest::get(format!("{}/{}", IM_V1_IMAGES, self.image_key));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "下载图片")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_image_request_builder() {
        let config = Config::default();
        let request = GetImageRequest::new(config).image_key("img_key_xxx");
        assert_eq!(request.image_key, "img_key_xxx");
    }

    #[test]
    fn test_get_image_request_default_values() {
        let config = Config::default();
        let request = GetImageRequest::new(config);
        assert_eq!(request.image_key, "");
    }

    #[test]
    fn test_get_image_request_with_different_keys() {
        let config = Config::default();
        let request1 = GetImageRequest::new(config.clone()).image_key("img_111");
        let request2 = GetImageRequest::new(config).image_key("img_222");
        assert_eq!(request1.image_key, "img_111");
        assert_eq!(request2.image_key, "img_222");
    }
}
