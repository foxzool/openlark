//! 创建成本分摊
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/cost_allocation/create_version>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 创建成本分摊请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateVersionBody {
    /// 员工雇佣 ID，可通过【搜索员工信息】接口获取
    pub employment_id: String,
    /// 成本分摊（创建时内部 `effective_time` 必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_allocation: Option<CostAllocationForCreate>,
}

/// 成本分摊（官方 employment_cost_allocation，创建场景，`effective_time` 必填）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CostAllocationForCreate {
    /// 分摊生效日期，格式 "YYYY-MM-DD"
    pub effective_time: String,
    /// 分摊失效日期，格式 "YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 成本分摊明细（1～50 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_data_cost_center_id: Option<Vec<JobDataCostCenterForCreate>>,
    /// 变更原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// 成本分摊明细（官方 job_data_cost_center，创建场景 `cost_center_id`/`new_rate` 必填）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobDataCostCenterForCreate {
    /// 成本中心 ID，可通过【搜索成本中心信息】接口获取
    pub cost_center_id: String,
    /// 分摊比例
    pub new_rate: f64,
}

/// 创建成本分摊请求
#[derive(Debug, Clone)]
pub struct CreateVersionRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: CreateVersionBody,
    /// 用于幂等控制的 client_token
    client_token: Option<String>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl CreateVersionRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: CreateVersionBody::default(),
            client_token: None,
            user_id_type: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: CreateVersionBody) -> Self {
        self.body = body;
        self
    }

    /// 设置员工雇佣 ID。
    pub fn employment_id(mut self, employment_id: impl Into<String>) -> Self {
        self.body.employment_id = employment_id.into();
        self
    }

    /// 设置成本分摊（创建时 `effective_time` 必填）。
    pub fn cost_allocation(mut self, cost_allocation: CostAllocationForCreate) -> Self {
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
    pub async fn execute(self) -> SDKResult<CreateVersionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateVersionResponse> {
        validate_required!(self.body.employment_id, "employment_id 不能为空");
        if let Some(cost_allocation) = &self.body.cost_allocation {
            validate_required!(
                cost_allocation.effective_time,
                "cost_allocation.effective_time 不能为空"
            );
        }

        let request = ApiRequest::<CreateVersionResponse>::post(
            "/open-apis/corehr/v2/cost_allocations/create_version",
        )
        .query_opt("client_token", self.client_token)
        .query_opt("user_id_type", self.user_id_type)
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 创建成本分摊响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateVersionResponse {
    /// 成本分摊ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_allocation_id: Option<String>,
}

impl ApiResponseTrait for CreateVersionResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/cost_allocations/create_version
    #[tokio::test]
    async fn test_create_version_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/cost_allocations/create_version"))
            .and(query_param("user_id_type", "open_id"))
            .and(body_json(json!({
                "employment_id": "6862995757234914821",
                "cost_allocation": {
                    "effective_time": "2024-12-01",
                    "expiration_time": "2024-12-02",
                    "job_data_cost_center_id": [
                        {
                            "cost_center_id": "6950635856373745165",
                            "new_rate": 50.2
                        }
                    ],
                    "reason": "异动"
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "cost_allocation_id": "12345678" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CreateVersionRequest::new(config)
            .employment_id("6862995757234914821")
            .cost_allocation(CostAllocationForCreate {
                effective_time: "2024-12-01".to_string(),
                expiration_time: Some("2024-12-02".to_string()),
                job_data_cost_center_id: Some(vec![JobDataCostCenterForCreate {
                    cost_center_id: "6950635856373745165".to_string(),
                    new_rate: 50.2,
                }]),
                reason: Some("异动".to_string()),
            })
            .user_id_type("open_id")
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.cost_allocation_id, Some("12345678".to_string()));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
