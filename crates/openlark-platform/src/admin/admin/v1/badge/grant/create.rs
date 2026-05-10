//! 创建勋章授予名单 API
//!
//! API文档: https://open.feishu.cn/document/server-docs/admin-v1/badge/badge-grant/create
//! docPath: https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/create-2

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 创建勋章授予名单请求
pub struct CreateBadgeGrantBuilder {
    badge_id: String,
    user_ids: Vec<String>,
    config: Config,
}

impl CreateBadgeGrantBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            user_ids: Vec::new(),
            config,
        }
    }

    /// 设置勋章 ID。
    pub fn badge_id(mut self, badge_id: impl Into<String>) -> Self {
        self.badge_id = badge_id.into();
        self
    }

    /// 设置用户 ID 列表。
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<CreateBadgeGrantResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateBadgeGrantResponse> {
        validate_required!(self.badge_id, "勋章ID不能为空");
        validate_required!(self.user_ids, "用户ID列表不能为空");

        let request_body = CreateBadgeGrantRequest {
            user_ids: self.user_ids,
        };

        let api_request: ApiRequest<CreateBadgeGrantResponse> = ApiRequest::post(format!(
            "/open-apis/admin/v1/badges/{}/grants",
            self.badge_id
        ))
        .body(serde_json::to_value(&request_body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("创建勋章授予名单", "响应数据为空")
        })
    }
}

#[derive(Debug, Serialize)]
struct CreateBadgeGrantRequest {
    user_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 创建勋章授予名单的响应。
pub struct CreateBadgeGrantResponse {
    /// 授予记录 ID。
    pub grant_id: String,
    /// 勋章 ID。
    pub badge_id: String,
    /// 用户 ID 列表。
    pub user_ids: Vec<String>,
    /// 创建时间。
    pub create_time: String,
}

impl ApiResponseTrait for CreateBadgeGrantResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateBadgeGrantBuilder::new(config.clone())
            .badge_id("test".to_string())
            .user_ids(vec!["user_001".to_string()]);
        let _ = request;
    }
}
