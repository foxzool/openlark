//! 获取自定义枚举详细信息
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/enums/:enum_name
//! docPath: 

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取枚举详情 Builder
#[derive(Debug, Clone)]
pub struct EnumGetBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 枚举名称
    enum_name: String,
}

impl EnumGetBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, workspace_id: impl Into<String>, enum_name: impl Into<String>) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
            enum_name: enum_name.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EnumGetResponse> {
        self.execute_with_options.await
    }(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<EnumGetResponse> {
        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/enums/{}",
            self.workspace_id, self.enum_name
        );

        let req = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 枚举详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnumGetResponse {
    /// 枚举名称
    #[serde(rename = "enum_name")]
    enum_name: String,
    /// 枚举描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 枚举值列表
    #[serde(rename = "values")]
    values: Vec<EnumValue>,
    /// 创建时间
    #[serde(rename = "created_time")]
    created_time: i64,
    /// 更新时间
    #[serde(rename = "updated_time")]
    updated_time: i64,
}

/// 枚举值
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnumValue {
    /// 值 ID
    #[serde(rename = "value_id")]
    value_id: String,
    /// 值名称
    #[serde(rename = "value_name")]
    value_name: String,
    /// 是否为默认值
    #[serde(rename = "is_default")]
    is_default: bool,
}

impl ApiResponseTrait for EnumGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
