//! 删除成本分摊
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/cost_allocation/remove_version>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 删除成本分摊请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveVersionBody {
    /// 员工雇佣 ID，可通过【搜索员工信息】接口获取
    pub employment_id: String,
    /// 成本分摊（删除时内部 `wk_id` 必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_allocation: Option<CostAllocationForRemove>,
}

/// 成本分摊（官方 employment_cost_allocation，删除场景，仅 `wk_id` 必填）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CostAllocationForRemove {
    /// 成本分摊 ID，可通过【查询成本分摊】接口获取
    pub wk_id: String,
}

/// 删除成本分摊请求
#[derive(Debug, Clone)]
pub struct RemoveVersionRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: RemoveVersionBody,
    /// 用于幂等控制的 client_token
    client_token: Option<String>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl RemoveVersionRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: RemoveVersionBody::default(),
            client_token: None,
            user_id_type: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: RemoveVersionBody) -> Self {
        self.body = body;
        self
    }

    /// 设置员工雇佣 ID。
    pub fn employment_id(mut self, employment_id: impl Into<String>) -> Self {
        self.body.employment_id = employment_id.into();
        self
    }

    /// 设置成本分摊（删除时 `wk_id` 必填）。
    pub fn cost_allocation(mut self, cost_allocation: CostAllocationForRemove) -> Self {
        self.body.cost_allocation = Some(cost_allocation);
        self
    }

    /// 设置用于幂等控制的 client_token。
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id/people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RemoveVersionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RemoveVersionResponse> {
        validate_required!(self.body.employment_id, "employment_id 不能为空");
        if let Some(cost_allocation) = &self.body.cost_allocation {
            validate_required!(cost_allocation.wk_id, "cost_allocation.wk_id 不能为空");
        }

        let request = ApiRequest::<RemoveVersionResponse>::post(
            "/open-apis/corehr/v2/cost_allocations/remove_version",
        )
        .query_opt("client_token", self.client_token)
        .query_opt("user_id_type", self.user_id_type)
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 删除成本分摊响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RemoveVersionResponse {
    /// 响应数据
    pub data: Value,
}

impl ApiResponseTrait for RemoveVersionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/cost_allocations/remove_version
    #[tokio::test]
    async fn test_remove_version_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/cost_allocations/remove_version"))
            .and(query_param("user_id_type", "open_id"))
            .and(body_json(json!({
                "employment_id": "7862995757234914824",
                "cost_allocation": {
                    "wk_id": "703912325303191204"
                }
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

        RemoveVersionRequest::new(config)
            .employment_id("7862995757234914824")
            .cost_allocation(CostAllocationForRemove {
                wk_id: "703912325303191204".to_string(),
            })
            .user_id_type("open_id")
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
