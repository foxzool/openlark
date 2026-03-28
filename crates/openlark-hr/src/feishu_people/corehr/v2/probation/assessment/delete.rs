//! 删除试用期考核信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/probation.assessment/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 删除试用期考核信息请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DeleteRequest {
    /// 配置信息
    config: Config,
    assessment_id: Option<String>,
    request_body: Option<Value>,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            assessment_id: None,
            request_body: None,
        }
    }

    pub fn assessment_id(mut self, assessment_id: String) -> Self {
        self.assessment_id = Some(assessment_id);
        self
    }

    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let assessment_id = self.assessment_id.unwrap_or_default();
        validate_required!(assessment_id.trim(), "assessment_id 不能为空");

        let api_endpoint = FeishuPeopleApiV2::ProbationAssessmentDelete(assessment_id);
        let mut request = ApiRequest::<DeleteResponse>::delete(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "删除试用期考核信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 删除试用期考核信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for DeleteResponse {
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
