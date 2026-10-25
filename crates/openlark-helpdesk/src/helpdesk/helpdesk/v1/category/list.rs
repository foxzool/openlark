//! 获取知识库分类列表
//!
//! 获取服务台知识库所有分类。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/category/list-categories>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;

/// 获取知识库分类列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCategoryResponse {
    /// 分类列表（官方响应键为 categories）
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CategoryItem>>,
}

impl ApiResponseTrait for ListCategoryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 知识库分类项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryItem {
    /// 知识库分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 知识库分类ID（旧版，请使用 category_id）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 分类名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 父知识库分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 服务台ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub helpdesk_id: Option<String>,
    /// 语言
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 子分类详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<CategoryItem>>,
}

/// 获取知识库分类列表请求
#[derive(Debug, Clone)]
pub struct ListCategoryRequest {
    config: Arc<Config>,
    /// 知识库分类语言（查询参数，如 zh_cn）
    lang: Option<String>,
    /// 排序键（查询参数。1: 根据知识库分类更新时间排序）
    order_by: Option<i32>,
    /// 顺序（查询参数。true: 正序；false: 反序）
    asc: Option<bool>,
}

impl ListCategoryRequest {
    /// 创建新的获取知识库分类列表请求
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            lang: None,
            order_by: None,
            asc: None,
        }
    }

    /// 设置知识库分类语言
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = Some(lang.into());
        self
    }

    /// 设置排序键（1: 根据知识库分类更新时间排序）
    pub fn order_by(mut self, order_by: i32) -> Self {
        self.order_by = Some(order_by);
        self
    }

    /// 设置顺序（true: 正序；false: 反序）
    pub fn asc(mut self, asc: bool) -> Self {
        self.asc = Some(asc);
        self
    }

    /// 执行获取知识库分类列表请求
    pub async fn execute(self) -> SDKResult<ListCategoryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListCategoryResponse> {
        let api_endpoint = HelpdeskApiV1::CategoryList;
        let request = ApiRequest::<ListCategoryResponse>::get(api_endpoint.to_url())
            .query_opt("lang", self.lang.as_ref())
            .query_opt("order_by", self.order_by.map(|v| v.to_string()))
            .query_opt("asc", self.asc.map(|v| v.to_string()));

        Transport::request_typed(request, &self.config, Some(option), "获取知识库分类列表").await
    }
}

/// 获取知识库分类列表请求构建器
#[derive(Debug, Clone)]
pub struct ListCategoryRequestBuilder {
    config: Arc<Config>,
    lang: Option<String>,
    order_by: Option<i32>,
    asc: Option<bool>,
}

impl ListCategoryRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            lang: None,
            order_by: None,
            asc: None,
        }
    }

    /// 设置知识库分类语言
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = Some(lang.into());
        self
    }

    /// 设置排序键（1: 根据知识库分类更新时间排序）
    pub fn order_by(mut self, order_by: i32) -> Self {
        self.order_by = Some(order_by);
        self
    }

    /// 设置顺序（true: 正序；false: 反序）
    pub fn asc(mut self, asc: bool) -> Self {
        self.asc = Some(asc);
        self
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<ListCategoryResponse> {
        ListCategoryRequest {
            config: self.config.clone(),
            lang: self.lang.clone(),
            order_by: self.order_by,
            asc: self.asc,
        }
        .execute()
        .await
    }
}

/// 执行获取知识库分类列表（无过滤参数的默认语义）
pub async fn list_categories(config: &Config) -> SDKResult<ListCategoryResponse> {
    ListCategoryRequest::new(Arc::new(config.clone()))
        .execute()
        .await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let _builder = ListCategoryRequestBuilder::new(Arc::new(config));
    }

    /// 端到端：GET .../categories + 查询参数 → 强类型 ListCategoryResponse 解析
    /// （官方 data 内层键为 categories，含递归 children）。
    #[tokio::test]
    async fn test_list_category_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/helpdesk/v1/categories"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "categories": [
                        {
                            "category_id": "cat_001",
                            "id": "cat_001",
                            "name": "公告分类",
                            "parent_id": "0",
                            "helpdesk_id": "hd_001",
                            "language": "zh_cn",
                            "children": [
                                { "id": "cat_002", "name": "Hardware", "parent_id": "cat_001" }
                            ]
                        }
                    ]
                }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = ListCategoryRequest::new(config)
            .lang("zh_cn")
            .order_by(1)
            .asc(true)
            .execute()
            .await
            .expect("获取分类列表应成功");
        let items = resp.items.expect("分类列表应存在");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].category_id.as_deref(), Some("cat_001"));
        assert_eq!(
            items[0].children.as_ref().expect("children 应存在").len(),
            1
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/helpdesk/v1/categories");
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("lang=zh_cn"));
        assert!(query.contains("order_by=1"));
        assert!(query.contains("asc=true"));
    }
}
