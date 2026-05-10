//! 获取邮件组所有别名
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/list

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// List Mail Group Alias Request。
#[derive(Debug, Clone)]
pub struct ListMailGroupAliasRequest {
    config: Arc<Config>,
    mailgroup_id: String,
}

/// List Mail Group Alias Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMailGroupAliasResponse {
    /// 响应数据。
    pub data: Option<ListMailGroupAliasData>,
}

impl ApiResponseTrait for ListMailGroupAliasResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// List Mail Group Alias Data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMailGroupAliasData {
    /// aliases 字段。
    pub aliases: Vec<MailGroupAlias>,
}

/// Mail Group Alias。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupAlias {
    /// 别名 ID。
    pub alias_id: String,
    /// alias 字段。
    pub alias: String,
}

impl ListMailGroupAliasRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListMailGroupAliasResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListMailGroupAliasResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/aliases",
            self.mailgroup_id
        );
        let req: ApiRequest<ListMailGroupAliasResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取邮件组所有别名", "响应数据为空")
        })
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
        let request = ListMailGroupAliasRequest::new(arc_config.clone(), "test".to_string());
        let _ = request;
    }
}
