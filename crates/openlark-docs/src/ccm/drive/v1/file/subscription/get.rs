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

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;
use crate::common::api_utils::*;

use super::models::Subscription;

/// 获取订阅状态请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscriptionRequest {
    /// 文件 token
    pub file_token: String,

    /// 订阅 ID
    pub subscription_id: String,

    /// 文档类型（必填，官方请求体字段）
    pub file_type: String,
}

impl GetSubscriptionRequest {
    /// 创建新的订阅状态查询请求。
    ///
    /// # 参数
    ///
    /// - `file_token`: 文件 token
    /// - `subscription_id`: 订阅 ID
    /// - `file_type`: 文档类型（doc/docx/wiki）
    pub fn new(
        file_token: impl Into<String>,
        subscription_id: impl Into<String>,
        file_type: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),

            subscription_id: subscription_id.into(),

            file_type: file_type.into(),
        }
    }
}

/// 获取订阅状态请求体
///
/// 官方文档为 GET 请求声明的请求体（Go SDK 同样以 body 发送）：`file_type` 必填。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscriptionRequestBody {
    /// 文档类型（doc/docx/wiki）
    pub file_type: String,
}

/// 获取订阅状态响应 data。
pub type GetSubscriptionResponse = Subscription;

/// 获取订阅状态
pub async fn get_subscription(
    request: GetSubscriptionRequest,

    config: &Config,

    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<GetSubscriptionResponse> {
    // ========== 参数校验 ==========

    validate_required!(request.file_token, "file_token 不能为空");
    validate_required!(request.subscription_id, "subscription_id 不能为空");
    validate_required!(request.file_type, "file_type 不能为空");

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
    let api_endpoint =
        DriveApi::GetFileSubscription(request.file_token.clone(), request.subscription_id.clone());

    // 官方文档为该 GET 请求声明了必填请求体 file_type，与 Go SDK 一致以 body 发送
    let api_request: ApiRequest<GetSubscriptionResponse> =
        api_endpoint.to_request().body(serialize_params(
            &GetSubscriptionRequestBody {
                file_type: request.file_type,
            },
            "获取订阅状态",
        )?);

    // ========== 发送请求并返回响应 ==========
    Transport::request_typed(api_request, config, option, "获取订阅状态").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    #[test]
    fn test_get_subscription_request_builder() {
        let request = GetSubscriptionRequest::new("file_token", "sub_123", "docx");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.subscription_id, "sub_123");
        assert_eq!(request.file_type, "docx");
    }

    #[test]
    fn test_get_subscription_request_empty_fields() {
        let request = GetSubscriptionRequest::new("", "sub_123", "docx");
        assert!(request.file_token.is_empty());

        let request2 = GetSubscriptionRequest::new("token", "", "docx");
        assert!(request2.subscription_id.is_empty());

        let request3 = GetSubscriptionRequest::new("token", "sub_123", "");
        assert!(request3.file_type.is_empty());
    }

    /// 端到端：GET .../subscriptions/:subscription_id 携带官方请求体 {"file_type": ...}。
    #[tokio::test]
    async fn test_get_subscription_sends_body_file_type() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/drive/v1/files/ftk001/subscriptions/sub_123",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "subscription_id": "sub_123",
                    "subscription_type": "comment_update",
                    "is_subcribe": false,
                    "file_type": "docx"
                }
            })))
            .mount(&server)
            .await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        let resp = get_subscription(
            GetSubscriptionRequest::new("ftk001", "sub_123", "docx"),
            &config,
            None,
        )
        .await
        .expect("获取订阅状态应成功");
        assert_eq!(resp.file_type.as_deref(), Some("docx"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/drive/v1/files/ftk001/subscriptions/sub_123"
        );
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为合法 JSON");
        assert_eq!(body["file_type"], "docx");
    }
}
