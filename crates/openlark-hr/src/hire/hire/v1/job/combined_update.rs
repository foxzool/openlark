//! 更新职位
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job/combined_update>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 更新职位请求
#[derive(Debug, Clone)]
pub struct CombinedUpdateRequest {
    job_id: Option<String>,
    request_body: CombinedUpdateRequestBody,
    /// 配置信息
    config: Config,
}

impl CombinedUpdateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            job_id: None,
            request_body: CombinedUpdateRequestBody::default(),
            config,
        }
    }

    /// 设置 `job_id`。
    pub fn job_id(mut self, job_id: impl Into<String>) -> Self {
        self.job_id = Some(job_id.into());
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: CombinedUpdateRequestBody) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CombinedUpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CombinedUpdateResponse> {
        self.request_body.validate()?;
        let job_id = self.job_id.unwrap_or_default();
        validate_required!(job_id.trim(), "job_id 不能为空");

        let request = ApiRequest::<CombinedUpdateResponse>::post(format!(
            "/open-apis/hire/v1/jobs/{job_id}/combined_update"
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
                "更新职位响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// `CombinedUpdateRequestBody`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CombinedUpdateRequestBody {
    #[serde(flatten)]
    /// `fields` 字段。
    pub fields: Value,
}

impl CombinedUpdateRequestBody {
    /// 创建新的请求实例。
    pub fn new(fields: Value) -> Self {
        Self { fields }
    }

    fn validate(&self) -> SDKResult<()> {
        if self.fields.is_null() {
            return Err(openlark_core::error::validation_error(
                "更新职位请求体不能为空",
                "请传入有效的请求参数",
            ));
        }

        Ok(())
    }
}

/// 更新职位响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CombinedUpdateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for CombinedUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_combined_update_request_builder_new() {
        let request = CombinedUpdateRequest::new(TestConfigBuilder::new().build());
        let _ = request;
    }
}
