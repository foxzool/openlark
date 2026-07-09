//! 获取企业自定义用户字段
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/custom_attr/list>

use std::collections::HashMap;

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

use crate::{common::api_utils::extract_response_data, endpoints::CONTACT_V3_CUSTOM_ATTRS};

/// 自定义用户字段配置（字段随文档演进，未显式建模字段使用 `extra` 透传）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAttr {
    /// 字段 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 字段类型。
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub attr_type: Option<String>,
    /// 字段选项。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 获取企业自定义用户字段响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCustomAttrsResponse {
    /// 自定义字段列表。
    #[serde(default)]
    pub items: Vec<CustomAttr>,
    /// 分页标记。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据。
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
    /// 配置信息。
    config: Config,
    /// 分页大小。
    page_size: Option<i32>,
    /// 分页标记。
    page_token: Option<String>,
}

impl ListCustomAttrsRequest {
    /// 创建新的请求构建器。
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

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
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
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/contact/v3/custom_attrs
    #[tokio::test]
    async fn test_list_custom_attrs_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/contact/v3/custom_attrs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        ListCustomAttrsRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
