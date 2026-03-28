//! 批量新增部门
//!
//! docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uMDOwUjLzgDM14yM4ATN>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::CONTACT_V2_DEPARTMENT_BATCH_ADD,
};

/// 批量新增部门请求
pub struct BatchAddDepartmentsRequest {
    config: Config,
}

impl BatchAddDepartmentsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uMDOwUjLzgDM14yM4ATN>
    pub async fn execute(self, params: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(params, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        params: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/contact/v2/department/batch_add
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(CONTACT_V2_DEPARTMENT_BATCH_ADD)
            .body(serialize_params(&params, "批量新增部门")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "批量新增部门")
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
