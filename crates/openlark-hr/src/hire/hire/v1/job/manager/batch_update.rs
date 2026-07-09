//! 更新职位相关人员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job.manager/batch_update>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hire::hire::common_models::JobManagerOperationResult;

/// 更新职位相关人员请求
#[derive(Debug, Clone)]
pub struct BatchUpdateRequest {
    job_id: Option<String>,
    request_body: BatchUpdateRequestBody,
    /// 配置信息
    config: Config,
}

impl BatchUpdateRequest {
    /// 创建请求
    pub fn new(config: Config, request_body: BatchUpdateRequestBody) -> Self {
        Self {
            job_id: None,
            request_body,
            config,
        }
    }

    /// 设置 `job_id`。
    pub fn job_id(mut self, job_id: impl Into<String>) -> Self {
        self.job_id = Some(job_id.into());
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: BatchUpdateRequestBody) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchUpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchUpdateResponse> {
        self.request_body.validate()?;
        let job_id = self.job_id.unwrap_or_default();
        validate_required!(job_id.trim(), "job_id 不能为空");

        let request = ApiRequest::<BatchUpdateResponse>::post(format!(
            "/open-apis/hire/v1/jobs/{job_id}/managers/batch_update"
        ));
        let request = request.body(serde_json::to_value(&self.request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新职位相关人员响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// `BatchUpdateRequestBody`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateRequestBody {
    #[serde(flatten)]
    /// `fields` 字段。
    pub fields: Value,
}

impl BatchUpdateRequestBody {
    /// 创建新的请求实例。
    pub fn new(fields: Value) -> Self {
        Self { fields }
    }

    fn validate(&self) -> SDKResult<()> {
        if self.fields.is_null() {
            return Err(openlark_core::error::validation_error(
                "更新职位相关人员请求体不能为空",
                "请传入有效的请求参数",
            ));
        }

        Ok(())
    }
}

/// 更新职位相关人员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchUpdateResponse {
    #[serde(flatten)]
    /// `operation` 字段。
    pub operation: JobManagerOperationResult,
}

impl ApiResponseTrait for BatchUpdateResponse {
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

    /// 端到端：POST /open-apis/hire/v1/jobs/test001/managers/batch_update
    #[tokio::test]
    async fn test_batch_update_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/hire/v1/jobs/test001/managers/batch_update",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "operation": {  } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        BatchUpdateRequest::new(
            config,
            serde_json::from_value::<BatchUpdateRequestBody>(json!({})).expect("body 构造"),
        )
        .job_id("test001".to_string())
        .execute()
        .await
        .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
