//! 查询单个职务
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/job/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::models::CorehrV2Job;

/// GetRequest
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    /// 职务 ID
    job_id: Option<String>,
    /// 请求体（可选）
    body: Option<Value>,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            job_id: None,
            body: None,
        }
    }

    /// 设置职务 ID
    pub fn job_id(mut self, job_id: impl Into<String>) -> Self {
        self.job_id = Some(job_id.into());
        self
    }

    /// 设置请求体
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        let job_id = self.job_id.unwrap_or_default();
        validate_required!(job_id.trim(), "job_id 不能为空");

        let mut request =
            ApiRequest::<GetResponse>::get(format!("/open-apis/corehr/v2/jobs/{job_id}"));

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// GetResponse
///
/// 响应 `data` 子树形状：`{job}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 职务信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<CorehrV2Job>,
}

impl ApiResponseTrait for GetResponse {
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

    /// 端到端：GET /open-apis/corehr/v2/jobs/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/jobs/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "job": {
                        "id": "4698040628992333549",
                        "code": "JP422119",
                        "name": [{ "lang": "zh-CN", "value": "软件工程师" }],
                        "description": [{ "lang": "zh-CN", "value": "负责公司软件产品的开发与维护" }],
                        "active": true,
                        "job_title": [{ "lang": "zh-CN", "value": "高级" }],
                        "pathway_id": "4719519211875096301",
                        "job_family_id_list": ["4719519211875096301"],
                        "job_level_id_list": ["4719519212005299950"],
                        "working_hours_type_id": "6890452208593372679",
                        "effective_time": "2020-01-01 00:00:00",
                        "expiration_time": "2021-01-01 00:00:00",
                        "custom_fields": [
                            { "field_name": "name", "value": "Sandy" }
                        ]
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

        let resp = GetRequest::new(config)
            .job_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let job = resp.job.as_ref().expect("应包含职务信息");
        assert_eq!(job.id.as_deref(), Some("4698040628992333549"));
        assert_eq!(job.code.as_deref(), Some("JP422119"));
        assert_eq!(
            job.name
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("软件工程师")
        );
        assert_eq!(job.active, Some(true));
        assert_eq!(job.job_level_id_list.as_ref().map(|l| l.len()), Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
