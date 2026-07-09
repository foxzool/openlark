//! 批量新增部门
//!
//! docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uMDOwUjLzgDM14yM4ATN>

use openlark_core::{SDKResult, api::ApiRequest, config::Config, http::Transport};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::CONTACT_V2_DEPARTMENT_BATCH_ADD,
};

/// 批量新增部门请求
pub struct BatchAddDepartmentsRequest {
    /// 配置信息。
    config: Config,
}

impl BatchAddDepartmentsRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uMDOwUjLzgDM14yM4ATN>
    pub async fn execute(self, params: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(params, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        params: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/contact/v2/department/batch_add
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(CONTACT_V2_DEPARTMENT_BATCH_ADD)
            .body(serialize_params(&params, "批量新增部门")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "批量新增部门")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/contact/v2/department/batch_add
    #[tokio::test]
    async fn test_batch_add_departments_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/contact/v2/department/batch_add"))
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

        let body = json!({});
        BatchAddDepartmentsRequest::new(config)
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
