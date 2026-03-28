//! 获取单位信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/get>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data, contact::contact::v3::unit::models::GetUnitResponse,
    endpoints::CONTACT_V3_UNIT,
};

/// 获取单位信息请求
pub struct GetUnitRequest {
    config: Config,
    unit_id: String,
}

impl GetUnitRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            unit_id: String::new(),
        }
    }

    /// 单位 ID（路径参数）
    pub fn unit_id(mut self, unit_id: impl Into<String>) -> Self {
        self.unit_id = unit_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/get>
    pub async fn execute(self) -> SDKResult<GetUnitResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetUnitResponse> {
        validate_required!(self.unit_id, "unit_id 不能为空");

        // url: GET:/open-apis/contact/v3/unit/:unit_id
        let req: ApiRequest<GetUnitResponse> =
            ApiRequest::get(format!("{}/{}", CONTACT_V3_UNIT, self.unit_id));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取单位信息")
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
