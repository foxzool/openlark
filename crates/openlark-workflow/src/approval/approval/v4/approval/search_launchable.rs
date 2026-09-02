//! 搜索可发起的审批定义（v4）
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/search_launchable>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::ApprovalExtraApiV4;
use crate::common::api_utils::serialize_params;

/// 搜索可发起的审批定义请求体（v4）。
///
/// 全部字段可选；未提供 `page_size` 时不写入请求体，沿用服务端默认。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchLaunchableApprovalBodyV4 {
    /// 关键词，用于搜索审批定义。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// 语言类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// 分页大小。官方默认 20，范围 1–100。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 可发起的审批定义。
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct LaunchableApprovalV4 {
    /// 审批定义编码。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    /// 审批定义名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_name: Option<String>,
    /// 审批定义描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否三方定义。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_external: Option<bool>,
    /// 提单链接。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_link: Option<String>,
}

/// 搜索可发起的审批定义响应 data（v4）。
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchLaunchableApprovalResponseV4 {
    /// 可发起审批定义列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approvals: Option<Vec<LaunchableApprovalV4>>,
    /// 分页标记。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for SearchLaunchableApprovalResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索可发起的审批定义请求（v4）。
#[derive(Debug, Clone)]
pub struct SearchLaunchableApprovalRequestV4 {
    config: Config,
}

impl SearchLaunchableApprovalRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        body: SearchLaunchableApprovalBodyV4,
    ) -> SDKResult<SearchLaunchableApprovalResponseV4> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: SearchLaunchableApprovalBodyV4,
        option: RequestOption,
    ) -> SDKResult<SearchLaunchableApprovalResponseV4> {
        validate_page_size(body.page_size)?;
        let req: ApiRequest<SearchLaunchableApprovalResponseV4> =
            ApprovalExtraApiV4::SearchLaunchable
                .to_request()
                .body(serialize_params(&body, "搜索可发起的审批定义")?);
        Transport::request_typed(req, &self.config, Some(option), "搜索可发起的审批定义").await
    }
}

fn validate_page_size(page_size: Option<i32>) -> SDKResult<()> {
    if let Some(page_size) = page_size
        && !(1..=100).contains(&page_size)
    {
        return Err(openlark_core::error::validation_error(
            "page_size",
            "page_size 必须在 1~100 之间",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::constants::AccessTokenType;
    use serde_json::json;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn user_option() -> RequestOption {
        RequestOption::builder()
            .user_access_token("test-user-token")
            .build()
    }

    async fn user_test_config() -> (MockServer, Config, RequestOption) {
        let server = MockServer::start().await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        (server, config, user_option())
    }

    #[test]
    fn extra_catalog_is_user_only_post() {
        let endpoint = ApprovalExtraApiV4::SearchLaunchable;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/approvals/search_launchable"
        );
        let req: ApiRequest<()> = endpoint.to_request();
        assert_eq!(
            req.supported_access_token_types(),
            vec![AccessTokenType::User]
        );
    }

    #[test]
    fn omitted_page_size_is_not_serialized() {
        let body = SearchLaunchableApprovalBodyV4 {
            keyword: Some("请假".to_string()),
            ..Default::default()
        };
        let value = serde_json::to_value(&body).expect("serialize");
        assert_eq!(value["keyword"], "请假");
        assert!(value.get("page_size").is_none());
        assert!(value.get("locale").is_none());
        assert!(value.get("page_token").is_none());
    }

    #[tokio::test]
    async fn page_size_out_of_range_fails_before_request() {
        let config = Config::default();
        for page_size in [0, 101] {
            let err = SearchLaunchableApprovalRequestV4::new(config.clone())
                .execute(SearchLaunchableApprovalBodyV4 {
                    page_size: Some(page_size),
                    ..Default::default()
                })
                .await
                .expect_err("非法 page_size 应校验失败");
            assert!(
                err.to_string().contains("page_size"),
                "错误应提到 page_size: {err}"
            );
        }
    }

    #[tokio::test]
    async fn search_launchable_uses_user_token_and_omits_page_size() {
        let (server, config, option) = user_test_config().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/approval/v4/approvals/search_launchable"))
            .and(header("Authorization", "Bearer test-user-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "approvals": [{
                        "approval_code": "code-1",
                        "approval_name": "请假",
                        "description": "请假审批",
                        "is_external": false,
                        "create_link": "https://www.example.com"
                    }],
                    "has_more": false
                }
            })))
            .mount(&server)
            .await;

        let response = SearchLaunchableApprovalRequestV4::new(config)
            .execute_with_options(
                SearchLaunchableApprovalBodyV4 {
                    keyword: Some("请假".to_string()),
                    ..Default::default()
                },
                option,
            )
            .await
            .expect("搜索可发起审批定义应成功");
        assert_eq!(response.has_more, Some(false));
        assert_eq!(
            response
                .approvals
                .as_ref()
                .and_then(|items| items[0].approval_name.clone()),
            Some("请假".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为合法 JSON");
        assert_eq!(body["keyword"], "请假");
        assert!(
            body.get("page_size").is_none(),
            "未传 page_size 时请求体不应含该字段: {body}"
        );
    }
}
