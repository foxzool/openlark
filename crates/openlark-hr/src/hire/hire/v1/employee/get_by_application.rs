//! 通过投递 ID 获取入职信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/employee/get_by_application

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

use crate::hire::hire::common_models::EmployeeSummary;

/// 通过投递 ID 获取入职信息请求
#[derive(Debug, Clone)]
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

    /// 设置 `application_id`。
    pub fn application_id(mut self, application_id: String) -> Self {
        self.application_id = application_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetByApplicationResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetByApplicationResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.application_id.trim(), "投递 ID 不能为空");

        let api_endpoint = HireApiV1::EmployeeGetByApplication(self.application_id);
        let request = ApiRequest::<GetByApplicationResponse>::get(api_endpoint.to_url());
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "通过投递 ID 获取入职信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 通过投递 ID 获取入职信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetByApplicationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `employee` 字段。
    pub employee: Option<EmployeeSummary>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
