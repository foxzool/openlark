//! 创建邮件组成员
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/create

use crate::common::api_utils::serialize_params;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
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
    /// 成员 ID。
    pub member_id: String,
    /// member_type 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
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

    /// member id。
    pub fn member_id(mut self, member_id: impl Into<String>) -> Self {
        self.body.member_id = member_id.into();
        self
    }

    /// member type。
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
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members",
            self.mailgroup_id
        );
        let req: ApiRequest<CreateMailGroupMemberResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "创建邮件组成员")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("创建邮件组成员", "响应数据为空"))
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
        let _config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateMailGroupMemberRequest::new(arc_config.clone(), "test".to_string())
            .member_id("test".to_string())
            .member_type("test".to_string());
        let _ = request;
    }
}
