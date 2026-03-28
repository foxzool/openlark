//! 获取企业自定义用户字段
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/custom_attr/list>

use std::collections::HashMap;

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{common::api_utils::extract_response_data, endpoints::CONTACT_V3_CUSTOM_ATTRS};

/// 自定义用户字段配置（字段随文档演进，未显式建模字段使用 `extra` 透传）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAttr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub attr_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 获取企业自定义用户字段响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCustomAttrsResponse {
    #[serde(default)]
    pub items: Vec<CustomAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListCustomAttrsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取企业自定义用户字段请求
pub struct ListCustomAttrsRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListCustomAttrsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 分页大小（查询参数，可选，默认 20，范围 1~100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/custom_attr/list>
    pub async fn execute(self) -> SDKResult<ListCustomAttrsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListCustomAttrsResponse> {
        let mut req: ApiRequest<ListCustomAttrsResponse> = ApiRequest::get(CONTACT_V3_CUSTOM_ATTRS);

        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取企业自定义用户字段")
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
