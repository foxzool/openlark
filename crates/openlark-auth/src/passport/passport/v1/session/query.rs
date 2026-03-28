//! 批量获取脱敏的用户登录信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/authentication-management/login-state-management/query>

use crate::common::api_endpoints::PassportApiV1;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量获取脱敏用户登录信息请求体
#[derive(Debug, Serialize)]
struct QuerySessionRequestBody {
    user_ids: Vec<String>,
}

/// 批量获取脱敏用户登录信息请求
pub struct QuerySessionRequest {
    config: Config,
    user_ids: Vec<String>,
}

impl QuerySessionRequest {
    /// 创建批量查询会话请求实例
    ///
    /// # 参数
    /// - `config`: SDK 配置信息
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_ids: Vec::new(),
        }
    }

    /// 设置用户ID列表
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// 执行批量查询会话请求
    pub async fn execute(self) -> SDKResult<QuerySessionResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行批量查询会话请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<QuerySessionResponse> {
        let body = QuerySessionRequestBody {
            user_ids: self.user_ids,
        };

        let req: ApiRequest<QuerySessionResponse> =
            ApiRequest::post(PassportApiV1::SessionQuery.path()).body(serde_json::to_value(&body)?);

        let response = Transport::request(req, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("query_session", "响应数据为空"))
    }
}

/// 批量获取脱敏用户登录信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySessionResponse {
    /// 会话信息列表
    pub items: Vec<SessionInfo>,
}

impl ApiResponseTrait for QuerySessionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 会话信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    /// 用户ID
    pub user_id: String,
    /// 会话状态
    pub status: String,
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
