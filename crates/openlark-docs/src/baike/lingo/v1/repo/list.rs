//! 获取词库列表
//!
//! docPath: <https://open.feishu.cn/document/lingo-v1/repo/list>
//! doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/repo/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::lingo::v1::models::Repo;
use crate::common::api_endpoints::LingoApiV1;

/// 获取词库列表响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRepoResp {
    /// 词库列表
    #[serde(default)]
    pub items: Vec<Repo>,
}

impl ApiResponseTrait for ListRepoResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取词库列表请求
pub struct ListRepoRequest {
    config: Config,
}

impl ListRepoRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<ListRepoResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ListRepoResp> {
        let api_request: ApiRequest<ListRepoResp> = ApiRequest::get(&LingoApiV1::RepoList.to_url());

        let response: Response<ListRepoResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_repo_request_builder() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = ListRepoRequest::new(config);

        // 验证 request 被成功创建且配置正确
        assert_eq!(request.config.app_id(), "test_app");
    }

    #[test]
    fn test_list_repo_request_new() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = ListRepoRequest::new(config);

        // 验证 request 被成功创建且配置正确
        assert_eq!(request.config.app_id(), "test_app");
        assert_eq!(request.config.app_secret(), "test_secret");
    }
}
