//! 上传用户人脸图片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/acs-v1/user/face/update>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption, validate_required,
};

/// 上传用户人脸图片请求
///
/// 通过 body 传入人脸数据。本计划范围只做 Transport 迁移，body 用 `serde_json::Value`
/// 透传（字段细化见 spec §9，不在本次范围）。
#[derive(Debug)]
pub struct UpdateUserFaceRequest {
    /// 配置信息。
    config: Config,
    /// 用户 ID（路径参数，必填）。
    user_id: String,
    /// 请求体（必填，调用方自行构造 JSON）。
    body: Option<serde_json::Value>,
}

impl UpdateUserFaceRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, user_id: impl Into<String>) -> Self {
        Self {
            config,
            user_id: user_id.into(),
            body: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: serde_json::Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.user_id, "user_id 不能为空");
        let body = self
            .body
            .ok_or_else(|| validation_error("上传人脸图片", "body 不能为空"))?;

        let path = format!("/open-apis/acs/v1/users/{}/face", self.user_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::put(&path)
            .body(body)
            .with_supported_access_token_types(vec![AccessTokenType::Tenant]);

        Transport::request_typed(req, &self.config, Some(option), "上传用户人脸图片").await
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
    async fn test_update_user_face_rejects_empty_id() {
        let req = UpdateUserFaceRequest::new(test_config(), "");
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("user_id"));
    }

    /// 端到端：PUT .../users/{id}/face + body 透传 + 响应解析。
    #[tokio::test]
    async fn test_update_user_face_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/acs/v1/users/u_001/face"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "face_url": "https://cdn.example.com/face/u_001.jpg" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = UpdateUserFaceRequest::new(config, "u_001")
            .body(json!({ "image_base64": "BASE64_DATA" }))
            .execute()
            .await
            .expect("上传用户人脸图片应成功");
        assert_eq!(data["face_url"], "https://cdn.example.com/face/u_001.jpg");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/acs/v1/users/u_001/face");
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert!(sent["image_base64"].as_str().unwrap().contains("BASE64"));
    }
}
