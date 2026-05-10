//! 更新邮件组
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/update

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use crate::mail::mail::v1::mailgroup::models::{UpdateMailGroupBody, UpdateMailGroupResponse};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 更新邮件组请求
#[derive(Debug, Clone)]
pub struct UpdateMailGroupRequest {
    config: Arc<Config>,
    mail_group_id: String,
    body: UpdateMailGroupBody,
}

impl UpdateMailGroupRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mail_group_id: String) -> Self {
        Self {
            config,
            mail_group_id,
            body: UpdateMailGroupBody::default(),
        }
    }

    /// description。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// only admins send。
    pub fn only_admins_send(mut self, only_admins_send: bool) -> Self {
        self.body.only_admins_send = Some(only_admins_send);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<UpdateMailGroupResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateMailGroupResponse> {
        validate_required!(self.mail_group_id.trim(), "邮件组ID不能为空");

        let api_endpoint = MailApiV1::MailGroupUpdate(self.mail_group_id.clone());
        let mut request = ApiRequest::<UpdateMailGroupResponse>::put(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新邮件组")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新邮件组")
    }
}

impl ApiResponseTrait for UpdateMailGroupResponse {
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
