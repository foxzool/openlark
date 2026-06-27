//! 回传笔试安排结果
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/eco_exam/login_info

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hire::hire::common_models::EcoExamOperationResult;

/// 回传笔试安排结果请求
#[derive(Debug, Clone)]
pub struct LoginInfoRequest {
    /// 配置信息
    config: Config,
    exam_id: String,
    request_body: Option<Value>,
}

impl LoginInfoRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            exam_id: String::new(),
            request_body: None,
        }
    }

    /// 设置 `exam_id`。
    pub fn exam_id(mut self, exam_id: impl Into<String>) -> Self {
        self.exam_id = exam_id.into();
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<LoginInfoResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<LoginInfoResponse> {
        validate_required!(self.exam_id.trim(), "exam_id 不能为空");

        let mut request = ApiRequest::<LoginInfoResponse>::post(format!(
            "/open-apis/hire/v1/eco_exams/{}/login_info",
            self.exam_id
        ));
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "回传笔试安排结果响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 回传笔试安排结果响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LoginInfoResponse {
    #[serde(flatten)]
    /// `exam` 字段。
    pub exam: EcoExamOperationResult,
}

impl ApiResponseTrait for LoginInfoResponse {
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
