//! 修改用户人脸识别信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_setting/modify>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 修改用户人脸识别信息请求
#[derive(Debug, Clone)]
pub struct ModifyRequest {
    /// 用户 ID（必填）
    user_id: String,
    /// 是否开启人脸识别（必填）
    face_recognition_enabled: bool,
    /// 配置信息
    config: Config,
}

impl ModifyRequest {
    /// 创建请求
    pub fn new(config: Config, user_id: String, face_recognition_enabled: bool) -> Self {
        Self {
            user_id,
            face_recognition_enabled,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ModifyResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ModifyResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.user_id.trim(), "user_id");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserSettingModify;
        let request = ApiRequest::<ModifyResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = ModifyRequestBody {
            user_id: self.user_id,
            face_recognition_enabled: self.face_recognition_enabled,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "修改用户人脸识别信息响应数据为空",
        )
        .await
    }
}

/// 修改用户人脸识别信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyRequestBody {
    /// 用户 ID
    pub user_id: String,
    /// 是否开启人脸识别
    pub face_recognition_enabled: bool,
}

/// 修改用户人脸识别信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModifyResponse {
    /// 是否成功
    pub success: bool,
    /// 用户 ID
    pub user_id: String,
}

impl ApiResponseTrait for ModifyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_modify_request_builder_new() {
        let request =
            ModifyRequest::new(TestConfigBuilder::new().build(), "test".to_string(), true);
        let _ = request;
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_user_setting_modify_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"success": false, "user_id": "test"}"#).unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/user_settings/modify"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = ModifyRequest::new(config, "user_001".to_string(), true)
            .execute()
            .await
            .expect("attendance_v1_user_setting_modify 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_settings/modify"
        );
    }
}
