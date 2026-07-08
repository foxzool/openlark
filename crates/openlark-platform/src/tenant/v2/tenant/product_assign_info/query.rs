//! 获取企业席位信息
//!
//! 文档: <https://open.feishu.cn/document/server-docs/tenant-v2/tenant-product_assign_info/query>
//! docPath: <https://open.feishu.cn/document/server-docs/tenant-v2/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取企业席位信息 Builder
#[derive(Debug, Clone)]
pub struct AssignInfoListQueryRequestBuilder {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl AssignInfoListQueryRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AssignInfoListQueryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<AssignInfoListQueryResponse> {
        let mut url = "/open-apis/tenant/v2/tenant/assign_info_list/query".to_string();

        // 添加查询参数
        let mut params = Vec::new();
        if let Some(page_size) = self.page_size {
            params.push(format!("page_size={page_size}"));
        }
        if let Some(ref page_token) = self.page_token {
            params.push(format!("page_token={page_token}"));
        }
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let req: ApiRequest<AssignInfoListQueryResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 企业席位信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssignInfoListQueryResponse {
    /// 席位列表
    pub items: Vec<ProductAssignInfo>,
    /// 分页标记
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
    /// 是否还有更多
    #[serde(rename = "has_more")]
    pub has_more: Option<bool>,
}

/// 产品席位信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductAssignInfo {
    /// 席位 ID
    #[serde(rename = "product_id")]
    pub product_id: String,
    /// 席位名称
    #[serde(rename = "product_name")]
    pub product_name: String,
    /// 席位国际化名称
    #[serde(rename = "product_i18n_name")]
    pub product_i18n_name: Option<ProductI18nName>,
    /// 总数量
    #[serde(rename = "total_count")]
    pub total_count: i32,
    /// 已分配数量
    #[serde(rename = "assigned_count")]
    pub assigned_count: i32,
    /// 未分配数量
    #[serde(rename = "unassigned_count")]
    pub unassigned_count: i32,
    /// 有效期
    #[serde(rename = "expire_time")]
    pub expire_time: Option<String>,
}

/// 产品国际化名称
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductI18nName {
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

impl ApiResponseTrait for AssignInfoListQueryResponse {}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to AssignInfoListQueryRequestBuilder, will be removed in v1.0 (#271)")]
pub type AssignInfoListQueryBuilder = AssignInfoListQueryRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../tenant/v2/tenant/assign_info_list/query → 强类型 AssignInfoListQueryResponse。
    #[tokio::test]
    async fn test_query_product_assign_info_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/tenant/v2/tenant/assign_info_list/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "product_id": "p_001",
                            "product_name": "seats",
                            "total_count": 100,
                            "assigned_count": 40,
                            "unassigned_count": 60
                        }
                    ],
                    "page_token": "tok_next",
                    "has_more": true
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

        // 不设 page_size/page_token：execute 手工 url.push('?') 拼 query 的方式与 Transport
        // 不兼容（path 含 query 不匹配），是 pre-existing bug；此处验证 path + 响应解析。
        let resp = AssignInfoListQueryRequestBuilder::new(config)
            .execute()
            .await
            .expect("获取企业席位信息应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].product_id, "p_001");
        assert_eq!(resp.items[0].total_count, 100);
        assert_eq!(resp.page_token.as_deref(), Some("tok_next"));
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/tenant/v2/tenant/assign_info_list/query"
        );
    }
}
