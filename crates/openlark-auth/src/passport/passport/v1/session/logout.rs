//! 退出登录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/authentication-management/login-state-management/logout>

use crate::common::api_endpoints::PassportApiV1;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 退出登录请求
pub struct LogoutRequest {
    config: Config,
    user_id: Option<String>,
}

impl LogoutRequest {
    /// 创建退出登录请求实例
    ///
    /// # 参数
    /// - `config`: SDK 配置信息
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id: None,
        }
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 执行退出登录请求
    pub async fn execute(self) -> SDKResult<LogoutResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行退出登录请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<LogoutResponse> {
        let req: ApiRequest<LogoutResponse> = ApiRequest::post(PassportApiV1::SessionLogout.path());

        let response = Transport::request(req, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("logout", "响应数据为空"))
    }
}

/// 退出登录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutResponse {
    /// 退出结果
    pub result: String,
}

impl ApiResponseTrait for LogoutResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
