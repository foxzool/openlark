//! 上传勋章图片 API
//! docPath: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 勋章图片类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeImageType {
    /// 勋章详情图
    Detail = 1,
    /// 勋章挂饰图
    Show = 2,
}

impl BadgeImageType {
    /// 图片类型值（官方取值 1～2）
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

/// 上传勋章图片的请求构建器。
///
/// 该接口为 multipart/form-data 上传：图片二进制经 `execute(image_bytes)` 传入，
/// 以官方部件名 `image_file` 发送；`image_type` 以表单字段发送。
pub struct CreateBadgeImageRequestBuilder {
    image_type: Option<BadgeImageType>,
    file_name: Option<String>,
    config: Config,
}

impl CreateBadgeImageRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            image_type: None,
            file_name: None,
            config,
        }
    }

    /// 设置图片类型（1：勋章详情图；2：勋章挂饰图）。
    pub fn image_type(mut self, image_type: BadgeImageType) -> Self {
        self.image_type = Some(image_type);
        self
    }

    /// 可选：设置上传文件名（仅用于 multipart 的文件名，不作为字段发送）。
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self, image_bytes: Vec<u8>) -> SDKResult<CreateBadgeImageResponse> {
        self.execute_with_options(image_bytes, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        image_bytes: Vec<u8>,
        option: RequestOption,
    ) -> SDKResult<CreateBadgeImageResponse> {
        let image_type = self.image_type.ok_or_else(|| {
            openlark_core::error::validation_error(
                "image_type 不能为空".to_string(),
                "上传勋章图片需要指定 image_type".to_string(),
            )
        })?;
        if image_bytes.is_empty() {
            return Err(openlark_core::error::validation_error(
                "image_file".to_string(),
                "勋章图片内容不能为空".to_string(),
            ));
        }

        // image_file 二进制文件部件：内容经 `.file_content(...)` 传入，部件名经
        // `.file_part_name("image_file")` 指定（官方请求体示例 name="image_file"）。
        // body 中以 null 声明 `image_file` 仅为对齐官方 body 参数表——multipart
        // 构建器会跳过 null 值，不会作为文本字段发送。
        let mut body = serde_json::json!({
            "image_type": image_type.as_i32(),
            "image_file": serde_json::Value::Null,
        });
        if let Some(file_name) = self.file_name
            && let Some(obj) = body.as_object_mut()
        {
            // 仅用于 multipart 文件名，不作为字段发送
            obj.insert(
                "__file_name".to_string(),
                serde_json::Value::String(file_name),
            );
        }

        let api_request: ApiRequest<CreateBadgeImageResponse> =
            ApiRequest::post("/open-apis/admin/v1/badge_images")
                .body(body)
                .file_content(image_bytes)
                .file_part_name("image_file");

        Transport::request_typed(api_request, &self.config, Some(option), "上传勋章图片").await
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 上传勋章图片的响应。
pub struct CreateBadgeImageResponse {
    /// 图片的 key。
    pub image_key: String,
}

impl ApiResponseTrait for CreateBadgeImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to CreateBadgeImageRequestBuilder, will be removed in v1.0 (#271)")]
pub type CreateBadgeImageBuilder = CreateBadgeImageRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateBadgeImageRequestBuilder::new(config.clone())
            .image_type(BadgeImageType::Detail)
            .file_name("badge.png");
        let _ = request;
    }

    /// 端到端：POST .../admin/v1/badge_images（multipart）→ 强类型 CreateBadgeImageResponse。
    #[tokio::test]
    async fn test_create_badge_image_multipart_upload() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // multipart 体：image_type 表单字段 + image_file 文件部件（PNG 魔数）
        let image_bytes: Vec<u8> = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        Mock::given(method("POST"))
            .and(path("/open-apis/admin/v1/badge_images"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "image_key": "f02a98aa-1413-4af6-93ab-431ba9e5f2cg"
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

        let resp = CreateBadgeImageRequestBuilder::new(config)
            .image_type(BadgeImageType::Show)
            .execute(image_bytes)
            .await
            .expect("上传勋章图片应成功");
        assert_eq!(resp.image_key, "f02a98aa-1413-4af6-93ab-431ba9e5f2cg");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].method, "POST");
        let body_text = String::from_utf8_lossy(&received[0].body);
        // 官方 multipart 形状：文件部件名 image_file + 表单字段 image_type（值为 2）
        assert!(body_text.contains("name=\"image_file\""));
        assert!(body_text.contains("name=\"image_type\""));
        assert!(body_text.contains("\n2\r\n") || body_text.contains("\n2\n"));
        // Content-Type 为 multipart/form-data
        let content_type = received[0]
            .headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(content_type.starts_with("multipart/form-data"));
    }
}
