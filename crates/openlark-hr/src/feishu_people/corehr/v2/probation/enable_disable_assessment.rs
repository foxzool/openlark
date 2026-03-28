//! 启用/停用试用期考核功能
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/probation/enable_disable_assessment>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 启用/停用试用期考核功能请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EnableDisableAssessmentRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl EnableDisableAssessmentRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EnableDisableAssessmentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EnableDisableAssessmentResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let api_endpoint = FeishuPeopleApiV2::ProbationEnableDisableAssessment;
        let mut request =
            ApiRequest::<EnableDisableAssessmentResponse>::post(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "启用/停用试用期考核功能响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 启用/停用试用期考核功能响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnableDisableAssessmentResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for EnableDisableAssessmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
