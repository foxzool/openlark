//! 获取用户列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/acs-v1/user/list>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取用户列表请求
///
/// 支持分页及部门过滤。
#[derive(Debug)]
pub struct ListUsersRequest {
    /// 配置信息。
    config: Config,
    /// 页面大小（可选，默认 20，最大 100）。
    page_size: Option<i32>,
    /// 分页标记（可选）。
    page_token: Option<String>,
    /// 部门 ID 过滤（可选）。
    department_id: Option<String>,
}

/// 获取用户列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersResponse {
    /// 是否有更多数据。
    pub has_more: bool,
    /// 分页标记。
    pub page_token: Option<String>,
    /// 用户列表（飞书文档未给出明确 schema，暂用 `serde_json::Value` 透传）。
    pub items: Option<Vec<serde_json::Value>>,
}

impl openlark_core::api::ApiResponseTrait for ListUsersResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

impl ListUsersRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            department_id: None,
        }
    }

    /// 设置页面大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置部门 ID 过滤。
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = Some(department_id.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListUsersResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ListUsersResponse> {
        let req: ApiRequest<ListUsersResponse> = ApiRequest::get("/open-apis/acs/v1/users")
            .query_opt("page_size", self.page_size.map(|v| v.to_string()))
            .query_opt("page_token", self.page_token.as_ref())
            .query_opt("department_id", self.department_id.as_ref())
            .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        Ok(resp.data.unwrap_or(ListUsersResponse {
            has_more: false,
            page_token: None,
            items: None,
        }))
    }
}
