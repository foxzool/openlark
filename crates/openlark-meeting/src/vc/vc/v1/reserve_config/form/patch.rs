//! 更新会议室预定表单
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/patch-2>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, validate_required,
    SDKResult,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 更新会议室预定表单请求
pub struct PatchReserveConfigFormRequest {
    config: Config,
    reserve_config_id: String,
}

impl PatchReserveConfigFormRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            reserve_config_id: String::new(),
        }
    }

    /// 预定配置 ID（路径参数）
    pub fn reserve_config_id(mut self, reserve_config_id: impl Into<String>) -> Self {
        self.reserve_config_id = reserve_config_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/patch-2>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default(), body)
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
        body: serde_json::Value,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.reserve_config_id, "reserve_config_id 不能为空");

        // url: PATCH:/open-apis/vc/v1/reserve_configs/:reserve_config_id/form
        let endpoint = VcApiV1::ReserveConfigFormPatch(self.reserve_config_id.clone());
        let req: ApiRequest<serde_json::Value> = ApiRequest::patch(endpoint.to_url())
            .body(serialize_params(&body, "更新会议室预定表单")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新会议室预定表单")
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
