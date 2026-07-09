//! 获取周期任务（全部用户）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/performance-v1/stage_task/find_by_page>

use openlark_core::validate_required;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取周期任务（全部用户）请求
#[derive(Debug, Clone)]
pub struct FindByPageRequest {
    /// 绩效周期 ID（必填）
    cycle_id: String,
    /// 分页大小（可选）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl FindByPageRequest {
    /// 创建请求
    pub fn new(config: Config, cycle_id: String) -> Self {
        Self {
            cycle_id,
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置分页大小（可选）
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
    pub async fn execute(self) -> SDKResult<FindByPageResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<FindByPageResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        validate_required!(self.cycle_id.trim(), "cycle_id");

        // 1. 构建端点
        let api_endpoint = PerformanceApiV1::StageTaskFindByPage;
        let mut request = ApiRequest::<FindByPageResponse>::post(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        request = request.query("cycle_id", &self.cycle_id);
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
                "获取周期任务响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取周期任务（全部用户）响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FindByPageResponse {
    /// 周期任务列表
    pub items: Vec<StageTask>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 周期任务
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StageTask {
    /// 用户 ID
    pub user_id: String,
    /// 阶段 ID
    pub stage_id: String,
    /// 任务状态
    pub status: i32,
}

impl ApiResponseTrait for FindByPageResponse {
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
    async fn test_performance_v1_stage_task_find_by_page_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"items": [], "has_more": false}"#).unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v1/stage_tasks/find_by_page"))
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

        let data = FindByPageRequest::new(config, "cycle_001".to_string())
            .execute()
            .await
            .expect("performance_v1_stage_task_find_by_page 应成功");

        assert!(data.items.is_empty());
        assert!(!data.has_more);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v1/stage_tasks/find_by_page"
        );
    }
}
