//! 人脸识别管理 API
//!
//! 提供用户人脸图片的上传和下载功能。

use std::sync::Arc;

use openlark_core::error::api_error;
/// 人脸识别管理服务
#[derive(Debug)]
pub struct UserFacesService {
    config: Arc<crate::models::SecurityConfig>,
}

impl UserFacesService {
    /// 创建新的人脸识别管理服务实例
    pub fn new(config: Arc<crate::models::SecurityConfig>) -> Self {
        Self { config }
    }

    /// 下载人脸图片
    pub fn get(&self) -> GetUserFaceBuilder {
        GetUserFaceBuilder {
            config: self.config.clone(),
            user_id: String::new(),
        }
    }

    /// 上传人脸图片
    pub fn update(&self) -> UpdateUserFaceBuilder {
        UpdateUserFaceBuilder {
            config: self.config.clone(),
            user_id: String::new(),
            face_image: Vec::new(),
            image_format: "jpeg".to_string(),
        }
    }
}

/// 获取用户人脸图片构建器
#[derive(Debug)]
pub struct GetUserFaceBuilder {
    config: Arc<crate::models::SecurityConfig>,
    user_id: String,
}

impl GetUserFaceBuilder {
    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 发送请求下载人脸图片
    pub async fn send(self) -> crate::SecurityResult<crate::models::acs::FaceImageInfo> {
        let url = format!(
            "{}/open-apis/acs/v1/users/{}/face",
            self.config.base_url, self.user_id
        );

        let response = reqwest::Client::new()
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<crate::models::acs::FaceImageInfo> =
                response.json().await?;
            match api_response.data {
                Some(face_info) => Ok(face_info),
                None => Err(api_error(
                    api_response.code as u16,
                    "/acs/v1/user_faces",
                    &api_response.msg,
                    None,
                )),
            }
        } else {
            Err(api_error(
                response.status().as_u16(),
                "/acs/v1/user_faces",
                format!("HTTP: {}", response.status()),
                None,
            ))
        }
    }
}

/// 更新用户人脸图片构建器
#[derive(Debug)]
pub struct UpdateUserFaceBuilder {
    config: Arc<crate::models::SecurityConfig>,
    user_id: String,
    face_image: Vec<u8>,
    image_format: String,
}

impl UpdateUserFaceBuilder {
    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 设置人脸图片数据
    pub fn face_image(mut self, face_image: Vec<u8>) -> Self {
        self.face_image = face_image;
        self
    }

    /// 设置图片格式
    pub fn image_format(mut self, image_format: impl Into<String>) -> Self {
        self.image_format = image_format.into();
        self
    }

    /// 从文件路径加载人脸图片
    pub async fn face_image_from_file(
        mut self,
        file_path: impl AsRef<std::path::Path>,
    ) -> crate::SecurityResult<Self> {
        use std::fs;
        let image_data = fs::read(file_path).map_err(|_e| {
            openlark_core::error::network_error_with_details(
                "Failed to read image file",
                None,
                Some("image_file_read".to_string()),
            )
        })?;
        self.face_image = image_data;
        Ok(self)
    }

    /// 发送请求上传人脸图片
    pub async fn send(self) -> crate::SecurityResult<crate::models::acs::FaceImageInfo> {
        let url = format!(
            "{}/open-apis/acs/v1/users/{}/face",
            self.config.base_url, self.user_id
        );

        // 创建 multipart 表单数据
        let form = reqwest::multipart::Form::new().part(
            "image",
            reqwest::multipart::Part::bytes(self.face_image)
                .file_name(format!("face_image.{}", self.image_format))
                .mime_str(&format!("image/{}", self.image_format))
                .map_err(|e| {
                    openlark_core::error::validation_error(
                        "mime_type",
                        format!("Invalid mime type: {e}"),
                    )
                })?,
        );

        let response = reqwest::Client::new()
            .put(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .multipart(form)
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<crate::models::acs::FaceImageInfo> =
                response.json().await?;
            match api_response.data {
                Some(face_info) => Ok(face_info),
                None => Err(api_error(
                    api_response.code as u16,
                    "/acs/v1/user_faces",
                    &api_response.msg,
                    None,
                )),
            }
        } else {
            Err(api_error(
                response.status().as_u16(),
                "/acs/v1/user_faces",
                format!("HTTP: {}", response.status()),
                None,
            ))
        }
    }
}

/// 获取应用访问令牌的辅助函数
async fn get_app_token(config: &crate::models::SecurityConfig) -> crate::SecurityResult<String> {
    // 使用 SecurityConfig 的 get_app_access_token 方法获取真实的 token
    config.get_app_access_token().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn create_test_config() -> Arc<crate::models::SecurityConfig> {
        Arc::new(crate::models::SecurityConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
        })
    }

    #[test]
    fn test_user_faces_service_creation() {
        let config = create_test_config();
        let service = UserFacesService::new(config.clone());
        assert_eq!(service.config.app_id, "test_app_id");
    }

    #[test]
    fn test_get_user_face_builder() {
        let config = create_test_config();
        let service = UserFacesService::new(config);
        let builder = service.get().user_id("user_123");
        assert_eq!(builder.user_id, "user_123");
    }

    #[test]
    fn test_update_user_face_builder_defaults() {
        let config = create_test_config();
        let service = UserFacesService::new(config);
        let builder = service.update();
        assert_eq!(builder.user_id, String::new());
        assert!(builder.face_image.is_empty());
        assert_eq!(builder.image_format, "jpeg");
    }

    #[test]
    fn test_update_user_face_builder_with_params() {
        let config = create_test_config();
        let service = UserFacesService::new(config);
        let image_data = vec![0x01, 0x02, 0x03, 0x04];
        let builder = service
            .update()
            .user_id("user_456")
            .face_image(image_data.clone())
            .image_format("png");

        assert_eq!(builder.user_id, "user_456");
        assert_eq!(builder.face_image, image_data);
        assert_eq!(builder.image_format, "png");
    }

    #[test]
    fn test_update_user_face_builder_chaining() {
        let config = create_test_config();
        let service = UserFacesService::new(config);
        let builder = service.update().user_id("user_789").image_format("jpeg");

        assert_eq!(builder.user_id, "user_789");
        assert_eq!(builder.image_format, "jpeg");
        assert!(builder.face_image.is_empty()); // 默认为空
    }

    #[test]
    fn test_update_user_face_image_format_variants() {
        let config = create_test_config();
        let service = UserFacesService::new(config);

        // 测试 jpeg 格式
        let builder_jpeg = service.update().image_format("jpeg");
        assert_eq!(builder_jpeg.image_format, "jpeg");

        // 测试 png 格式
        let builder_png = service.update().image_format("png");
        assert_eq!(builder_png.image_format, "png");

        // 测试 webp 格式
        let builder_webp = service.update().image_format("webp");
        assert_eq!(builder_webp.image_format, "webp");
    }
}
