//! 获取人脸信息
//!
//! docPath: https://open.feishu.cn/document/acs-v1/face/get

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption, validate_required,
};

/// 获取人脸信息请求
#[derive(Debug)]
pub struct FaceGetRequest {
    /// 配置信息。
    config: Config,
    /// 人脸 ID（路径参数，必填）。
    face_id: String,
}

impl FaceGetRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, face_id: impl Into<String>) -> Self {
        Self {
            config,
            face_id: face_id.into(),
        }
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.face_id, "face_id 不能为空");

        let path = format!("/open-apis/acs/v1/faces/{}", self.face_id);
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get(&path).with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("获取人脸信息", "响应数据为空"))
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
    async fn test_face_get_rejects_empty_id() {
        let req = FaceGetRequest::new(test_config(), "");
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("face_id"));
    }
}
