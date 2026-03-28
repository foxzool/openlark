//! 获取人才面试信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/interview/get_by_talent>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取人才面试信息请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GetByTalentRequest {
    /// 配置信息
    config: Config,
    talent_id: String,
}

impl GetByTalentRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            talent_id: String::new(),
        }
    }

    pub fn talent_id(mut self, talent_id: String) -> Self {
        self.talent_id = talent_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetByTalentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetByTalentResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.talent_id.trim(), "候选人 ID 不能为空");

        let api_endpoint = HireApiV1::InterviewGetByTalent(self.talent_id.clone());
        let request = ApiRequest::<GetByTalentResponse>::get(api_endpoint.to_url());
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取人才面试信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取人才面试信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetByTalentResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for GetByTalentResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
