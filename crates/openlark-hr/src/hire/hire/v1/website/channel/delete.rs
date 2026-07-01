//! 删除招聘官网推广渠道
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/website.channel/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 删除招聘官网推广渠道请求
#[derive(Debug, Clone)]
pub struct DeleteRequest {
    /// 配置信息
    config: Config,
    website_id: String,
    channel_id: String,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            website_id: String::new(),
            channel_id: String::new(),
        }
    }

    /// 设置 `website_id`。
    pub fn website_id(mut self, website_id: String) -> Self {
        self.website_id = website_id;
        self
    }

    /// 设置 `channel_id`。
    pub fn channel_id(mut self, channel_id: String) -> Self {
        self.channel_id = channel_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResponse> {
        validate_required!(self.website_id.trim(), "官网 ID 不能为空");
        validate_required!(self.channel_id.trim(), "渠道 ID 不能为空");

        let request = ApiRequest::<DeleteResponse>::delete(format!(
            "/open-apis/hire/v1/websites/{}/channels/{}",
            self.website_id, self.channel_id
        ));
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "删除招聘官网推广渠道响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 删除招聘官网推广渠道响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `channel_id` 字段。
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for DeleteResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
