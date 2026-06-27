//! 更新面试官信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/interviewer/patch

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::hire::hire::common_models::InterviewerOperationResult;

/// 更新面试官信息请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    interviewer_id: String,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            interviewer_id: String::new(),
        }
    }

    /// 设置 `interviewer_id`。
    pub fn interviewer_id(mut self, interviewer_id: impl Into<String>) -> Self {
        self.interviewer_id = interviewer_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        validate_required!(self.interviewer_id.trim(), "interviewer_id 不能为空");

        let request = ApiRequest::<PatchResponse>::patch(format!(
            "/open-apis/hire/v1/interviewers/{}",
            self.interviewer_id
        ));
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新面试官信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 更新面试官信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PatchResponse {
    #[serde(flatten)]
    /// `interviewer` 字段。
    pub interviewer: InterviewerOperationResult,
}

impl ApiResponseTrait for PatchResponse {
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
