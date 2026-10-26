//! 批量删除填报行
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/report_detail_row/batchDelete>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

use super::batch_save::WorkforcePlanDetailRow;

/// 批量删除填报行请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchdeleteBody {
    /// 编制规划 id（必填）
    pub workforce_plan_id: String,
    /// 集中填报 id（必填）
    pub centralized_reporting_project_id: String,
    /// 填报行列表（必填，1～5 条）
    pub items: Vec<WorkforcePlanDetailRow>,
}

/// 批量删除填报行请求
#[derive(Debug, Clone)]
pub struct BatchdeleteRequest {
    /// 配置信息
    config: Config,
    body: BatchdeleteBody,
}

impl BatchdeleteRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: BatchdeleteBody::default(),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: BatchdeleteBody) -> Self {
        self.body = body;
        self
    }

    /// 设置编制规划 id（必填）。
    pub fn workforce_plan_id(mut self, workforce_plan_id: impl Into<String>) -> Self {
        self.body.workforce_plan_id = workforce_plan_id.into();
        self
    }

    /// 设置集中填报 id（必填）。
    pub fn centralized_reporting_project_id(
        mut self,
        centralized_reporting_project_id: impl Into<String>,
    ) -> Self {
        self.body.centralized_reporting_project_id = centralized_reporting_project_id.into();
        self
    }

    /// 设置填报行列表（必填，1～5 条）。
    pub fn items(mut self, items: Vec<WorkforcePlanDetailRow>) -> Self {
        self.body.items = items;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.body.workforce_plan_id, "workforce_plan_id 不能为空");
        validate_required!(
            self.body.centralized_reporting_project_id,
            "centralized_reporting_project_id 不能为空"
        );
        validate_required_list!(self.body.items, 5, "items 不能为空且单次最多 5 条");
        for item in &self.body.items {
            validate_required_list!(item.dimensions, 20, "dimensions 不能为空且单次最多 20 条");
            for dimension in &item.dimensions {
                validate_required!(dimension.dimension_key, "dimension_key 不能为空");
                validate_required!(dimension.dimension_value, "dimension_value 不能为空");
            }
        }

        let request = ApiRequest::<serde_json::Value>::post(
            "/open-apis/corehr/v2/report_detail_row/batchDelete",
        )
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feishu_people::corehr::v2::report_detail_row::batch_save::{
        DimensionEntity, WorkforcePlanEaiDetail,
    };
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/report_detail_row/batchDelete
    #[tokio::test]
    async fn test_batchdelete_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/report_detail_row/batchDelete"))
            .and(body_json(json!({
                "workforce_plan_id": "7435976673130317356",
                "centralized_reporting_project_id": "7436723164333753900",
                "items": [
                    {
                        "dimensions": [
                            { "dimension_key": "department", "dimension_value": "7371716294248908332" },
                            { "dimension_key": "location", "dimension_value": "7312702817660487212" }
                        ],
                        "plan_value": "919",
                        "eai_details": [
                            { "date": "2029-01-31", "estimated_active_individuals": "9" }
                        ]
                    }
                ]
            })))
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

        BatchdeleteRequest::new(config)
            .workforce_plan_id("7435976673130317356")
            .centralized_reporting_project_id("7436723164333753900")
            .items(vec![WorkforcePlanDetailRow {
                dimensions: vec![
                    DimensionEntity {
                        dimension_key: "department".to_string(),
                        dimension_value: "7371716294248908332".to_string(),
                    },
                    DimensionEntity {
                        dimension_key: "location".to_string(),
                        dimension_value: "7312702817660487212".to_string(),
                    },
                ],
                plan_value: Some("919".to_string()),
                eai_details: Some(vec![WorkforcePlanEaiDetail {
                    date: Some("2029-01-31".to_string()),
                    estimated_active_individuals: Some("9".to_string()),
                }]),
                multi_period_values: None,
            }])
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
