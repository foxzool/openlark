//! 编辑离职信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/offboarding/edit>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 编辑离职信息请求
#[derive(Debug, Clone)]
pub struct EditRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl EditRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EditResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EditResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let api_endpoint = FeishuPeopleApiV2::OffboardingEdit;
        let mut request = ApiRequest::<EditResponse>::post(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "编辑离职信息响应数据为空",
        )
        .await
    }
}

/// 编辑离职信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditResponse {
    /// 编辑字段数据信息（官方 data.data 为 object_field_data[]）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ObjectFieldData>>,
}

/// 对象字段数据（官方 object_field_data 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObjectFieldData {
    /// 字段名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段值，是 json 转义后的字符串
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApiResponseTrait for EditResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/offboardings/edit
    #[tokio::test]
    async fn test_edit_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/offboardings/edit"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "data": [
                        { "field_name": "name", "value": "Sandy" }
                    ]
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = EditRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let fields = resp.data.expect("data 应解码");
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].field_name.as_deref(), Some("name"));
        assert_eq!(fields[0].value.as_deref(), Some("Sandy"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
