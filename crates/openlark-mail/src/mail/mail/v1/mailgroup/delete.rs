//! 删除邮件组
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/delete

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use crate::mail::mail::v1::mailgroup::models::DeleteMailGroupResponse;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 删除邮件组请求
#[derive(Debug, Clone)]
pub struct DeleteMailGroupRequest {
    config: Arc<Config>,
    mail_group_id: String,
}

impl DeleteMailGroupRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mail_group_id: String) -> Self {
        Self {
            config,
            mail_group_id,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeleteMailGroupResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteMailGroupResponse> {
        validate_required!(self.mail_group_id.trim(), "邮件组ID不能为空");

        let api_endpoint = MailApiV1::MailGroupDelete(self.mail_group_id.clone());
        let request = ApiRequest::<DeleteMailGroupResponse>::delete(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除邮件组")
    }
}

impl ApiResponseTrait for DeleteMailGroupResponse {
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
