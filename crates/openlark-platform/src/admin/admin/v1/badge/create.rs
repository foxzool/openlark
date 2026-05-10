//! 创建勋章 API
//!
//! API文档: https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/create
//! docPath: https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/create-2

use crate::common::api_endpoints::AdminApiV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 创建勋章请求
pub struct CreateBadgeBuilder {
    name: String,
    description: Option<String>,
    icon_url: Option<String>,
    config: Config,
}

impl CreateBadgeBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            name: String::new(),
            description: None,
            icon_url: None,
            config,
        }
    }

    /// 设置名称。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 设置描述。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置图标地址。
    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<CreateBadgeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateBadgeResponse> {
        validate_required!(self.name, "勋章名称不能为空");

        let request_body = CreateBadgeRequest {
            name: self.name,
            description: self.description,
            icon_url: self.icon_url,
        };

        let api_request: ApiRequest<CreateBadgeResponse> =
            ApiRequest::post(AdminApiV1::CreateBadge.path())
                .body(serde_json::to_value(&request_body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("创建勋章", "响应数据为空"))
    }
}

/// 创建勋章请求体
#[derive(Debug, Serialize)]
struct CreateBadgeRequest {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<String>,
}

/// 创建勋章响应
#[derive(Debug, Clone, Deserialize, Serialize)]
/// 创建勋章的响应。
pub struct CreateBadgeResponse {
    /// 勋章 ID。
    pub badge_id: String,
    /// 名称。
    pub name: String,
    /// 描述。
    pub description: Option<String>,
    /// 图标地址。
    pub icon_url: Option<String>,
    /// 创建时间。
    pub create_time: String,
}

impl ApiResponseTrait for CreateBadgeResponse {
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
        let request = CreateBadgeBuilder::new(config.clone())
            .name("test".to_string())
            .description("test".to_string());
        let _ = request;
    }
}
