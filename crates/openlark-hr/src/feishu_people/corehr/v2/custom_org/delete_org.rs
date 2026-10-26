//! 删除自定义组织
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/custom_org/delete_org>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 删除自定义组织请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteOrgBody {
    /// 自定义组织 ID（必填，可从批量查询自定义组织的 org_id 字段中获取）
    pub org_id: String,
    /// 组织类型编码（必填，可在「飞书人事-设置-组织设置」中相应的自定义组织目录下查看）
    pub object_api_name: String,
}

/// 删除自定义组织请求
#[derive(Debug, Clone)]
pub struct DeleteOrgRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: DeleteOrgBody,
}

impl DeleteOrgRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: DeleteOrgBody::default(),
        }
    }

    /// 设置请求体
    pub fn body(mut self, body: DeleteOrgBody) -> Self {
        self.body = body;
        self
    }

    /// 设置自定义组织 ID（必填）。
    pub fn org_id(mut self, org_id: impl Into<String>) -> Self {
        self.body.org_id = org_id.into();
        self
    }

    /// 设置组织类型编码（必填）。
    pub fn object_api_name(mut self, object_api_name: impl Into<String>) -> Self {
        self.body.object_api_name = object_api_name.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteOrgResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteOrgResponse> {
        validate_required!(self.body.org_id, "org_id 不能为空");
        validate_required!(self.body.object_api_name, "object_api_name 不能为空");

        let request =
            ApiRequest::<DeleteOrgResponse>::post("/open-apis/corehr/v2/custom_orgs/delete_org")
                .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 删除自定义组织响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteOrgResponse {
    /// 响应数据
    pub data: Value,
}

impl ApiResponseTrait for DeleteOrgResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/custom_orgs/delete_org
    #[tokio::test]
    async fn test_delete_org_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/custom_orgs/delete_org"))
            .and(body_json(json!({
                "org_id": "6862995757234914824",
                "object_api_name": "custom_org_01"
            })))
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

        DeleteOrgRequest::new(config)
            .org_id("6862995757234914824")
            .object_api_name("custom_org_01")
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
