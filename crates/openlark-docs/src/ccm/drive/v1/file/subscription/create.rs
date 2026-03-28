//! 创建订阅
//!
//! 为云文档创建订阅关系，用于接收文档变更通知。
//!
//! ## 功能说明
//! - 为文档创建订阅关系
//! - 支持订阅评论更新等事件
//!
//! ## 字段说明
//! - `file_token`: 文件 token，标识要订阅的文档
//! - `subscription_id`: 订阅关系 ID（可选）
//! - `subscription_type`: 订阅类型，如 comment_update
//! - `is_subcribe`: 是否订阅（可选）
//! - `file_type`: 文档类型，支持 doc/docx/wiki
//!
//! ## 使用示例
//! ```ignore
//! let request = CreateFileSubscriptionRequest::new("file_token", "comment_update", "docx")
//!     .is_subcribe(true);
//! let subscription = create_file_subscription(request, &config, None).await?;
//! ```
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/create>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::Subscription;

/// 创建订阅请求

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct CreateFileSubscriptionRequest {
    /// 文件 token
    pub file_token: String,

    /// 订阅关系 ID（可选）
    pub subscription_id: Option<String>,

    /// 订阅类型（例如 comment_update）
    pub subscription_type: String,

    /// 是否订阅（可选）
    pub is_subcribe: Option<bool>,

    /// 文档类型（必填）
    pub file_type: String,
}

impl CreateFileSubscriptionRequest {
    pub fn new(
        file_token: impl Into<String>,

        subscription_type: impl Into<String>,

        file_type: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),

            subscription_id: None,

            subscription_type: subscription_type.into(),

            is_subcribe: None,

            file_type: file_type.into(),
        }
    }

    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.subscription_id = Some(subscription_id.into());

        self
    }

    pub fn is_subcribe(mut self, is_subcribe: bool) -> Self {
        self.is_subcribe = Some(is_subcribe);

        self
    }
}

#[derive(Debug, Serialize)]

struct CreateFileSubscriptionRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_id: Option<String>,

    subscription_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    is_subcribe: Option<bool>,

    file_type: String,
}

pub type CreateFileSubscriptionResponse = Subscription;

/// 创建订阅
pub async fn create_file_subscription(
    request: CreateFileSubscriptionRequest,

    config: &Config,

    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<CreateFileSubscriptionResponse> {
    // ========== 参数校验 ==========

    if request.file_token.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_token",
            "file_token 不能为空",
        ));
    }

    if request.subscription_type.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "subscription_type",
            "subscription_type 不能为空",
        ));
    }

    match request.subscription_type.as_str() {
        "comment_update" => {}

        _ => {
            return Err(openlark_core::error::validation_error(
                "subscription_type",
                "subscription_type 仅支持 comment_update",
            ));
        }
    }

    if request.file_type.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_type",
            "file_type 不能为空",
        ));
    }

    match request.file_type.as_str() {
        "doc" | "docx" | "wiki" => {}

        _ => {
            return Err(openlark_core::error::validation_error(
                "file_type",
                "file_type 仅支持 doc/docx/wiki",
            ));
        }
    }

    // ========== 构建 API 请求 ==========
    let api_endpoint = DriveApi::CreateFileSubscription(request.file_token.clone());

    let api_request: ApiRequest<CreateFileSubscriptionResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(
            &CreateFileSubscriptionRequestBody {
                subscription_id: request.subscription_id,

                subscription_type: request.subscription_type,

                is_subcribe: request.is_subcribe,

                file_type: request.file_type,
            },
            "创建订阅",
        )?);

    // ========== 发送请求并返回响应 ==========
    let response = Transport::request(api_request, config, option).await?;

    extract_response_data(response, "创建订阅")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file_subscription_request_builder() {
        let request = CreateFileSubscriptionRequest::new("file_token", "comment_update", "docx");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.subscription_type, "comment_update");
        assert_eq!(request.file_type, "docx");
        assert!(request.subscription_id.is_none());
        assert!(request.is_subcribe.is_none());
    }

    #[test]
    fn test_create_file_subscription_request_with_params() {
        let request = CreateFileSubscriptionRequest::new("file_token", "comment_update", "docx")
            .subscription_id("sub_123")
            .is_subcribe(true);

        assert_eq!(request.subscription_id, Some("sub_123".to_string()));
        assert_eq!(request.is_subcribe, Some(true));
    }

    #[test]
    fn test_create_file_subscription_request_empty_fields() {
        let request = CreateFileSubscriptionRequest::new("", "comment_update", "docx");
        assert!(request.file_token.is_empty());

        let request2 = CreateFileSubscriptionRequest::new("token", "", "docx");
        assert!(request2.subscription_type.is_empty());

        let request3 = CreateFileSubscriptionRequest::new("token", "comment_update", "");
        assert!(request3.file_type.is_empty());
    }
}
