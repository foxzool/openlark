//! 批量获取薪资项分类信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/item_category/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

use crate::common::shared_models::I18nContent;

/// 批量获取薪资项分类信息请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::CompensationApiV1;

        // 1. 构建端点
        let api_endpoint = CompensationApiV1::ItemCategoryList;
        let request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 2. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量获取薪资项分类信息响应数据为空",
        )
        .await
    }
}

/// 批量获取薪资项分类信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 薪资项分类列表
    pub items: Vec<ItemCategory>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时返回，否则不返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 薪资项分类
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemCategory {
    /// 薪资项分类 ID
    pub id: String,
    /// 薪资项分类名称
    pub name: String,
    /// 薪资项多语言分类
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<Vec<I18nContent>>,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_list_item_categories_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.items[]（含 i18n_names）+ 分页
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"items": [{"id": "4532312334", "name": "基本薪资类", "i18n_names": [{"locale": "zh_cn", "value": "中文名称"}]}], "page_token": "2314123434", "has_more": true}"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/item_categories"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = ListRequest::new(config)
            .execute()
            .await
            .expect("查询薪资项类别应成功");

        assert_eq!(data.items.len(), 1);
        assert_eq!(data.items[0].id, "4532312334");
        assert_eq!(data.items[0].name, "基本薪资类");
        assert_eq!(
            data.items[0].i18n_names.as_ref().unwrap()[0]
                .locale
                .as_deref(),
            Some("zh_cn")
        );
        assert_eq!(
            data.items[0].i18n_names.as_ref().unwrap()[0]
                .value
                .as_deref(),
            Some("中文名称")
        );
        assert!(data.has_more);
        assert_eq!(data.page_token.as_deref(), Some("2314123434"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/compensation/v1/item_categories"
        );
    }
}
