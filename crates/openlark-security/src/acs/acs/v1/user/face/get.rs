//! 下载用户人脸图片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/acs-v1/user/face/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 下载用户人脸图片请求
#[derive(Debug)]
pub struct GetUserFaceRequest {
    /// 配置信息。
    config: Config,
    /// 用户 ID（路径参数，必填）。
    user_id: String,
}

/// 人脸图片数据（响应 `data` 字段内容）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceData {
    /// 人脸图片 URL。
    pub face_url: String,
}

impl ApiResponseTrait for FaceData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetUserFaceRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, user_id: impl Into<String>) -> Self {
        Self {
            config,
            user_id: user_id.into(),
        }
    }

    /// 执行请求，返回人脸图片数据。
    pub async fn execute(self) -> SDKResult<FaceData> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<FaceData> {
        validate_required!(self.user_id, "user_id 不能为空");

        let path = format!("/open-apis/acs/v1/users/{}/face", self.user_id);
        let req: ApiRequest<FaceData> =
            ApiRequest::get(&path).with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("下载用户人脸图片", "响应数据为空")
        })
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
}
