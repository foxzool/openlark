//! 获取所有邮件组成员
//! docPath: <https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取所有邮件组成员的请求。
#[derive(Debug, Clone)]
pub struct ListMailGroupMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
}

/// 获取所有邮件组成员的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMailGroupMemberResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListMailGroupMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListMailGroupMemberRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
        }
    }

    /// 执行获取所有邮件组成员请求。
    pub async fn execute(self) -> SDKResult<ListMailGroupMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListMailGroupMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members",
            self.mailgroup_id
        );
        let req: ApiRequest<ListMailGroupMemberResponse> = ApiRequest::get(&path);

        Transport::request_typed(req, &self.config, Some(option), "获取所有邮件组成员").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;
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
        let request = ListMailGroupMemberRequest::new(arc_config.clone(), "test".to_string());
        let _ = request;
    }
}
