//! 下载开门时的人脸识别图片
//!
//! 官方响应为文件二进制流（无 JSON `data` 信封），本接口返回原始字节。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/acs-v1/access_record/access_photo/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption, validate_required,
};

/// 下载开门时的人脸识别图片请求
#[derive(Debug)]
pub struct GetAccessPhotoRequest {
    /// 配置信息。
    config: Config,
    /// 门禁记录 ID（路径参数，必填）。
    access_record_id: String,
}

impl GetAccessPhotoRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, access_record_id: impl Into<String>) -> Self {
        Self {
            config,
            access_record_id: access_record_id.into(),
        }
    }

    /// 执行请求，返回人脸识别图片二进制内容。
    pub async fn execute(self) -> SDKResult<Vec<u8>> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<Vec<u8>> {
        validate_required!(self.access_record_id, "access_record_id 不能为空");

        let path = format!(
            "/open-apis/acs/v1/access_records/{}/access_photo",
            self.access_record_id
        );
        let req: ApiRequest<Vec<u8>> =
            ApiRequest::get(&path).with_supported_access_token_types(vec![AccessTokenType::Tenant]);

        Transport::request_typed(req, &self.config, Some(option), "下载开门时的人脸识别图片").await
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
    async fn test_get_access_photo_rejects_empty_id() {
        let req = GetAccessPhotoRequest::new(test_config(), "");
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("access_record_id"));
    }

    /// 端到端：GET .../access_records/{id}/access_photo → 文件二进制流原样返回。
    #[tokio::test]
    async fn test_get_access_photo_returns_binary_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/acs/v1/access_records/rec_001/access_photo",
            ))
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

        let data = GetAccessPhotoRequest::new(config, "rec_001")
            .execute()
            .await
            .expect("下载开门人脸识别图片应成功");
        assert_eq!(data, b"JPEG-BYTES-001".to_vec());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/acs/v1/access_records/rec_001/access_photo"
        );
    }
}
