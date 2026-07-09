//! 更新待入职信息（不推荐）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/pre_hire/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新待入职信息（不推荐）请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    config: Config,
    pre_hire_id: String,
    pre_hire: Option<Value>,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            pre_hire_id: String::new(),
            pre_hire: None,
        }
    }

    /// 设置 `pre_hire_id`。
    pub fn pre_hire_id(mut self, pre_hire_id: String) -> Self {
        self.pre_hire_id = pre_hire_id;
        self
    }

    /// 设置 `pre_hire`。
    pub fn pre_hire(mut self, pre_hire: Value) -> Self {
        self.pre_hire = Some(pre_hire);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        validate_required!(self.pre_hire_id.trim(), "待入职ID不能为空");

        let api_endpoint = FeishuPeopleApiV1::PreHirePatch(self.pre_hire_id);
        let mut request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url());
        if let Some(pre_hire) = self.pre_hire {
            request = request.body(serde_json::json!({ "pre_hire": pre_hire }));
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新待入职信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 更新待入职信息（不推荐）响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 原始响应数据。
    pub data: Value,
}

impl ApiResponseTrait for PatchResponse {
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

    /// 端到端：PATCH /open-apis/corehr/v1/pre_hires/test001
    #[tokio::test]
    async fn test_patch_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/corehr/v1/pre_hires/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "data": {} }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        PatchRequest::new(config)
            .pre_hire_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
