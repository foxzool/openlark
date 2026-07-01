//! 批量获取邮件组管理员
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

#[derive(Debug, Clone)]
/// 待补充文档。
pub struct ListMailGroupManagerRequest {
    config: Arc<Config>,
    mailgroup_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 待补充文档。
pub struct ListMailGroupManagerResponse {
    /// 待补充文档。
    pub data: Option<ListMailGroupManagerData>,
}

impl ApiResponseTrait for ListMailGroupManagerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 待补充文档。
pub struct ListMailGroupManagerData {
    /// 待补充文档。
    pub managers: Vec<MailGroupManager>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 待补充文档。
pub struct MailGroupManager {
    /// 待补充文档。
    pub manager_id: String,
    /// 待补充文档。
    pub manager_email: String,
}

impl ListMailGroupManagerRequest {
    /// 待补充文档。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
        }
    }

    /// 待补充文档。
    pub async fn execute(self) -> SDKResult<ListMailGroupManagerResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 待补充文档。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListMailGroupManagerResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/managers",
            self.mailgroup_id
        );
        let req: ApiRequest<ListMailGroupManagerResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("批量获取邮件组管理员", "响应数据为空")
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
        let request = ListMailGroupManagerRequest::new(arc_config.clone(), "test".to_string());
        let _ = request;
    }
}
