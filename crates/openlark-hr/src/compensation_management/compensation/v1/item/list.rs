//! 批量查询薪资项
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/item/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

use crate::common::shared_models::I18nContent;

/// 批量查询薪资项请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 分页大小（可选）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置分页大小（可选）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
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
        let api_endpoint = CompensationApiV1::ItemList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询薪资项响应数据为空",
        )
        .await
    }
}

/// 批量查询薪资项响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 薪资项列表
    pub items: Vec<CompensationItem>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 薪资项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompensationItem {
    /// 薪资项 ID
    pub id: String,
    /// 薪资项名称
    pub name: String,
    /// 薪资项描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 薪资项分类 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 薪资项数值类型：money-金额 number-数值 percent-百分比
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
    /// 发放频率：year/half_year/quarterly/bimonthly/month/biweekly/week/day/hour
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_off_frequency_type: Option<String>,
    /// 小数位数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<i32>,
    /// 启用状态：1-启用 0-禁用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    /// 多语言名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<Vec<I18nContent>>,
    /// 多语言描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_descriptions: Option<Vec<I18nContent>>,
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
    async fn test_list_items_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.items[]（含 value_type/i18n 等）+ 分页
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"items": [{"id": "7196951947268589113", "name": "基本月薪", "description": "每月份的薪酬", "category_id": "7196951947268589223", "value_type": "money", "pay_off_frequency_type": "month", "decimal_places": 2, "active_status": 1, "i18n_names": [{"locale": "zh_cn", "value": "中文名称"}], "i18n_descriptions": [{"locale": "zh_cn", "value": "中文名称"}]}], "page_token": "1234452132", "has_more": true}"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/items"))
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
            .expect("查询薪资项应成功");

        assert!(data.has_more);
        assert_eq!(data.items.len(), 1);
        assert_eq!(data.items[0].id, "7196951947268589113");
        assert_eq!(data.items[0].name, "基本月薪");
        assert_eq!(data.items[0].value_type.as_deref(), Some("money"));
        assert_eq!(
            data.items[0].pay_off_frequency_type.as_deref(),
            Some("month")
        );
        assert_eq!(data.items[0].decimal_places, Some(2));
        assert_eq!(data.items[0].active_status, Some(1));
        assert_eq!(
            data.items[0].i18n_names.as_ref().unwrap()[0]
                .locale
                .as_deref(),
            Some("zh_cn")
        );
        assert_eq!(
            data.items[0].i18n_descriptions.as_ref().unwrap()[0]
                .value
                .as_deref(),
            Some("中文名称")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/compensation/v1/items");
    }
}
