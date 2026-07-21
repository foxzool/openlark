//! 获取企业信息
//!
//! 文档: <https://open.feishu.cn/document/server-docs/tenant-v2/query>
//! docPath: <https://open.feishu.cn/document/server-docs/tenant-v2/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取企业信息 Builder
#[derive(Debug, Clone)]
pub struct TenantQueryRequestBuilder {
    config: Config,
}

impl TenantQueryRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TenantQueryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TenantQueryResponse> {
        let url = "/open-apis/tenant/v2/tenant/query";

        let req: ApiRequest<TenantQueryResponse> = ApiRequest::get(url);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 企业信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TenantQueryResponse {
    /// 租户名称
    pub name: String,
    /// 租户编号
    #[serde(rename = "tenant_code")]
    pub tenant_code: Option<String>,
    /// 租户图标
    pub icon: Option<String>,
    /// 租户国际化名称
    #[serde(rename = "i18n_name")]
    pub i18n_name: Option<I18nName>,
}

/// 国际化名称
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct I18nName {
    /// 中文名称
    #[serde(rename = "zh_cn")]
    pub zh_cn: Option<String>,
    /// 英文名称
    #[serde(rename = "en_us")]
    pub en_us: Option<String>,
    /// 日文名称
    #[serde(rename = "ja_jp")]
    pub ja_jp: Option<String>,
}

impl ApiResponseTrait for TenantQueryResponse {}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to TenantQueryRequestBuilder, will be removed in v1.0 (#271)")]
pub type TenantQueryBuilder = TenantQueryRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../tenant/v2/tenant/query → 强类型 TenantQueryResponse。
    #[tokio::test]
    async fn test_query_tenant_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/tenant/v2/tenant/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "name": "acme",
                    "tenant_code": "tc_001",
                    "icon": "https://example.com/icon.png",
                    "i18n_name": {
                        "zh_cn": "ACME 中国",
                        "en_us": "ACME",
                        "ja_jp": "ACME JP"
                    }
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

        let resp = TenantQueryRequestBuilder::new(config)
            .execute()
            .await
            .expect("获取企业信息应成功");
        assert_eq!(resp.name, "acme");
        assert_eq!(resp.tenant_code.as_deref(), Some("tc_001"));
        assert_eq!(resp.i18n_name.unwrap().en_us.unwrap(), "ACME");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/tenant/v2/tenant/query");
    }
}
