//! 查询成本分摊
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/cost_allocation/batch_query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询成本分摊请求
#[derive(Debug, Clone)]
pub struct BatchQueryRequest {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl BatchQueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchQueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchQueryResponse> {
        let mut request = ApiRequest::<BatchQueryResponse>::post(
            "/open-apis/corehr/v2/cost_allocations/batch_query",
        );

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 查询成本分摊响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchQueryResponse {
    /// 查询到的成本分摊信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<EmployeeCostAllocation>>,
}

/// 员工成本分摊（employee_cost_allocation）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeeCostAllocation {
    /// 员工ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 成本分摊信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_allocations: Option<Vec<EmploymentCostAllocation>>,
}

/// 成本分摊信息（employment_cost_allocation）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmploymentCostAllocation {
    /// 成本分摊ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    /// 分摊生效日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 分摊失效日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 成本分摊
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_cost_center_id: Option<Vec<JobDataCostCenter>>,
    /// 任职ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_id: Option<JobDataId>,
    /// 变更原因
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// 成本分摊明细（job_data_cost_center）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobDataCostCenter {
    /// 成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 分摊比例（整数）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<i64>,
    /// 分摊比例
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_rate: Option<f64>,
}

/// 任职ID（job_data_id）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobDataId {
    /// 记录ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
}

impl ApiResponseTrait for BatchQueryResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/cost_allocations/batch_query
    #[tokio::test]
    async fn test_batch_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/cost_allocations/batch_query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "employment_id": "7475686493613920050",
                            "cost_allocations": [
                                {
                                    "wk_id": "703912325303191204",
                                    "effective_time": "2024-12-01",
                                    "expiration_time": "2024-12-02",
                                    "job_data_cost_center_id": [
                                        {
                                            "cost_center_id": "6950635856373745165",
                                            "rate": 100,
                                            "new_rate": 50.2
                                        }
                                    ],
                                    "job_data_id": { "wk_id": "7305354116229514796" },
                                    "reason": "异动"
                                }
                            ]
                        }
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

        let resp = BatchQueryRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应存在");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].employment_id,
            Some("7475686493613920050".to_string())
        );
        let allocations = items[0]
            .cost_allocations
            .as_ref()
            .expect("cost_allocations 应存在");
        assert_eq!(allocations.len(), 1);
        assert_eq!(
            allocations[0].effective_time,
            Some("2024-12-01".to_string())
        );
        let cost_centers = allocations[0]
            .job_data_cost_center_id
            .as_ref()
            .expect("job_data_cost_center_id 应存在");
        assert_eq!(cost_centers[0].rate, Some(100));
        assert_eq!(cost_centers[0].new_rate, Some(50.2));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
