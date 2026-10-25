//! 创建成本中心版本
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/cost_center.version/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::feishu_people::corehr::v2::cost_center::CostCenterVersion;

/// 创建成本中心版本请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    cost_center_id: Option<String>,
    body: Option<Value>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            cost_center_id: None,
            body: None,
        }
    }

    /// 设置 `cost_center_id`。
    pub fn cost_center_id(mut self, cost_center_id: impl Into<String>) -> Self {
        self.cost_center_id = Some(cost_center_id.into());
        self
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        let cost_center_id = self.cost_center_id.unwrap_or_default();
        validate_required!(cost_center_id.trim(), "cost_center_id 不能为空");

        let mut request = ApiRequest::<CreateResponse>::post(format!(
            "/open-apis/corehr/v2/cost_centers/{cost_center_id}/versions"
        ));

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 创建成本中心版本响应
///
/// 响应 `data` 子树形状：`{version: {...}}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 成本中心版本信息（官方 `cost_center.version` 结构）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<CostCenterVersion>,
}

impl ApiResponseTrait for CreateResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/cost_centers/test001/versions
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/cost_centers/test001/versions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "version": {
                        "cost_center_id": "6969828847121885087",
                        "version_id": "6969828847121885087",
                        "name": [
                            { "lang": "zh-CN", "value": "中文示例" }
                        ],
                        "code": "MDPD00000023",
                        "parent_cost_center_id": "6862995757234914824",
                        "managers": ["6862995757234914824"],
                        "description": [
                            { "lang": "zh-CN", "value": "中文示例" }
                        ],
                        "effective_time": "2020-01-01",
                        "expiration_time": "2020-01-01",
                        "active": true
                    }
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

        let resp = CreateRequest::new(config)
            .cost_center_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let version = resp.version.as_ref().expect("version 应存在");
        assert_eq!(version.version_id.as_deref(), Some("6969828847121885087"));
        assert_eq!(
            version.cost_center_id.as_deref(),
            Some("6969828847121885087")
        );
        assert_eq!(
            version.parent_cost_center_id.as_deref(),
            Some("6862995757234914824")
        );
        assert_eq!(
            version.managers.as_deref(),
            Some(&["6862995757234914824".to_string()][..])
        );
        assert_eq!(version.active, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
