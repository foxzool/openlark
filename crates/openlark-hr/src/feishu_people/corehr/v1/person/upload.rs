//! 上传文件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/person/upload>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 上传文件请求
#[derive(Debug, Clone)]
pub struct UploadRequest {
    config: Config,
    file: Option<Value>,
}

impl UploadRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, file: None }
    }

    /// 设置 `file`。
    pub fn file(mut self, file: Value) -> Self {
        self.file = Some(file);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UploadResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UploadResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::PersonUpload;
        let mut request = ApiRequest::<UploadResponse>::post(api_endpoint.to_url());
        if let Some(file) = self.file {
            request = request.body(serde_json::json!({ "file": file }));
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "上传文件响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 上传文件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UploadResponse {
    /// 原始响应数据。
    pub data: Value,
}

impl ApiResponseTrait for UploadResponse {
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

    /// 端到端：POST /open-apis/corehr/v1/persons/upload
    #[tokio::test]
    async fn test_upload_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/persons/upload"))
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

        UploadRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
