//! 下载用户人脸图片
//!
//! 官方响应为文件二进制流（无 JSON `data` 信封），本接口返回原始字节。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/acs-v1/user/face/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption, validate_required,
};

/// 下载用户人脸图片请求
#[derive(Debug)]
pub struct GetUserFaceRequest {
    /// 配置信息。
    config: Config,
    /// 用户 ID（路径参数，必填）。
    user_id: String,
    /// 裁剪图。
    is_cropped: Option<bool>,
    /// 用户 ID 类型（open_id/union_id/user_id）。
    user_id_type: Option<String>,
}

impl GetUserFaceRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, user_id: impl Into<String>) -> Self {
        Self {
            config,
            user_id: user_id.into(),
            is_cropped: None,
            user_id_type: None,
        }
    }

    /// 裁剪图（查询参数）。
    pub fn is_cropped(mut self, is_cropped: bool) -> Self {
        self.is_cropped = Some(is_cropped);
        self
    }

    /// 用户 ID 类型（open_id/union_id/user_id，查询参数）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求，返回人脸图片二进制内容。
    pub async fn execute(self) -> SDKResult<Vec<u8>> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<Vec<u8>> {
        validate_required!(self.user_id, "user_id 不能为空");

        let path = format!("/open-apis/acs/v1/users/{}/face", self.user_id);
        let mut req: ApiRequest<Vec<u8>> =
            ApiRequest::get(&path).with_supported_access_token_types(vec![AccessTokenType::Tenant]);
        if let Some(is_cropped) = self.is_cropped {
            req = req.query("is_cropped", is_cropped.to_string());
        }
        if let Some(user_id_type) = &self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        Transport::request_typed(req, &self.config, Some(option), "下载用户人脸图片").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build()
    }

    #[tokio::test]
    async fn test_get_user_face_rejects_empty_id() {
        let req = GetUserFaceRequest::new(test_config(), "  ");
        let result = req.execute_with_options(RequestOption::default()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("user_id"));
    }

    /// 端到端：GET .../users/{id}/face → 文件二进制流原样返回。
    #[tokio::test]
    async fn test_get_user_face_returns_binary_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/acs/v1/users/u_001/face"))
            .and(query_param("is_cropped", "true"))
            .and(query_param("user_id_type", "open_id"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/jpeg")
                    .set_body_bytes(b"JPEG-BYTES-001".as_slice()),
            )
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = GetUserFaceRequest::new(config, "u_001")
            .is_cropped(true)
            .user_id_type("open_id")
            .execute()
            .await
            .expect("下载用户人脸图片应成功");
        assert_eq!(data, b"JPEG-BYTES-001".to_vec());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/acs/v1/users/u_001/face");
    }
}
