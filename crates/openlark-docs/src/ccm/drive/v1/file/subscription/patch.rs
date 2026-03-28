//! 更新订阅状态
//!
//! 更新指定文档的订阅状态。
//!
//! ## 功能说明
//! - 更新订阅的开关状态
//! - 支持订阅/取消订阅操作
//!
//! ## 字段说明
//! - `file_token`: 文件 token，标识文档
//! - `subscription_id`: 订阅 ID，标识订阅关系
//! - `is_subscribe`: 是否订阅，true 表示订阅，false 表示取消订阅
//! - `file_type`: 文档类型，支持 doc/docx/wiki
//!
//! ## 使用示例
//! ```ignore
//! let request = PatchSubscriptionRequest::new("file_token", "sub_123", true, "docx");
//! let subscription = patch_subscription(request, &config, None).await?;
//! ```
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/patch>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::Subscription;

/// 更新订阅状态请求

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct PatchSubscriptionRequest {
    /// 文件 token
    pub file_token: String,

    /// 订阅 ID
    pub subscription_id: String,

    /// 是否订阅
    pub is_subscribe: bool,

    /// 文档类型（必填）
    pub file_type: String,
}

impl PatchSubscriptionRequest {
    pub fn new(
        file_token: impl Into<String>,

        subscription_id: impl Into<String>,

        is_subscribe: bool,

        file_type: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),

            subscription_id: subscription_id.into(),

            is_subscribe,

            file_type: file_type.into(),
        }
    }
}

#[derive(Debug, Serialize)]

struct PatchSubscriptionRequestBody {
    is_subscribe: bool,

    file_type: String,
}

pub type PatchSubscriptionResponse = Subscription;

/// 更新订阅状态
pub async fn patch_subscription(
    request: PatchSubscriptionRequest,

    config: &Config,

    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<PatchSubscriptionResponse> {
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
    let api_endpoint = DriveApi::UpdateFileSubscription(
        request.file_token.clone(),
        request.subscription_id.clone(),
    );

    let api_request: ApiRequest<PatchSubscriptionResponse> =
        ApiRequest::patch(&api_endpoint.to_url()).body(serialize_params(
            &PatchSubscriptionRequestBody {
                is_subscribe: request.is_subscribe,

                file_type: request.file_type,
            },
            "更新订阅状态",
        )?);

    // ========== 发送请求并返回响应 ==========
    let response = Transport::request(api_request, config, option).await?;

    extract_response_data(response, "更新订阅状态")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_subscription_request_builder() {
        let request = PatchSubscriptionRequest::new("file_token", "sub_123", true, "docx");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.subscription_id, "sub_123");
        assert!(request.is_subscribe);
        assert_eq!(request.file_type, "docx");
    }

    #[test]
    fn test_patch_subscription_request_empty_fields() {
        let request = PatchSubscriptionRequest::new("", "sub_123", true, "docx");
        assert!(request.file_token.is_empty());

        let request2 = PatchSubscriptionRequest::new("token", "", true, "docx");
        assert!(request2.subscription_id.is_empty());

        let request3 = PatchSubscriptionRequest::new("token", "sub_123", true, "");
        assert!(request3.file_type.is_empty());
    }
}
