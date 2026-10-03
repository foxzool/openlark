//! 云空间文件生成妙记
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute/upload>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::minutes::MinutesExtraApiV1;
use crate::common::api_utils::serialize_params;

/// 云空间文件生成妙记请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MinuteUploadBody {
    /// 云空间文件 token。
    pub file_token: String,
}

/// 云空间文件生成妙记响应 data。
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct MinuteUploadResponse {
    /// 妙记链接。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute_url: Option<String>,
}

impl ApiResponseTrait for MinuteUploadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 云空间文件生成妙记请求。
#[derive(Debug, Clone)]
pub struct MinuteUploadRequest {
    config: Config,
}

impl MinuteUploadRequest {
    /// 创建请求。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self, body: MinuteUploadBody) -> SDKResult<MinuteUploadResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: MinuteUploadBody,
        option: RequestOption,
    ) -> SDKResult<MinuteUploadResponse> {
        validate_required!(body.file_token, "file_token 不能为空");
        let req: ApiRequest<MinuteUploadResponse> = MinutesExtraApiV1::Upload
            .to_request()
            .body(serialize_params(&body, "云空间文件生成妙记")?);
        Transport::request_typed(req, &self.config, Some(option), "云空间文件生成妙记").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::api_endpoints::CatalogEndpoint;
    use crate::common::test_utils::user_test_transport;
    use openlark_core::constants::AccessTokenType;
    use serde_json::json;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, ResponseTemplate};

    #[test]
    fn extra_catalog_upload_is_user_only_post() {
        let endpoint = MinutesExtraApiV1::Upload;
        assert_eq!(endpoint.to_url(), "/open-apis/minutes/v1/minutes/upload");
        let req: ApiRequest<()> = endpoint.to_request();
        assert_eq!(
            req.supported_access_token_types(),
            vec![AccessTokenType::User]
        );
    }

    #[tokio::test]
    async fn empty_file_token_fails_before_request() {
        let err = MinuteUploadRequest::new(Config::default())
            .execute(MinuteUploadBody {
                file_token: "   ".to_string(),
            })
            .await
            .expect_err("空白 file_token 应校验失败");
        assert!(err.to_string().contains("file_token"));
    }

    #[tokio::test]
    async fn upload_uses_user_token_and_typed_body() {
        let (server, config, option) = user_test_transport().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/minutes/v1/minutes/upload"))
            .and(header("Authorization", "Bearer test-user-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "minute_url": "https://sample.feishu.cn/minutes/upload-1" }
            })))
            .mount(&server)
            .await;

        let response = MinuteUploadRequest::new(config)
            .execute_with_options(
                MinuteUploadBody {
                    file_token: "doccnfYZzTlvXqZIGTdAHKabcef".to_string(),
                },
                option,
            )
            .await
            .expect("云空间文件生成妙记应成功");
        assert_eq!(
            response.minute_url.as_deref(),
            Some("https://sample.feishu.cn/minutes/upload-1")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为合法 JSON");
        assert_eq!(body["file_token"], "doccnfYZzTlvXqZIGTdAHKabcef");
    }
}
