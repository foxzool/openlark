//! 修改字段枚举值选项
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/common_data.meta_data/edit_enum_option>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 修改字段枚举值选项请求
#[derive(Debug, Clone)]
pub struct EditEnumOptionRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    request_body: Value,
}

impl EditEnumOptionRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: Value::Object(serde_json::Map::new()),
        }
    }

    /// 设置请求体
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EditEnumOptionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EditEnumOptionResponse> {
        use crate::common::api_endpoints::CorehrApiV1;

        let api_endpoint = CorehrApiV1::CommonDataMetaDataEditEnumOption;
        let endpoint_url = api_endpoint.to_url();
        validate_required!(endpoint_url.as_str(), "API 端点不能为空");

        let request =
            ApiRequest::<EditEnumOptionResponse>::post(endpoint_url).body(self.request_body);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "修改字段枚举值选项响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 修改字段枚举值选项响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditEnumOptionResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<EnumOptionInfo>,
}

/// 枚举值选项信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EnumOptionInfo {
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

impl ApiResponseTrait for EditEnumOptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/common_data/meta_data/edit_enum_option
    #[tokio::test]
    async fn test_edit_enum_option_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/corehr/v1/common_data/meta_data/edit_enum_option",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        EditEnumOptionRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
