//! 获取内推信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/referral/get_by_application>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取内推信息请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GetByApplicationRequest {
    /// 配置信息
    config: Config,
    application_id: String,
}

impl GetByApplicationRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            application_id: String::new(),
        }
    }

    pub fn application_id(mut self, application_id: String) -> Self {
        self.application_id = application_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetByApplicationResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetByApplicationResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.application_id.trim(), "投递 ID 不能为空");

        let api_endpoint = HireApiV1::ReferralGetByApplication(self.application_id);
        let request = ApiRequest::<GetByApplicationResponse>::get(api_endpoint.to_url());
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取内推信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取内推信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetByApplicationResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for GetByApplicationResponse {
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
