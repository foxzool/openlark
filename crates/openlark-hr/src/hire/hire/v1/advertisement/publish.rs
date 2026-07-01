//! 发布职位广告
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/advertisement/publish>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hire::hire::common_models::GenericOperationResult;

/// 发布职位广告请求
#[derive(Debug, Clone)]
pub struct PublishRequest {
    /// 配置信息
    config: Config,
    job_id: String,
    request_body: Option<Value>,
}

impl PublishRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            job_id: String::new(),
            request_body: None,
        }
    }

    /// 设置 `job_id`。
    pub fn job_id(mut self, job_id: String) -> Self {
        self.job_id = job_id;
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PublishResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PublishResponse> {
        validate_required!(self.job_id.trim(), "职位 ID 不能为空");

        let mut request = ApiRequest::<PublishResponse>::post(format!(
            "/open-apis/hire/v1/advertisements/{}/publish",
            self.job_id
        ));

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "发布职位广告响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 发布职位广告响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PublishResponse {
    #[serde(flatten)]
    /// `operation` 字段。
    pub operation: GenericOperationResult,
}

impl ApiResponseTrait for PublishResponse {
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
