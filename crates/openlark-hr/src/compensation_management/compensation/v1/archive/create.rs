//! 创建薪资档案
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/archive/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 创建薪资档案请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::CompensationApiV1;

        let api_endpoint = CompensationApiV1::ArchiveCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建薪资档案响应数据为空",
        )
        .await
    }
}

/// 创建薪资档案响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 定调薪任务创建的唯一 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    /// 薪资档案的 TID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_tid: Option<String>,
}

impl ApiResponseTrait for CreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    /// 官方成功响应可能无 `data`；全 Option 字段时声明空成功（非 `{}` 探测）。
    fn empty_success() -> Option<Self> {
        Some(Self {
            unique_id: None,
            archive_tid: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_create_archive_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data 含 unique_id + archive_tid
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"unique_id": "123e4567-e89b-42d3-a456-426614174000", "archive_tid": "7434007780111336970"}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/archives"))
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

        let data = CreateRequest::new(config)
            .execute()
            .await
            .expect("创建薪资档案应成功");

        assert_eq!(
            data.unique_id.as_deref(),
            Some("123e4567-e89b-42d3-a456-426614174000")
        );
        assert_eq!(data.archive_tid.as_deref(), Some("7434007780111336970"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/compensation/v1/archives"
        );
    }
}
