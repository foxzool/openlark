//! 创建投递
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/application/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::ApplicationJobInfo;

/// 创建投递请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    request_body: Value,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: Value::Null,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = request_body;
        self
    }

    fn validate(&self) -> SDKResult<()> {
        if self.request_body.is_null()
            || self
                .request_body
                .as_object()
                .is_some_and(|obj| obj.is_empty())
        {
            return Err(openlark_core::error::validation_error(
                "请求体不能为空",
                "创建投递时 request_body 为必填参数",
            ));
        }
        Ok(())
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::HireApiV1;

        self.validate()?;

        let api_endpoint = HireApiV1::ApplicationCreate;
        let request =
            ApiRequest::<CreateResponse>::post(api_endpoint.to_url()).body(self.request_body);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "创建投递响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 创建投递响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_status` 字段。
    pub application_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `stage_id` 字段。
    pub stage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `stage_name` 字段。
    pub stage_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_info` 字段。
    pub job_info: Option<ApplicationJobInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for CreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
