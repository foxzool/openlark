//! 获取订阅状态
//!
//! 获取指定文档的订阅状态详情。
//!
//! ## 功能说明
//! - 根据 file_token 和 subscription_id 获取订阅详情
//!
//! ## 字段说明
//! - `file_token`: 文件 token，标识文档
//! - `subscription_id`: 订阅 ID，标识订阅关系
//!
//! ## 使用示例
//! ```ignore
//! let request = GetSubscriptionRequest::new("file_token", "sub_123");
//! let subscription = get_subscription(request, &config, None).await?;
//! ```
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/get>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::Subscription;

/// 获取订阅状态请求

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct GetSubscriptionRequest {
    /// 文件 token
    pub file_token: String,

    /// 订阅 ID
    pub subscription_id: String,
}

impl GetSubscriptionRequest {
    pub fn new(file_token: impl Into<String>, subscription_id: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),

            subscription_id: subscription_id.into(),
        }
    }
}

pub type GetSubscriptionResponse = Subscription;

/// 获取订阅状态
pub async fn get_subscription(
    request: GetSubscriptionRequest,

    config: &Config,

    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<GetSubscriptionResponse> {
    // ========== 参数校验 ==========

    if request.file_token.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_token",
            "file_token 不能为空",
        ));
    }

    if request.subscription_id.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "subscription_id",
            "subscription_id 不能为空",
        ));
    }

    // ========== 构建 API 请求 ==========
    let api_endpoint =
        DriveApi::GetFileSubscription(request.file_token.clone(), request.subscription_id.clone());

    let api_request: ApiRequest<GetSubscriptionResponse> = ApiRequest::get(&api_endpoint.to_url());

    // ========== 发送请求并返回响应 ==========
    let response = Transport::request(api_request, config, option).await?;

    extract_response_data(response, "获取订阅状态")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_subscription_request_builder() {
        let request = GetSubscriptionRequest::new("file_token", "sub_123");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.subscription_id, "sub_123");
    }

    #[test]
    fn test_get_subscription_request_empty_fields() {
        let request = GetSubscriptionRequest::new("", "sub_123");
        assert!(request.file_token.is_empty());

        let request2 = GetSubscriptionRequest::new("token", "");
        assert!(request2.subscription_id.is_empty());
    }
}
