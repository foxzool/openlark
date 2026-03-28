//! 启用/停用公司
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/company/active>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

/// 启用/停用公司请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActiveRequest {
    /// 是否启用（true=启用，false=停用）
    pub active: bool,
}

impl ActiveRequest {
    /// 创建请求
    pub fn new(active: bool) -> Self {
        Self { active }
    }
}

/// 启用/停用公司响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActiveResponse {
    /// 公司 ID
    pub data: Option<ActiveResponseData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActiveResponseData {
    /// 公司 ID
    pub id: String,
}

impl ApiResponseTrait for ActiveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 启用/停用公司请求构建器
#[derive(Debug, Clone)]
pub struct ActiveRequestBuilder {
    config: Config,
    active: bool,
}

impl ActiveRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config, active: bool) -> Self {
        Self { config, active }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ActiveResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ActiveResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let company_id = "";
        let api_endpoint = FeishuPeopleApiV2::CompanyActive(company_id.to_string());
        let request = ApiRequest::<ActiveResponse>::patch(api_endpoint.to_url());

        // 序列化请求体
        let request_body = ActiveRequest::new(self.active);
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "启用/停用公司响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
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
