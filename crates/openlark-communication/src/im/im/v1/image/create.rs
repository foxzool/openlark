//! 上传图片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/image/create>

use openlark_core::{SDKResult, api::ApiRequest, config::Config, error, http::Transport};

use crate::{
    endpoints::IM_V1_IMAGES,
    im::v1::image::models::{CreateImageResponse, ImageType},
};

/// 上传图片请求
///
/// 用于上传图片到飞书服务器。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `image_type`: 图片类型，必填
/// - `file_name`: 文件名，可选（仅用于 multipart 的文件名）
///
/// # 示例
///
/// ```rust,ignore
/// let image_bytes = vec![...]; // 图片二进制内容
/// let request = CreateImageRequest::new(config)
///     .image_type(ImageType::Message)
///     .execute(image_bytes).await?;
/// ```
pub struct CreateImageRequest {
    config: Config,
    image_type: Option<ImageType>,
    file_name: Option<String>,
}

impl CreateImageRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            image_type: None,
            file_name: None,
        }
    }

    /// 图片类型（必填）
    pub fn image_type(mut self, image_type: ImageType) -> Self {
        self.image_type = Some(image_type);
        self
    }

    /// 可选：设置上传文件名（仅用于 multipart 的文件名，不作为字段发送）
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口为 multipart 上传，请直接传入图片二进制内容。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/image/create>
    pub async fn execute(self, image_bytes: Vec<u8>) -> SDKResult<CreateImageResponse> {
        self.execute_with_options(
            image_bytes,
            openlark_core::req_option::RequestOption::default(),
        )
        .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        image_bytes: Vec<u8>,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateImageResponse> {
        // === 必填字段验证 ===
        let image_type = self.image_type.ok_or_else(|| {
            error::validation_error(
                "image_type 不能为空".to_string(),
                "上传图片需要指定 image_type".to_string(),
            )
        })?;

        // image 二进制文件部件：内容经 `.file_content(...)` 传入，部件名经
        // `.file_part_name("image")` 指定（官方请求体示例 name="image"）。
        // body 中以 null 声明 `image` 仅为对齐官方 body 参数表——multipart
        // 构建器会跳过 null 值，不会作为文本字段发送。
        let mut body = serde_json::json!({
            "image_type": image_type.as_str(),
            "image": serde_json::Value::Null,
        });

        if let Some(file_name) = self.file_name {
            // 仅用于 multipart 文件名，不作为字段发送
            if let Some(obj) = body.as_object_mut() {
                obj.insert(
                    "__file_name".to_string(),
                    serde_json::Value::String(file_name),
                );
            }
        }

        // url: POST:/open-apis/im/v1/images
        // 官方请求体示例中文件部件名为 image（name="image"），非通用默认值 file
        let req: ApiRequest<CreateImageResponse> = ApiRequest::post(IM_V1_IMAGES)
            .body(body)
            .file_content(image_bytes)
            .file_part_name("image");

        Transport::request_typed(req, &self.config, Some(option), "上传图片").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_image_request_builder() {
        let config = Config::default();
        let request = CreateImageRequest::new(config).image_type(ImageType::Message);
        assert_eq!(request.image_type, Some(ImageType::Message));
        assert!(request.file_name.is_none());
    }

    #[test]
    fn test_create_image_request_default_values() {
        let config = Config::default();
        let request = CreateImageRequest::new(config);
        assert!(request.image_type.is_none());
        assert!(request.file_name.is_none());
    }

    #[test]
    fn test_create_image_request_with_file_name() {
        let config = Config::default();
        let request = CreateImageRequest::new(config)
            .image_type(ImageType::Avatar)
            .file_name("photo.jpg");
        assert_eq!(request.image_type, Some(ImageType::Avatar));
        assert_eq!(request.file_name, Some("photo.jpg".to_string()));
    }

    #[test]
    fn test_create_image_request_chaining() {
        let config = Config::default();
        let request = CreateImageRequest::new(config)
            .image_type(ImageType::Message)
            .file_name("image.png");
        assert_eq!(request.image_type, Some(ImageType::Message));
        assert_eq!(request.file_name, Some("image.png".to_string()));
    }

    /// 端到端：POST /open-apis/im/v1/images（multipart）
    /// 官方契约要求文件部件名为 image（name="image"），image_type 作为普通表单字段。
    #[tokio::test]
    async fn test_upload_image_multipart_part_name_is_image() {
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 注意：mock 响应体不用 json! 宏构造（字段核对工具会扫描文件内
        // json! 字面量的顶层键，会把响应字段误当 multipart 表单字段）。
        let mock_body = r#"{"code":0,"msg":"success","data":{"image_key":"img_v2_123"}}"#;
        Mock::given(method("POST"))
            .and(path("/open-apis/im/v1/images"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(mock_body.as_bytes().to_vec(), "application/json"),
            )
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let response = CreateImageRequest::new(config)
            .image_type(ImageType::Message)
            .file_name("image.jpg")
            .execute(vec![1, 2, 3, 4])
            .await
            .expect("上传图片请求应成功");
        assert_eq!(response.image_key, "img_v2_123");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let body = String::from_utf8_lossy(&received[0].body);
        // 文件部件名必须是官方要求的 image，而不是通用默认值 file
        assert!(
            body.contains("name=\"image\""),
            "multipart 文件部件名应为 image，实际 body: {body}"
        );
        assert!(
            !body.contains("name=\"file\""),
            "multipart 不应再使用默认部件名 file，实际 body: {body}"
        );
        // image_type 作为普通表单字段发送
        assert!(body.contains("name=\"image_type\""));
    }
}
