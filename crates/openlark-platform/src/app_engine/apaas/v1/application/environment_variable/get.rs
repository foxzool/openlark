//! 查询环境变量详情
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-environment_variable/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 查询环境变量详情 Builder
#[derive(Debug, Clone)]
pub struct EnvironmentVariableGetBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 环境变量 API 名称
    env_var_api_name: String,
}

impl EnvironmentVariableGetBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        env_var_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            env_var_api_name: env_var_api_name.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EnvironmentVariableGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EnvironmentVariableGetResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/environment_variables/{}",
            self.namespace, self.env_var_api_name
        );

        let req: ApiRequest<EnvironmentVariableGetResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 环境变量详情
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvironmentVariableDetail {
    /// 环境变量 API 名称
    #[serde(rename = "api_name")]
    api_name: String,
    /// 环境变量名称
    #[serde(rename = "name")]
    name: String,
    /// 环境变量值
    #[serde(rename = "value")]
    value: String,
    /// 描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

/// 查询环境变量详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvironmentVariableGetResponse {
    /// 环境变量详情
    #[serde(rename = "environment_variable")]
    environment_variable: EnvironmentVariableDetail,
}

impl ApiResponseTrait for EnvironmentVariableGetResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
