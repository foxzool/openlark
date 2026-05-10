//! 创建邮件组别名
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

/// Create Mail Group Alias Request。
#[derive(Debug, Clone)]
pub struct CreateMailGroupAliasRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: CreateMailGroupAliasBody,
}

/// Create Mail Group Alias Body。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateMailGroupAliasBody {
    /// alias 字段。
    pub alias: String,
}

/// Create Mail Group Alias Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMailGroupAliasResponse {
    /// 响应数据。
    pub data: Option<MailGroupAliasData>,
}

impl ApiResponseTrait for CreateMailGroupAliasResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Mail Group Alias Data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupAliasData {
    /// 别名 ID。
    pub alias_id: String,
}

impl CreateMailGroupAliasRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: CreateMailGroupAliasBody::default(),
        }
    }

    /// 访问别名资源。
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.body.alias = alias.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreateMailGroupAliasResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateMailGroupAliasResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/aliases",
            self.mailgroup_id
        );
        let req: ApiRequest<CreateMailGroupAliasResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "创建邮件组别名")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("创建邮件组别名", "响应数据为空"))
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
        let request = CreateMailGroupAliasRequest::new(arc_config.clone(), "test".to_string())
            .alias("test".to_string());
        let _ = request;
    }
}
