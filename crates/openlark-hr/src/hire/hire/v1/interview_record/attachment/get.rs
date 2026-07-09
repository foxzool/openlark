//! 获取面试记录附件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/interview_record.attachment/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// `GetRequest` 请求。
#[derive(Debug, Clone)]
pub struct GetRequest {
    config: Config,
    application_id: String,
    interview_record_id: Option<String>,
    language: Option<i32>,
}

impl GetRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            application_id: String::new(),
            interview_record_id: None,
            language: None,
        }
    }

    /// 设置 `application_id`。
    pub fn application_id(mut self, application_id: impl Into<String>) -> Self {
        self.application_id = application_id.into();
        self
    }

    /// 设置 `interview_record_id`。
    pub fn interview_record_id(mut self, interview_record_id: impl Into<String>) -> Self {
        self.interview_record_id = Some(interview_record_id.into());
        self
    }

    /// 设置 `language`。
    pub fn language(mut self, language: i32) -> Self {
        self.language = Some(language);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        if self.application_id.trim().is_empty() {
            return Err(error::validation_error(
                "application_id",
                "application_id 不能为空",
            ));
        }

        let mut request =
            ApiRequest::<GetResponse>::get("/open-apis/hire/v1/interview_records/attachments");
        request = request.query("application_id", self.application_id);
        if let Some(interview_record_id) = self.interview_record_id {
            request = request.query("interview_record_id", interview_record_id);
        }
        if let Some(language) = self.language {
            request = request.query("language", language.to_string());
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error("获取面试记录附件响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// `InterviewRecordAttachment`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewRecordAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `url` 字段。
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `mime` 字段。
    pub mime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `create_time` 字段。
    pub create_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `GetResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `attachment` 字段。
    pub attachment: Option<InterviewRecordAttachment>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
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

    /// 端到端：GET /open-apis/hire/v1/interview_records/attachments
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/interview_records/attachments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        GetRequest::new(config)
            .application_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
