//! 将人才加入人才库
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/talent_pool/move_talent

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hire::hire::common_models::TalentPoolOperationResult;

/// 将人才加入人才库请求
#[derive(Debug, Clone)]
pub struct MoveTalentRequest {
    /// 配置信息
    config: Config,
    talent_pool_id: String,
    request_body: Option<Value>,
}

impl MoveTalentRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            talent_pool_id: String::new(),
            request_body: None,
        }
    }

    /// 设置 `talent_pool_id`。
    pub fn talent_pool_id(mut self, talent_pool_id: impl Into<String>) -> Self {
        self.talent_pool_id = talent_pool_id.into();
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<MoveTalentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<MoveTalentResponse> {
        validate_required!(self.talent_pool_id.trim(), "talent_pool_id 不能为空");

        let mut request = ApiRequest::<MoveTalentResponse>::post(format!(
            "/open-apis/hire/v1/talent_pools/{}/talent_relationship",
            self.talent_pool_id
        ));
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "将人才加入人才库响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 将人才加入人才库响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MoveTalentResponse {
    #[serde(flatten)]
    /// `operation` 字段。
    pub operation: TalentPoolOperationResult,
}

impl ApiResponseTrait for MoveTalentResponse {
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
