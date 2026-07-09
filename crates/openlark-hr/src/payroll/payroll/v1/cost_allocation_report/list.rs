//! 查询成本分摊报表汇总数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/cost_allocation_report/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 查询成本分摊报表汇总数据请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 发薪活动 ID（可选）
    activity_id: Option<String>,
    /// 成本分摊方案 ID（可选）
    plan_id: Option<String>,
    /// 分页大小（可选，默认 50，最大 100）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            activity_id: None,
            plan_id: None,
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置发薪活动 ID（可选）
    pub fn activity_id(mut self, activity_id: String) -> Self {
        self.activity_id = Some(activity_id);
        self
    }

    /// 设置成本分摊方案 ID（可选）
    pub fn plan_id(mut self, plan_id: String) -> Self {
        self.plan_id = Some(plan_id);
        self
    }

    /// 设置分页大小（可选，默认 50，最大 100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::PayrollApiV1;

        // 1. 构建端点
        let api_endpoint = PayrollApiV1::CostAllocationReportList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(ref activity_id) = self.activity_id {
            request = request.query("activity_id", activity_id);
        }
        if let Some(ref plan_id) = self.plan_id {
            request = request.query("plan_id", plan_id);
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询成本分摊报表汇总数据响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询成本分摊报表汇总数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 成本分摊报表汇总列表
    pub items: Vec<CostAllocationReport>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 成本分摊报表汇总信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CostAllocationReport {
    /// 报表 ID
    pub report_id: String,
    /// 发薪活动 ID
    pub activity_id: String,
    /// 成本分摊方案 ID
    pub plan_id: String,
    /// 成本中心 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 成本中心名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center_name: Option<String>,
    /// 分摊总人数
    pub employee_count: i32,
    /// 分摊总金额
    pub total_amount: f64,
    /// 币种
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// 统计时间（Unix 时间戳）
    pub stats_time: i64,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_list_cost_allocation_reports_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"items": [], "has_more": false}"#).unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/cost_allocation_reports"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = ListRequest::new(config)
            .execute()
            .await
            .expect("查询成本分摊报告应成功");

        assert!(!data.has_more);
        assert!(data.items.is_empty());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/payroll/v1/cost_allocation_reports"
        );
    }
}
