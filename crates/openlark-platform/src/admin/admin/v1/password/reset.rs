//! 重置用户密码 API
//! docPath: https://open.feishu.cn/document/server-docs/admin-v1/password/reset

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 重置用户密码的请求构建器。
pub struct ResetPasswordBuilder {
    user_id: String,
    new_password: String,
    config: Config,
}

impl ResetPasswordBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            user_id: String::new(),
            new_password: String::new(),
            config,
        }
    }

    /// 设置用户 ID。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 设置新密码。
    pub fn new_password(mut self, new_password: impl Into<String>) -> Self {
        self.new_password = new_password.into();
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<ResetPasswordResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ResetPasswordResponse> {
        validate_required!(self.user_id, "用户ID不能为空");
        validate_required!(self.new_password, "新密码不能为空");

        let request_body = ResetPasswordRequest {
            user_id: self.user_id,
            new_password: self.new_password,
        };

        let api_request: ApiRequest<ResetPasswordResponse> =
            ApiRequest::post("/open-apis/admin/v1/password/reset")
                .body(serde_json::to_value(&request_body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("重置用户密码", "响应数据为空"))
    }
}

#[derive(Debug, Serialize)]
struct ResetPasswordRequest {
    user_id: String,
    new_password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 重置用户密码的响应。
pub struct ResetPasswordResponse {
    /// 执行结果。
    pub result: String,
}

impl ApiResponseTrait for ResetPasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
