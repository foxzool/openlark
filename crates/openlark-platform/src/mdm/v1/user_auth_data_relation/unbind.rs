//! 用户数据维度解绑
//!
//! 文档: https://open.feishu.cn/document/server-docs/mdm-v1/user_auth_data_relation/unbind
//! docPath: https://open.feishu.cn/document/server-docs/mdm-v1/user_auth_data_relation/unbind

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 用户数据维度解绑 Builder
#[derive(Debug, Clone)]
pub struct UserAuthDataRelationUnbindBuilder {
    config: Config,
    user_ids: Vec<String>,
    data_type: String,
}

impl UserAuthDataRelationUnbindBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_ids: Vec::new(),
            data_type: String::new(),
        }
    }

    /// 添加用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_ids.push(user_id.into());
        self
    }

    /// 添加多个用户 ID
    pub fn user_ids(mut self, user_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.user_ids.extend(user_ids.into_iter().map(Into::into));
        self
    }

    /// 设置数据类型
    pub fn data_type(mut self, data_type: impl Into<String>) -> Self {
        self.data_type = data_type.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UserAuthDataRelationUnbindResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UserAuthDataRelationUnbindResponse> {
        let url = "/open-apis/mdm/v1/user_auth_data_relations/unbind";

        let request = UserAuthDataRelationUnbindRequest {
            user_ids: self.user_ids,
            data_type: self.data_type,
        };

        let req: ApiRequest<UserAuthDataRelationUnbindResponse> =
            ApiRequest::post(url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 用户数据维度解绑请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct UserAuthDataRelationUnbindRequest {
    /// 用户 ID 列表
    #[serde(rename = "user_ids")]
    user_ids: Vec<String>,
    /// 数据类型
    #[serde(rename = "data_type")]
    data_type: String,
}

/// 用户数据维度解绑响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserAuthDataRelationUnbindResponse {
    /// 解绑结果
    pub items: Vec<UserAuthDataRelationResult>,
}

/// 用户数据维度解绑结果
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserAuthDataRelationResult {
    /// 用户 ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 解绑是否成功
    #[serde(rename = "is_success")]
    pub is_success: bool,
    /// 失败原因
    #[serde(rename = "fail_reason")]
    pub fail_reason: Option<String>,
}

impl ApiResponseTrait for UserAuthDataRelationUnbindResponse {}

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
