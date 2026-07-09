//! 查询批量任务执行状态
//!
//! docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uUDOwUjL1gDM14SN4ATN>

use std::collections::HashMap;

use openlark_core::{SDKResult, api::ApiRequest, config::Config, http::Transport};

use crate::{common::api_utils::extract_response_data, endpoints::CONTACT_V2_TASK_GET};

/// 查询批量任务执行状态请求
pub struct GetBatchTaskRequest {
    /// 配置信息。
    config: Config,
    /// 查询参数集合。
    query: HashMap<String, String>,
}

impl GetBatchTaskRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: HashMap::new(),
        }
    }

    /// 添加查询参数（例如 task_id）
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(key.into(), value.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uUDOwUjL1gDM14SN4ATN>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(CONTACT_V2_TASK_GET);
        for (k, v) in self.query {
            req = req.query(k, v);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询批量任务执行状态")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/contact/v2/task/get
    #[tokio::test]
    async fn test_get_batch_task_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/contact/v2/task/get"))
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

        GetBatchTaskRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
