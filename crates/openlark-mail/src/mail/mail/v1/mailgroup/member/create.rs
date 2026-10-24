//! 创建邮件组成员
//! docPath: <https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup-member/create>

use crate::common::api_utils::serialize_params;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Create Mail Group Member Request。
#[derive(Debug, Clone)]
pub struct CreateMailGroupMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: CreateMailGroupMemberBody,
}

/// Create Mail Group Member Body。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateMailGroupMemberBody {
    /// 成员邮箱地址（当成员类型是 EXTERNAL_USER/MAIL_GROUP/OTHER_MEMBER 时有值）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 租户内用户的唯一标识（当成员类型是 USER 时有值）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 租户内部门的唯一标识（当成员类型是 DEPARTMENT 时有值）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 成员类型（USER/DEPARTMENT/COMPANY/EXTERNAL_USER/MAIL_GROUP/PUBLIC_MAILBOX/OTHER_MEMBER）。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

/// Create Mail Group Member Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMailGroupMemberResponse {
    /// 响应数据。
    pub data: Option<MailGroupMemberData>,
}

impl ApiResponseTrait for CreateMailGroupMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Mail Group Member Data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupMemberData {
    /// 成员 ID。
    pub member_id: String,
}

impl CreateMailGroupMemberRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: CreateMailGroupMemberBody::default(),
        }
    }

    /// email。
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.body.email = Some(email.into());
        self
    }

    /// user id。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = Some(user_id.into());
        self
    }

    /// department id。
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.body.department_id = Some(department_id.into());
        self
    }

    /// member type（USER/DEPARTMENT/COMPANY/EXTERNAL_USER/MAIL_GROUP/PUBLIC_MAILBOX/OTHER_MEMBER）。
    pub fn member_type(mut self, member_type: impl Into<String>) -> Self {
        self.body.member_type = Some(member_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreateMailGroupMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateMailGroupMemberResponse> {
        validate_required!(self.mailgroup_id, "mailgroup_id 不能为空");
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members",
            self.mailgroup_id
        );
        let req: ApiRequest<CreateMailGroupMemberResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "创建邮件组成员")?);

        Transport::request_typed(req, &self.config, Some(option), "创建邮件组成员").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_builder_basic() {
        let arc_config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = CreateMailGroupMemberRequest::new(arc_config.clone(), "test".to_string())
            .user_id("ou_123")
            .member_type("USER");
        assert_eq!(request.body.user_id.as_deref(), Some("ou_123"));
        assert_eq!(request.body.member_type.as_deref(), Some("USER"));
        // wire format：member_type 序列化为官方字段名 type，member_id 不在请求体中
        let value = serde_json::to_value(&request.body).expect("序列化 body");
        assert_eq!(
            value,
            serde_json::json!({ "user_id": "ou_123", "type": "USER" })
        );
    }
}
